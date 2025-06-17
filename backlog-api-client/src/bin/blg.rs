use backlog_api_client::{
    AddCommentParamsBuilder, AttachmentId, GetIssueListParamsBuilder, IssueIdOrKey, ProjectId,
    ProjectIdOrKey, PullRequestAttachmentId, PullRequestNumber, RepositoryIdOrName, StatusId,
    UserId, client::BacklogApiClient,
};
use backlog_project::requests::GetProjectParams;
use clap::{Args, Parser};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;

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
    /// Manage space
    Space(SpaceArgs),
    /// Manage projects
    Project(ProjectArgs),
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
    /// Add a comment to an issue
    #[command(about = "Add a comment to an issue")]
    AddComment(AddCommentArgs),
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

#[derive(Args, Debug)]
struct AddCommentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    issue_id_or_key: String,

    /// The comment content
    #[arg(short, long)]
    content: String,

    /// User IDs to notify (comma-separated, e.g., "123,456")
    #[arg(short, long)]
    notify_users: Option<String>,

    /// Attachment IDs to include (comma-separated, e.g., "789,101112")
    #[arg(short, long)]
    attachments: Option<String>,
}

#[derive(Parser)]
struct SpaceArgs {
    #[clap(subcommand)]
    command: SpaceCommands,
}

#[derive(Parser)]
enum SpaceCommands {
    /// Download space logo
    Logo {
        /// Output file path to save the logo
        #[clap(short, long, value_name = "FILE_PATH")]
        output: PathBuf,
    },
}

#[derive(Parser)]
struct ProjectArgs {
    #[clap(subcommand)]
    command: ProjectCommands,
}

#[derive(Parser)]
enum ProjectCommands {
    /// List all projects
    List,
    /// Show details of a specific project
    Show {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List statuses for a project
    StatusList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
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
                let pr_num = PullRequestNumber::from(pr_number);

                let pr = client
                    .git()
                    .get_pull_request(proj_id_or_key, repo_id_or_name, pr_num)
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
                let parsed_attachment_id = PullRequestAttachmentId::new(dl_args.attachment_id);

                let parsed_pr_number = PullRequestNumber::from(dl_args.pr_number);

                match client
                    .git()
                    .download_pull_request_attachment(
                        parsed_project_id,
                        parsed_repo_id,
                        parsed_pr_number,
                        parsed_attachment_id,
                    )
                    .await
                {
                    Ok(downloaded_file) => {
                        if let Err(e) = fs::write(&dl_args.output, &downloaded_file.bytes).await {
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
                    Ok(downloaded_file) => {
                        if let Err(e) = fs::write(&dl_args.output, &downloaded_file.bytes).await {
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
            IssueCommands::AddComment(add_args) => {
                println!(
                    "Adding comment to issue {}: {}",
                    add_args.issue_id_or_key, add_args.content
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&add_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            add_args.issue_id_or_key, e
                        )
                    })?;

                let mut builder = AddCommentParamsBuilder::default();
                builder.content(&add_args.content);

                // Parse notify_users if provided
                if let Some(notify_str) = &add_args.notify_users {
                    let user_ids: Result<Vec<UserId>, _> = notify_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(UserId::new))
                        .collect();
                    match user_ids {
                        Ok(ids) => builder.notified_user_id(ids),
                        Err(e) => {
                            eprintln!("Error parsing notify_users '{}': {}", notify_str, e);
                            return Ok(());
                        }
                    };
                }

                // Parse attachments if provided
                if let Some(attach_str) = &add_args.attachments {
                    let attachment_ids: Result<Vec<AttachmentId>, _> = attach_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(AttachmentId::new))
                        .collect();
                    match attachment_ids {
                        Ok(ids) => builder.attachment_id(ids),
                        Err(e) => {
                            eprintln!("Error parsing attachments '{}': {}", attach_str, e);
                            return Ok(());
                        }
                    };
                }

                let params = builder.build()?;

                match client
                    .issue()
                    .add_comment(parsed_issue_id_or_key, &params)
                    .await
                {
                    Ok(comment) => {
                        println!("Comment added successfully!");
                        println!("Comment ID: {}", comment.id);
                        println!("Created by: {}", comment.created_user.name);
                        println!("Created at: {}", comment.created);
                        if let Some(content) = &comment.content {
                            println!("Content: {}", content);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error adding comment: {}", e);
                    }
                }
            }
        },
        Commands::Space(space_args) => match space_args.command {
            SpaceCommands::Logo { output } => {
                println!("Downloading space logo to {}", output.display());

                match client.space().get_space_logo().await {
                    Ok(logo_bytes) => {
                        if let Err(e) = fs::write(&output, &logo_bytes).await {
                            eprintln!("Error writing logo to {}: {}", output.display(), e);
                        } else {
                            println!(
                                "Space logo downloaded successfully to: {}",
                                output.display()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error downloading space logo: {}", e);
                    }
                }
            }
        },
        Commands::Project(project_args) => match project_args.command {
            ProjectCommands::List => {
                println!("Listing all projects");

                let params = GetProjectParams {
                    archived: None,
                    all: true,
                };

                match client.project().get_project_list(params).await {
                    Ok(projects) => {
                        if projects.is_empty() {
                            println!("No projects found");
                        } else {
                            for project in projects {
                                println!(
                                    "[{}] {} (Key: {})",
                                    project.id, project.name, project.project_key
                                );
                                println!("  Chart Enabled: {}", project.chart_enabled);
                                println!("  Subtasking Enabled: {}", project.subtasking_enabled);
                                println!("  Archived: {}", project.archived);
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing projects: {}", e);
                    }
                }
            }
            ProjectCommands::Show { project_id_or_key } => {
                println!("Showing project: {}", project_id_or_key);

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                match client.project().get_project(proj_id_or_key).await {
                    Ok(project) => {
                        println!("Project ID: {}", project.id);
                        println!("Project Key: {}", project.project_key);
                        println!("Name: {}", project.name);
                        println!("Chart Enabled: {}", project.chart_enabled);
                        println!("Subtasking Enabled: {}", project.subtasking_enabled);
                        println!(
                            "Project Leader Can Edit Project Leader: {}",
                            project.project_leader_can_edit_project_leader
                        );
                        println!("Use Wiki: {}", project.use_wiki);
                        println!("Use File Sharing: {}", project.use_file_sharing);
                        println!("Use Wiki Tree View: {}", project.use_wiki_tree_view);
                        println!(
                            "Use Original Image Size at Wiki: {}",
                            project.use_original_image_size_at_wiki
                        );
                        println!("Text Formatting Rule: {:?}", project.text_formatting_rule);
                        println!("Archived: {}", project.archived);
                        println!("Display Order: {}", project.display_order);
                        println!("Use Dev Attributes: {}", project.use_dev_attributes);
                    }
                    Err(e) => {
                        eprintln!("Error getting project: {}", e);
                    }
                }
            }
            ProjectCommands::StatusList { project_id_or_key } => {
                println!("Listing statuses for project: {}", project_id_or_key);

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                match client.project().get_status_list(proj_id_or_key).await {
                    Ok(statuses) => {
                        if statuses.is_empty() {
                            println!("No statuses found");
                        } else {
                            for status in statuses {
                                println!(
                                    "[{}] {} (Color: {})",
                                    status.id, status.name, status.color
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing statuses: {}", e);
                    }
                }
            }
        },
    }

    Ok(())
}
