use dotenv;

mod github;
mod jira;
use github::github_update;
use jira::jira_update;

mod structs;
use structs::config::Config;

mod traits;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::from_env().expect("Error loading config!");
    jira_update(&config).await;
    github_update(&config).await;
}
