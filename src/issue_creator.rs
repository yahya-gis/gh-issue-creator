use log::{error, info};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_json::json;

use crate::config::Config;

pub struct IssueCreator {
    config: Config,
}

impl IssueCreator {
    pub fn new(config: Config) -> IssueCreator {
        IssueCreator { config }
    }

    pub async fn create_issue_and_add_to_project(
        &self,
        repo_id: &str,
        project_id: &str,
        title: &str,
        body: &str,
    ) -> Result<(), String> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.config.gh_access_token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("your-app-name"));

        // First, create the issue
        let create_issue_mutation = json!({
            "query": format!(r#"
                mutation createIssue {{
                    createIssue(input: {{repositoryId: "{}", title: "{}", body: "{}"}}) {{
                        issue {{
                            id
                            title
                        }}
                    }}
                }}
            "#, repo_id, title, body)
        });

        let res = client
            .post(&self.config.gh_graphql_api_url)
            .headers(headers.clone())
            .json(&create_issue_mutation)
            .send()
            .await;

        match res {
            Ok(response) => {
                // Parse the response
                let res_text = response.text().await.unwrap();
                let json: serde_json::Value = serde_json::from_str(&res_text).unwrap();
                if let Some(issue_id) = json["data"]["createIssue"]["issue"]["id"].as_str() {
                    info!("✅ Issue {} created successfully!", title);

                    // Then, add the issue to the project
                    let add_to_project_mutation = json!({
                        "query": format!(r#"
                            mutation {{
                                addProjectV2ItemById(input: {{projectId: "{}", contentId: "{}"}}) {{
                                    clientMutationId
                                    item {{
                                        id
                                    }}
                                }}
                            }}
                        "#, project_id, issue_id)
                    });

                    let res = client
                        .post(&self.config.gh_graphql_api_url)
                        .headers(headers)
                        .json(&add_to_project_mutation)
                        .send()
                        .await;

                    match res {
                        Ok(_) => {
                            info!("🚀 Issue {} added to the project.", title);
                            Ok(())
                        }
                        Err(e) => {
                            error!("Failed to add issue {} to project.", title);
                            Err(format!(
                                "Failed to add issue {} to project. Error: {}",
                                title, e
                            ))
                        }
                    }
                } else {
                    error!("Failed to create issue.");
                    Err(format!("Failed to create issue. Response: {}", res_text))
                }
            }
            Err(e) => {
                error!("❌ ERROR: Failed to send request to GitHub GraphQL API.");
                Err(format!(
                    "Failed to send request to GitHub GraphQL API. Error: {}",
                    e
                ))
            }
        }
    }
}
