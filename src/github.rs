use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

use log::{error, info};
use serde_json::json;

use crate::config::Config;

pub struct GitHub {
    config: Config,
}

impl GitHub {
    pub fn new(config: Config) -> GitHub {
        GitHub { config }
    }

    pub async fn get_repo_and_project_id(&self) -> Result<(String, String), String> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.config.gh_access_token)).unwrap(),
        );

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        headers.insert(USER_AGENT, HeaderValue::from_static("my-rust-app"));

        let query = json!({
            "query": format!("query {{ repository(owner: \"{}\", name: \"{}\") {{ id projectV2(number: {}) {{ id }} }} }}", self.config.repository_owner, self.config.repository_name, self.config.project_number)
        });

        let res = client
            .post(&self.config.gh_graphql_api_url)
            .headers(headers)
            .json(&query)
            .send()
            .await;

        match res {
            Ok(response) => {
                // Parse the response
                let res_text = response.text().await.unwrap();
                let json: serde_json::Value = serde_json::from_str(&res_text).unwrap();
                match (
                    json["data"]["repository"]["id"].as_str(),
                    json["data"]["repository"]["projectV2"]["id"].as_str(),
                ) {
                    (Some(repo_id), Some(project_id)) => {
                        info!("Successfully fetched repository ID and project ID.");
                        Ok((repo_id.to_string(), project_id.to_string()))
                    }
                    _ => {
                        error!("❌ ERROR: Failed to fetch repository ID and/or project ID.");
                        Err(format!(
                            "❌ ERROR: Failed to fetch repository ID and/or project ID. Response: {}",
                            res_text
                        ))
                    }
                }
            }
            Err(e) => {
                error!("❌ ERROR: Failed to send request to GitHub GraphQL API.");
                Err(format!(
                    "❌ ERROR: Failed to send request to GitHub GraphQL API. Error: {}",
                    e
                ))
            }
        }
    }
}
