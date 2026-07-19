use std::collections::HashSet;

use chrono::{DateTime, Local, NaiveDate};
use reqwest::Client;

use crate::structs::config::Config;
use crate::structs::jira::{Comment, CommentsResponse, Issue, Myself, SearchResponse};

const JIRA_V3_API_ENDPOINT: &str = "/rest/api/3";
const SEARCH_RESOURCE: &str = "/search/jql";
const MYSELF_RESOURCE: &str = "/myself";

pub async fn jira_update(config: &Config) {
    let client = Client::new();
    let base_url = format!("{}{JIRA_V3_API_ENDPOINT}", config.jira_base_url);

    let myself: Myself = get_json(
        &client,
        config,
        &format!("{base_url}{MYSELF_RESOURCE}"),
        &[],
    )
    .await;

    let mut seen_keys: HashSet<String> = HashSet::new();
    let mut activity: Vec<(Issue, &'static str)> = Vec::new();

    let owned_jql = "(assignee = currentUser() OR reporter = currentUser()) \
         AND updated >= startOfDay() ORDER BY updated DESC"
        .into();
    for issue in search_issues(&client, config, &base_url, owned_jql).await {
        if seen_keys.insert(issue.key.clone()) {
            activity.push((issue, "assignee/reporter"));
        }
    }

    let candidate_jql = format!(
        "project IN ({}) AND updated >= startOfDay() ORDER BY updated DESC",
        config.jira_project_keys.join(",")
    );
    let today = Local::now().date_naive();
    for issue in search_issues(&client, config, &base_url, candidate_jql).await {
        if seen_keys.contains(&issue.key) {
            continue;
        }
        if issue_has_my_comment_today(
            &client,
            config,
            &base_url,
            &issue.key,
            &myself.account_id,
            today,
        )
        .await
        {
            seen_keys.insert(issue.key.clone());
            activity.push((issue, "commented"));
        }
    }

    println!("Today's Jira activity:");
    for (issue, reason) in &activity {
        println!("- {} [{reason}]: {}", issue.key, issue.fields.summary);
    }
}

async fn search_issues(
    client: &Client,
    config: &Config,
    base_url: &str,
    jql: String,
) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut next_page_token: Option<String> = None;

    loop {
        let mut query = vec![("jql", jql.clone()), ("fields", "summary".to_string())];
        if let Some(token) = next_page_token.take() {
            query.push(("nextPageToken", token));
        }

        let response: SearchResponse = get_json(
            client,
            config,
            &format!("{base_url}{SEARCH_RESOURCE}"),
            &query,
        )
        .await;

        next_page_token = response.next_page_token;
        issues.extend(response.issues);

        if next_page_token.is_none() {
            break;
        }
    }

    issues
}

async fn issue_has_my_comment_today(
    client: &Client,
    config: &Config,
    base_url: &str,
    issue_key: &str,
    my_account_id: &str,
    today: NaiveDate,
) -> bool {
    let response: CommentsResponse = get_json(
        client,
        config,
        &format!("{base_url}/issue/{issue_key}/comment"),
        &[],
    )
    .await;

    response
        .comments
        .iter()
        .any(|comment| comment_is_mine_today(comment, my_account_id, today))
}

fn comment_is_mine_today(comment: &Comment, my_account_id: &str, today: NaiveDate) -> bool {
    if comment.author.account_id != my_account_id {
        return false;
    }

    DateTime::parse_from_str(&comment.created, "%Y-%m-%dT%H:%M:%S%.3f%z")
        .map(|created| created.with_timezone(&Local).date_naive() == today)
        .unwrap_or(false)
}

async fn get_json<T: serde::de::DeserializeOwned>(
    client: &Client,
    config: &Config,
    url: &str,
    query: &[(&str, String)],
) -> T {
    client
        .get(url)
        .query(query)
        .basic_auth(&config.jira_email, Some(&config.jira_token))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
