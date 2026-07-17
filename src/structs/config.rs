use anyhow::{Context, Result};

pub struct Config {
    pub jira_base_url: String,
    pub jira_email: String,
    pub jira_token: String,
    pub jira_project_keys: Vec<String>,
    pub github_token: String,
    pub github_username: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            jira_base_url: env_var("JIRA_BASE_URL")?,
            jira_email: env_var("JIRA_EMAIL")?,
            jira_token: env_var("JIRA_TOKEN")?,
            jira_project_keys: parse_project_keys(&env_var("JIRA_PROJECT_KEYS")?),
            github_token: env_var("GITHUB_TOKEN")?,
            github_username: env_var("GITHUB_USERNAME")?,
        })
    }
}

fn env_var(var: &str) -> Result<String> {
    std::env::var(var).context(format!("{var} not set"))
}

fn parse_project_keys(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|key| key.trim().to_string())
        .filter(|key| !key.is_empty())
        .collect()
}
