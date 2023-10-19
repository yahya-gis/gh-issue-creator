use dotenv::dotenv;
use env_logger::Env;
use log::info;

mod config;
mod description;
mod file_handler;
mod github;
mod issue_creator;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv().ok();

    let config = config::Config::new().unwrap();
    let github = github::GitHub::new(config.clone());
    let issue_creator = issue_creator::IssueCreator::new(config.clone());

    match github.get_repo_and_project_id().await {
        Ok((repo_id, project_id)) => {
            info!("🔑 Repository ID: {}", repo_id);
            info!("🔑 Project ID: {}", project_id);

            // Read tasks and generate description text
            match file_handler::read_and_generate_description(&config.json_file_path) {
                Ok(tasks) => {
                    for task in tasks {
                        // Create issue and add to project
                        match issue_creator
                            .create_issue_and_add_to_project(
                                &repo_id,
                                &project_id,
                                &task.0,
                                &task.1,
                            )
                            .await
                        {
                            Ok(_) => info!("🎉 Successfully created issue and added to project."),
                            Err(e) => eprintln!("❌ ERROR: Error creating issue and adding to project: {}", e),
                        }
                    }
                    info!("💯 All tasks have been processed successfully.");
                }
                Err(e) => eprintln!("❌ ERROR: Error reading tasks from file: {}", e),
            }
        }
        Err(e) => eprintln!("❌ ERROR: Error: {}", e),
    }
}
