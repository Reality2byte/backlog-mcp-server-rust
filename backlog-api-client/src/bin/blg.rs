use backlog_api_client::client::BacklogApiClient;
use backlog_core::project_id_or_key::ProjectIdOrKey; // Ensure this is the correct import
use clap::Parser;
use std::env;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Manage repositories
    Repo(RepoArgs),
    /// Manage pull requests
    Pr(PrArgs),
}

#[derive(Parser)]
struct RepoArgs {
    #[clap(subcommand)]
    command: RepoCommands,
}

#[derive(Parser)]
enum RepoCommands {
    /// List repositories in a project
    List {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
    },
    /// Show details of a specific repository
    Show {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
    },
}

#[derive(Parser)]
struct PrArgs {
    #[clap(subcommand)]
    command: PrCommands,
}

#[derive(Parser)]
enum PrCommands {
    /// List pull requests in a repository
    List {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
    },
    /// Show details of a specific pull request
    Show {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull Request number
        #[clap(short, long)]
        pr_number: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url =
        env::var("BACKLOG_BASE_URL").expect("BACKLOG_BASE_URL environment variable is required");
    let api_key =
        env::var("BACKLOG_API_KEY").expect("BACKLOG_API_KEY environment variable is required");

    // Ensure client is built with the 'git' feature for git() method to be available.
    // This binary should be compiled with `cargo run --bin blg --features git,cli`
    let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);

    let cli = Cli::parse();

    match cli.command {
        Commands::Repo(repo_args) => match repo_args.command {
            RepoCommands::List { project_id } => {
                println!("Listing repositories for project: {}", project_id);
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                // Assumes backlog_git is enabled via features for the client build
                let repos = client.git().list_repositories(&proj_id_or_key).await?;
                // TODO: Pretty print repositories
                println!("{:?}", repos);
            }
            RepoCommands::Show { project_id, repo_id } => {
                println!(
                    "Showing repository {} in project: {}",
                    repo_id, project_id
                );
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                let repo = client
                    .git()
                    .get_repository(&proj_id_or_key, &repo_id)
                    .await?;
                // TODO: Pretty print repository
                println!("{:?}", repo);
            }
        },
        Commands::Pr(pr_args) => match pr_args.command {
            PrCommands::List { project_id, repo_id } => {
                println!(
                    "Listing pull requests for repo {} in project: {}",
                    repo_id, project_id
                );
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                let prs = client
                    .git()
                    .list_pull_requests(&proj_id_or_key, &repo_id)
                    .await?;
                // TODO: Pretty print pull requests
                println!("{:?}", prs);
            }
            PrCommands::Show {
                project_id,
                repo_id,
                pr_number,
            } => {
                println!(
                    "Showing PR #{} for repo {} in project: {}",
                    pr_number, repo_id, project_id
                );
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                let pr = client
                    .git()
                    .get_pull_request(&proj_id_or_key, &repo_id, pr_number)
                    .await?;
                // TODO: Pretty print pull request
                println!("{:?}", pr);
            }
        },
    }

    Ok(())
}
