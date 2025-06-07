use backlog_api_client::{
    AttachmentId, // Corrected import
    GetIssueListParamsBuilder,
    IssueIdOrKey,
    ProjectId,
    ProjectIdOrKey,
    RepositoryIdOrName,
    StatusId,
    UserId,
    client::BacklogApiClient,
};
use clap::{Args, Parser}; // Removed Subcommand
use std::env;
use std::path::PathBuf;
use std::str::FromStr; // Added for parsing
use tokio::fs; // Added for file system operations

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
    /// Manage issues
    Issue(IssueArgs),
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
    /// Download a pull request attachment
    #[command(about = "Download a pull request attachment")]
    DownloadAttachment(DownloadPrAttachmentArgs),
}

#[derive(Args, Debug)]
struct DownloadPrAttachmentArgs {
    /// Project ID or Key
    #[clap(short = 'p', long)]
    project_id: String,
    /// Repository ID or Name
    #[clap(short = 'r', long)]
    repo_id: String,
    /// Pull Request number
    #[clap(short = 'n', long)]
    pr_number: u64,
    /// The numeric ID of the attachment to download
    #[clap(short = 'a', long)]
    attachment_id: u32,
    /// Output file path to save the attachment
    #[clap(short = 'o', long, value_name = "FILE_PATH")]
    output: PathBuf,
}

#[derive(Parser)]
struct IssueArgs {
    #[clap(subcommand)]
    command: IssueCommands,
}

#[derive(Parser)]
enum IssueCommands {
    /// List issues
    List {
        #[clap(flatten)]
        params: IssueListCliParams,
    },
    /// Show details of a specific issue
    Show {
        /// Issue ID or Key (e.g., "PROJECT-123" or "12345")
        #[clap(name = "ISSUE_ID_OR_KEY")]
        issue_id_or_key: String,
    },
    /// Download an issue attachment
    #[command(about = "Download an issue attachment")]
    DownloadAttachment(DownloadAttachmentArgs),
}

#[derive(Args, Debug)]
struct DownloadAttachmentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    issue_id_or_key: String,

    /// The numeric ID of the attachment to download
    attachment_id: u32,

    /// Output file path to save the attachment
    #[arg(short, long, value_name = "FILE_PATH")]
    output: PathBuf,
}

#[derive(Parser, Debug, Default)]
struct IssueListCliParams {
    /// Filter by project ID(s)
    #[clap(long)]
    project_id: Option<Vec<String>>,
    /// Filter by assignee ID(s)
    #[clap(long)]
    assignee_id: Option<Vec<String>>,
    /// Filter by status ID(s)
    #[clap(long)]
    status_id: Option<Vec<String>>,
    /// Keyword to search for in summary or description
    #[clap(long)]
    keyword: Option<String>,
    /// Number of issues to retrieve (1-100, default 20)
    #[clap(long, default_value_t = 20)]
    count: u32,
    // TODO: Add more filters like sort, order, offset, issue_type_id, etc.
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("BACKLOG_BASE_URL")?;
    let api_key = env::var("BACKLOG_API_KEY")?;

    let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);

    let cli = Cli::parse();
    match cli.command {
        Commands::Repo(repo_args) => match repo_args.command {
            RepoCommands::List { project_id } => {
                println!("Listing repositories for project: {}", project_id);
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                // Assumes backlog_git is enabled via features for the client build
                let repos = client.git().get_repository_list(proj_id_or_key).await?;
                // TODO: Pretty print repositories
                println!("{:?}", repos);
            }
            RepoCommands::Show {
                project_id,
                repo_id,
            } => {
                println!("Showing repository {} in project: {}", repo_id, project_id);
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                let repo_id_or_name = repo_id.parse::<RepositoryIdOrName>()?;
                let repo = client
                    .git()
                    .get_repository(proj_id_or_key, repo_id_or_name)
                    .await?;
                // TODO: Pretty print repository
                println!("{:?}", repo);
            }
        },
        Commands::Pr(pr_args) => match pr_args.command {
            PrCommands::List {
                project_id,
                repo_id,
            } => {
                println!(
                    "Listing pull requests for repo {} in project: {}",
                    repo_id, project_id
                );
                let proj_id_or_key = project_id.parse::<ProjectIdOrKey>()?;
                let repo_id_or_name = repo_id.parse::<RepositoryIdOrName>()?;
                let prs = client
                    .git()
                    .get_pull_request_list(proj_id_or_key, repo_id_or_name)
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
                let repo_id_or_name = repo_id.parse::<RepositoryIdOrName>()?;

                let pr = client
                    .git()
                    .get_pull_request(proj_id_or_key, repo_id_or_name, pr_number)
                    .await?;
                // TODO: Pretty print pull request
                println!("{:?}", pr);
            }
            PrCommands::DownloadAttachment(dl_args) => {
                println!(
                    "Downloading attachment {} for PR #{} in repo {} (project {}) to {}",
                    dl_args.attachment_id,
                    dl_args.pr_number,
                    dl_args.repo_id,
                    dl_args.project_id,
                    dl_args.output.display()
                );

                let parsed_project_id =
                    ProjectIdOrKey::from_str(&dl_args.project_id).map_err(|e| {
                        format!("Failed to parse project_id '{}': {}", dl_args.project_id, e)
                    })?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&dl_args.repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{}': {}", dl_args.repo_id, e))?;
                let parsed_attachment_id = AttachmentId::new(dl_args.attachment_id);

                match client
                    .git()
                    .download_pull_request_attachment(
                        parsed_project_id,
                        parsed_repo_id,
                        dl_args.pr_number,
                        parsed_attachment_id,
                    )
                    .await
                {
                    Ok(file_bytes) => {
                        if let Err(e) = fs::write(&dl_args.output, &file_bytes).await {
                            eprintln!(
                                "Error writing attachment to {}: {}",
                                dl_args.output.display(),
                                e
                            );
                        } else {
                            println!(
                                "Attachment downloaded successfully to: {}",
                                dl_args.output.display()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error downloading PR attachment: {}", e);
                    }
                }
            }
        },
        Commands::Issue(issue_args) => match issue_args.command {
            IssueCommands::Show { issue_id_or_key } => {
                println!("Showing issue: {}", issue_id_or_key);
                let parsed_issue_id_or_key = issue_id_or_key.parse::<IssueIdOrKey>()?;
                let issue = client.issue().get_issue(parsed_issue_id_or_key).await?;
                // TODO: Pretty print issue
                println!("{:?}", issue);
            }
            IssueCommands::List { params } => {
                println!("Listing issues with params: {:?}", params);
                let mut builder = GetIssueListParamsBuilder::default();

                if let Some(p_ids) = params.project_id {
                    let parsed_ids: std::result::Result<Vec<ProjectId>, _> = p_ids
                        .iter()
                        .map(|s| s.parse::<u32>().map(ProjectId::from))
                        .collect();
                    builder.project_id(parsed_ids?);
                }
                if let Some(a_ids) = params.assignee_id {
                    let parsed_ids: std::result::Result<Vec<UserId>, _> = a_ids
                        .iter()
                        .map(|s| s.parse::<u32>().map(UserId::from))
                        .collect();
                    builder.assignee_id(parsed_ids?);
                }
                if let Some(s_ids) = params.status_id {
                    let parsed_ids: std::result::Result<Vec<StatusId>, _> = s_ids
                        .iter()
                        .map(|s| s.parse::<u32>().map(StatusId::from))
                        .collect();
                    builder.status_id(parsed_ids?);
                }
                if let Some(keyword) = params.keyword {
                    builder.keyword(keyword);
                }
                builder.count(params.count); // count has a default_value_t

                let list_params = builder.build()?;
                let issues = client.issue().get_issue_list(list_params).await?;
                // TODO: Pretty print issues
                println!("{:?}", issues);
            }
            IssueCommands::DownloadAttachment(dl_args) => {
                println!(
                    "Downloading attachment {} for issue {} to {}",
                    dl_args.attachment_id,
                    dl_args.issue_id_or_key,
                    dl_args.output.display()
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&dl_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            dl_args.issue_id_or_key, e
                        )
                    })?;

                let parsed_attachment_id = AttachmentId::new(dl_args.attachment_id);

                match client
                    .issue()
                    .get_attachment_file(parsed_issue_id_or_key, parsed_attachment_id)
                    .await
                {
                    Ok(file_bytes) => {
                        if let Err(e) = fs::write(&dl_args.output, &file_bytes).await {
                            eprintln!(
                                "Error writing attachment to {}: {}",
                                dl_args.output.display(),
                                e
                            );
                        } else {
                            println!(
                                "Attachment downloaded successfully to: {}",
                                dl_args.output.display()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error downloading attachment: {}", e);
                    }
                }
            }
        },
    }

    Ok(())
}
