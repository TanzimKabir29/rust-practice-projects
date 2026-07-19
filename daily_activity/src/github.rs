use crate::structs::{config::Config, github::Event};
use chrono::{DateTime, Local};
use reqwest::Client;

use crate::traits::str_ext::StrExt;

const GITHUB_BASE_URL: &str = "https://api.github.com";
const EVENTS_ENDPOINT: &str = "/users/{}/events";
const USER_AGENT: &str = "daily_activity";
const GITHUB_API_VERSION: &str = "2026-03-10";
const FALLBACK_VALUE: &str = "!BAD-URL!";

pub async fn github_update(config: &Config) {
    let client = Client::new();
    let url = format!(
        "{}{}",
        GITHUB_BASE_URL,
        EVENTS_ENDPOINT.replace("{}", &config.github_username),
    );

    let mut events: Vec<Event> = get_json(&client, config, url.as_str()).await;

    let today = Local::now().date_naive();

    events.retain(|event| {
        let event_creation_date = match DateTime::parse_from_rfc3339(&event.created_at) {
            Ok(dt) => dt,
            Err(err) => {
                eprintln!("Error parsing {} into datetime: {}", &event.created_at, err);
                return false;
            }
        };
        if event_creation_date.with_timezone(&Local).date_naive() != today {
            return false;
        }
        true
    });

    display_events(events);
}

async fn get_json<T: serde::de::DeserializeOwned + std::fmt::Debug>(
    client: &Client,
    config: &Config,
    url: &str,
) -> T {
    client
        .get(url)
        .header("Authorization", format!("Bearer {}", config.github_token))
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .header("X-Github-Api-Version", GITHUB_API_VERSION)
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .inspect(|x| {
            println!("{:#?}", x);
        })
        .unwrap()
}

fn display_events(events: Vec<Event>) {
    println!("Today's github update:");
    events.iter().for_each(|event| println!("{:#?}", event));
    events
        .iter()
        .for_each(|event| match event.event_type.as_str() {
            "CommitCommentEvent" => println!(
                concat!("- Commented on a commit:\n", "  \"{}\"\n", "  Link: {}"),
                get(event, &["comment", "body"]),
                get(event, &["comment", "html_url"]),
            ),
            "CreateEvent" => println!(
                "- Created {}: {}",
                get(event, &["ref_type"]),
                get(event, &["full_ref"]),
            ),
            "DeleteEvent" => println!(
                "- Deleted {}: {}",
                get(event, &["ref_type"]),
                get(event, &["full_ref"]),
            ),
            "DiscussionEvent" => println!(
                "- Discussion created: {}",
                get(event, &["discussion", "url"]),
            ),
            "PullRequestEvent" => println!(
                "- {} a pull request:\n  Title: {}#{}\n  Link: {}",
                get(event, &["action"]).capitalize_first(),
                get(event, &["pull_request", "title"]),
                get(event, &["pull_request", "number"]),
                get(event, &["pull_request", "html_url"]),
            ),
            "PushEvent" => println!(
                "- Pushed a commit {}\n{}",
                get(event, &["head"]),
                get(event, &["ref"])
            ),
            _ => println!("And some other stuff"),
        });
}

fn get(event: &Event, keys: &[&str]) -> String {
    let mut current = &event.payload;

    for key in keys {
        current = current
            .get(key)
            .expect(format!("Missing key: {key}").as_str());
    }

    current
        .as_str()
        .unwrap_or_else(|| FALLBACK_VALUE)
        .to_string()
}
