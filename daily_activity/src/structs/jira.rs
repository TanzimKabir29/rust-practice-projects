use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Myself {
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub issues: Vec<Issue>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Issue {
    pub key: String,
    pub fields: IssueFields,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IssueFields {
    pub summary: String,
}

#[derive(Deserialize, Debug)]
pub struct CommentsResponse {
    pub comments: Vec<Comment>,
}

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub author: CommentAuthor,
    pub created: String,
}

#[derive(Deserialize, Debug)]
pub struct CommentAuthor {
    #[serde(rename = "accountId")]
    pub account_id: String,
}
