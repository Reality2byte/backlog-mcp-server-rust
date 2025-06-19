use backlog_api_client::{
    AddCommentParamsBuilder, AttachmentId, GetIssueListParamsBuilder, IssueIdOrKey, ProjectId,
    ProjectIdOrKey, PullRequestAttachmentId, PullRequestNumber, RepositoryIdOrName, StatusId,
    UserId, client::BacklogApiClient,
};
use backlog_core::identifier::CommentId;
#[cfg(any(feature = "issue_writable", feature = "project_writable"))]
use backlog_core::{
    IssueKey,
    identifier::{CategoryId, IssueTypeId, MilestoneId, PriorityId, ResolutionId},
};
#[cfg(feature = "project_writable")]
use backlog_domain_models::IssueTypeColor;
#[cfg(feature = "issue_writable")]
use backlog_issue::requests::{AddIssueParamsBuilder, UpdateIssueParamsBuilder};
use backlog_project::requests::GetProjectParams;
#[cfg(feature = "project_writable")]
use backlog_project::requests::{
    AddCategoryParams, AddIssueTypeParams, AddVersionParams, DeleteIssueTypeParams,
    UpdateCategoryParams, UpdateIssueTypeParams,
};
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
    /// Manage users
    User(UserArgs),
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
    /// Create a new issue
    #[command(about = "Create a new issue")]
    Create(CreateIssueArgs),
    /// Update an existing issue
    #[command(about = "Update an existing issue")]
    Update(UpdateIssueArgs),
    /// Delete an issue
    #[command(about = "Delete an issue")]
    Delete(DeleteIssueArgs),
    /// Count comments for an issue
    #[command(about = "Count comments for an issue")]
    CountComment(CountCommentArgs),
    /// Get a specific comment for an issue
    #[command(about = "Get a specific comment for an issue")]
    GetComment(GetCommentArgs),
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

#[derive(Args, Debug)]
struct CreateIssueArgs {
    /// Project ID or Key
    #[arg(short, long)]
    project_id: String,

    /// Issue summary (title)
    #[arg(short, long)]
    summary: String,

    /// Issue type ID
    #[arg(short = 't', long)]
    issue_type_id: u32,

    /// Priority ID
    #[arg(long)]
    priority_id: u32,

    /// Issue description
    #[arg(short, long)]
    description: Option<String>,

    /// Assignee user ID
    #[arg(short, long)]
    assignee_id: Option<u32>,

    /// Due date (YYYY-MM-DD format)
    #[arg(long)]
    due_date: Option<String>,

    /// Category IDs (comma-separated)
    #[arg(short, long)]
    category_ids: Option<String>,

    /// Milestone IDs (comma-separated)
    #[arg(short, long)]
    milestone_ids: Option<String>,
}

#[derive(Args, Debug)]
struct UpdateIssueArgs {
    /// Issue ID or Key
    issue_id_or_key: String,

    /// Issue summary (title)
    #[arg(short, long)]
    summary: Option<String>,

    /// Issue description
    #[arg(short, long)]
    description: Option<String>,

    /// Issue type ID
    #[arg(short = 't', long)]
    issue_type_id: Option<u32>,

    /// Priority ID
    #[arg(long)]
    priority_id: Option<u32>,

    /// Status ID
    #[arg(long)]
    status_id: Option<String>,

    /// Assignee user ID
    #[arg(short, long)]
    assignee_id: Option<u32>,

    /// Resolution ID
    #[arg(short, long)]
    resolution_id: Option<u32>,

    /// Due date (YYYY-MM-DD format)
    #[arg(long)]
    due_date: Option<String>,

    /// Comment to add with the update
    #[arg(short, long)]
    comment: Option<String>,
}

#[derive(Args, Debug)]
struct DeleteIssueArgs {
    /// Issue Key (e.g., "PROJECT-123")
    issue_key: String,
}

#[derive(Args, Debug)]
struct CountCommentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    issue_id_or_key: String,
}

#[derive(Args, Debug)]
struct GetCommentArgs {
    /// The ID or key of the issue (e.g., "PROJECT-123" or "12345")
    issue_id_or_key: String,
    /// The ID of the comment
    comment_id: u32,
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
    /// List milestones for a project
    MilestoneList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List issue types for a project
    IssueTypeList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// List categories for a project
    CategoryList {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
    },
    /// Add a category to a project
    CategoryAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Category name
        #[clap(short, long)]
        name: String,
    },
    /// Update a category in a project
    CategoryUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Category ID
        #[clap(short, long)]
        category_id: u32,
        /// New category name
        #[clap(short, long)]
        name: String,
    },
    /// Delete a category from a project
    CategoryDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Category ID
        #[clap(short, long)]
        category_id: u32,
    },
    /// Add an issue type to a project
    IssueTypeAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Issue type name
        #[clap(short, long)]
        name: String,
        /// Issue type color (available: red, dark-red, purple, violet, blue, teal, green, orange, pink, gray)
        #[clap(short, long)]
        color: String,
        /// Template summary (optional)
        #[clap(short = 's', long)]
        template_summary: Option<String>,
        /// Template description (optional)
        #[clap(short = 'd', long)]
        template_description: Option<String>,
    },
    /// Delete an issue type from a project
    IssueTypeDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Issue type ID to delete
        #[clap(long)]
        issue_type_id: u32,
        /// Substitute issue type ID for existing issues
        #[clap(long)]
        substitute_issue_type_id: u32,
    },
    /// Update an issue type in a project
    IssueTypeUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Issue type ID to update
        #[clap(long)]
        issue_type_id: u32,
        /// New issue type name (optional)
        #[clap(short, long)]
        name: Option<String>,
        /// New issue type color (optional: red, dark-red, purple, violet, blue, teal, green, orange, pink, gray)
        #[clap(short, long)]
        color: Option<String>,
        /// New template summary (optional)
        #[clap(short = 's', long)]
        template_summary: Option<String>,
        /// New template description (optional)
        #[clap(short = 'd', long)]
        template_description: Option<String>,
    },
    /// List priorities (space-wide)
    PriorityList,
    /// List resolutions (space-wide)
    ResolutionList,
    /// Add a version/milestone to a project
    #[cfg(feature = "project_writable")]
    VersionAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Version name
        #[clap(short, long)]
        name: String,
        /// Version description
        #[clap(short, long)]
        description: Option<String>,
        /// Start date (YYYY-MM-DD)
        #[clap(long)]
        start_date: Option<String>,
        /// Release due date (YYYY-MM-DD)
        #[clap(long)]
        release_due_date: Option<String>,
    },
    /// Download project icon
    Icon {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Output file path to save the icon
        #[clap(short, long, value_name = "FILE_PATH")]
        output: PathBuf,
    },
}

#[derive(Parser)]
struct UserArgs {
    #[clap(subcommand)]
    command: UserCommands,
}

#[derive(Parser)]
enum UserCommands {
    /// List all users
    List,
    /// Get current user info
    Me,
    /// Download user icon
    Icon {
        /// User ID
        #[clap(name = "USER_ID")]
        user_id: u32,
        /// Output file path to save the icon
        #[clap(short, long, value_name = "FILE_PATH")]
        output: PathBuf,
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
            #[cfg(feature = "issue_writable")]
            IssueCommands::Create(create_args) => {
                println!("Creating new issue...");

                let project_id_or_key = create_args.project_id.parse::<ProjectIdOrKey>()?;
                let project_id = match project_id_or_key {
                    ProjectIdOrKey::Id(id) => id,
                    ProjectIdOrKey::Key(_) => {
                        eprintln!(
                            "Error: Project key not supported for issue creation. Please use project ID."
                        );
                        return Ok(());
                    }
                    ProjectIdOrKey::EitherIdOrKey(id, _) => id,
                };

                let mut builder = AddIssueParamsBuilder::default();
                builder
                    .project_id(project_id)
                    .summary(&create_args.summary)
                    .issue_type_id(IssueTypeId::new(create_args.issue_type_id))
                    .priority_id(PriorityId::new(create_args.priority_id));

                if let Some(description) = &create_args.description {
                    builder.description(description);
                }

                if let Some(assignee_id) = create_args.assignee_id {
                    builder.assignee_id(UserId::new(assignee_id));
                }

                if let Some(_due_date) = &create_args.due_date {
                    // Due date parsing would need proper DateTime conversion
                    // For now, skip this implementation detail
                }

                if let Some(category_str) = &create_args.category_ids {
                    let category_ids: Result<Vec<CategoryId>, _> = category_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(CategoryId::new))
                        .collect();
                    if let Ok(ids) = category_ids {
                        builder.category_id(ids);
                    }
                }

                if let Some(milestone_str) = &create_args.milestone_ids {
                    let milestone_ids: Result<Vec<MilestoneId>, _> = milestone_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(MilestoneId::new))
                        .collect();
                    if let Ok(ids) = milestone_ids {
                        builder.milestone_id(ids);
                    }
                }

                let params = builder.build()?;

                match client.issue().add_issue(params).await {
                    Ok(issue) => {
                        println!("Issue created successfully!");
                        println!("Issue Key: {}", issue.issue_key);
                        println!("Issue ID: {}", issue.id);
                        println!("Summary: {}", issue.summary);
                        println!("Status: {}", issue.status.name);
                    }
                    Err(e) => {
                        eprintln!("Error creating issue: {}", e);
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::Update(update_args) => {
                println!("Updating issue: {}", update_args.issue_id_or_key);

                let issue_id_or_key = update_args.issue_id_or_key.parse::<IssueIdOrKey>()?;

                let mut builder = UpdateIssueParamsBuilder::default();

                if let Some(summary) = &update_args.summary {
                    builder.summary(summary);
                }

                if let Some(description) = &update_args.description {
                    builder.description(description);
                }

                if let Some(issue_type_id) = update_args.issue_type_id {
                    builder.issue_type_id(IssueTypeId::new(issue_type_id));
                }

                if let Some(priority_id) = update_args.priority_id {
                    builder.priority_id(PriorityId::new(priority_id));
                }

                if let Some(status_id) = &update_args.status_id {
                    builder.status_id(status_id);
                }

                if let Some(assignee_id) = update_args.assignee_id {
                    builder.assignee_id(UserId::new(assignee_id));
                }

                if let Some(resolution_id) = update_args.resolution_id {
                    builder.resolution_id(ResolutionId::new(resolution_id));
                }

                if let Some(comment) = &update_args.comment {
                    builder.comment(comment);
                }

                let params = builder.build()?;

                match client.issue().update_issue(issue_id_or_key, &params).await {
                    Ok(issue) => {
                        println!("Issue updated successfully!");
                        println!("Issue Key: {}", issue.issue_key);
                        println!("Summary: {}", issue.summary);
                        println!("Status: {}", issue.status.name);
                    }
                    Err(e) => {
                        eprintln!("Error updating issue: {}", e);
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::Delete(delete_args) => {
                println!("Deleting issue: {}", delete_args.issue_key);

                let issue_key = delete_args.issue_key.parse::<IssueKey>()?;

                match client.issue().delete_issue(issue_key).await {
                    Ok(issue) => {
                        println!("Issue deleted successfully!");
                        println!("Deleted Issue Key: {}", issue.issue_key);
                        println!("Summary: {}", issue.summary);
                    }
                    Err(e) => {
                        eprintln!("Error deleting issue: {}", e);
                    }
                }
            }
            IssueCommands::CountComment(count_args) => {
                println!(
                    "Counting comments for issue: {}",
                    count_args.issue_id_or_key
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&count_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            count_args.issue_id_or_key, e
                        )
                    })?;

                match client.issue().count_comment(parsed_issue_id_or_key).await {
                    Ok(response) => {
                        println!(
                            "Comment count for issue {}: {}",
                            count_args.issue_id_or_key, response.count
                        );
                    }
                    Err(e) => {
                        eprintln!("Error counting comments: {}", e);
                    }
                }
            }
            IssueCommands::GetComment(get_args) => {
                println!(
                    "Getting comment {} for issue: {}",
                    get_args.comment_id, get_args.issue_id_or_key
                );

                let parsed_issue_id_or_key = IssueIdOrKey::from_str(&get_args.issue_id_or_key)
                    .map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            get_args.issue_id_or_key, e
                        )
                    })?;

                let comment_id = CommentId::new(get_args.comment_id);

                match client
                    .issue()
                    .get_comment(parsed_issue_id_or_key, comment_id)
                    .await
                {
                    Ok(comment) => {
                        println!("Comment ID: {}", comment.id);
                        println!("Created by: {}", comment.created_user.name);
                        println!("Created at: {}", comment.created);
                        println!("Updated at: {}", comment.updated);
                        if let Some(content) = &comment.content {
                            println!("Content: {}", content);
                        } else {
                            println!("Content: (empty)");
                        }
                        if !comment.change_log.is_empty() {
                            println!("Change log entries: {}", comment.change_log.len());
                        }
                        if !comment.notifications.is_empty() {
                            println!("Notifications: {}", comment.notifications.len());
                        }
                        if !comment.stars.is_empty() {
                            println!("Stars: {}", comment.stars.len());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting comment: {}", e);
                    }
                }
            }
            #[cfg(not(feature = "issue_writable"))]
            IssueCommands::Create(_) | IssueCommands::Update(_) | IssueCommands::Delete(_) => {
                eprintln!(
                    "Issue creation, update, and deletion are not available. Please build with 'issue_writable' feature."
                );
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
            ProjectCommands::MilestoneList { project_id_or_key } => {
                println!("Listing milestones for project: {}", project_id_or_key);

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                match client
                    .project()
                    .get_version_milestone_list(proj_id_or_key)
                    .await
                {
                    Ok(milestones) => {
                        if milestones.is_empty() {
                            println!("No milestones found");
                        } else {
                            for milestone in milestones {
                                print!("[{}] {}", milestone.id, milestone.name);
                                if let Some(description) = &milestone.description {
                                    print!(" - {}", description);
                                }
                                println!();

                                if let Some(start_date) = milestone.start_date {
                                    println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
                                }
                                if let Some(release_date) = milestone.release_due_date {
                                    println!("  Release Due: {}", release_date.format("%Y-%m-%d"));
                                }
                                if milestone.archived {
                                    println!("  Status: Archived");
                                }
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing milestones: {}", e);
                    }
                }
            }
            ProjectCommands::IssueTypeList { project_id_or_key } => {
                println!("Listing issue types for project: {}", project_id_or_key);

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                match client.project().get_issue_type_list(proj_id_or_key).await {
                    Ok(issue_types) => {
                        if issue_types.is_empty() {
                            println!("No issue types found");
                        } else {
                            for issue_type in issue_types {
                                println!(
                                    "[{}] {} (Color: {})",
                                    issue_type.id, issue_type.name, issue_type.color
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing issue types: {}", e);
                    }
                }
            }
            ProjectCommands::CategoryList { project_id_or_key } => {
                println!("Listing categories for project: {}", project_id_or_key);

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                match client.project().get_category_list(proj_id_or_key).await {
                    Ok(categories) => {
                        if categories.is_empty() {
                            println!("No categories found");
                        } else {
                            for category in categories {
                                println!(
                                    "[{}] {} (Display Order: {})",
                                    category.id, category.name, category.display_order
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing categories: {}", e);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CategoryAdd {
                project_id_or_key,
                name,
            } => {
                println!(
                    "Adding category '{}' to project: {}",
                    name, project_id_or_key
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = AddCategoryParams { name: name.clone() };

                match client.project().add_category(proj_id_or_key, &params).await {
                    Ok(category) => {
                        println!("Category added successfully:");
                        println!(
                            "[{}] {} (Display Order: {})",
                            category.id, category.name, category.display_order
                        );
                    }
                    Err(e) => {
                        eprintln!("Error adding category: {}", e);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CategoryUpdate {
                project_id_or_key,
                category_id,
                name,
            } => {
                println!(
                    "Updating category {} in project {} to name '{}'",
                    category_id, project_id_or_key, name
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let cat_id = CategoryId::new(category_id);
                let params = UpdateCategoryParams { name: name.clone() };

                match client
                    .project()
                    .update_category(proj_id_or_key, cat_id, &params)
                    .await
                {
                    Ok(category) => {
                        println!("Category updated successfully:");
                        println!(
                            "[{}] {} (Display Order: {})",
                            category.id, category.name, category.display_order
                        );
                    }
                    Err(e) => {
                        eprintln!("Error updating category: {}", e);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::CategoryDelete {
                project_id_or_key,
                category_id,
            } => {
                println!(
                    "Deleting category {} from project: {}",
                    category_id, project_id_or_key
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let cat_id = CategoryId::new(category_id);

                match client
                    .project()
                    .delete_category(proj_id_or_key, cat_id)
                    .await
                {
                    Ok(category) => {
                        println!("Category deleted successfully:");
                        println!(
                            "[{}] {} (Display Order: {})",
                            category.id, category.name, category.display_order
                        );
                    }
                    Err(e) => {
                        eprintln!("Error deleting category: {}", e);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::IssueTypeAdd {
                project_id_or_key,
                name,
                color,
                template_summary,
                template_description,
            } => {
                println!(
                    "Adding issue type '{}' to project: {}",
                    name, project_id_or_key
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;

                // Parse and validate the color
                let parsed_color = color.parse::<IssueTypeColor>().map_err(|e| {
                    format!(
                        "Invalid color '{}': {}\nAvailable colors: {}",
                        color,
                        e,
                        IssueTypeColor::all_names().join(", ")
                    )
                })?;

                let params = AddIssueTypeParams {
                    name: name.clone(),
                    color: parsed_color,
                    template_summary: template_summary.clone(),
                    template_description: template_description.clone(),
                };

                match client
                    .project()
                    .add_issue_type(proj_id_or_key, &params)
                    .await
                {
                    Ok(issue_type) => {
                        println!("Issue type added successfully:");
                        println!(
                            "[{}] {} (Color: {})",
                            issue_type.id, issue_type.name, issue_type.color
                        );
                        if let Some(template_summary) = &issue_type.template_summary {
                            println!("  Template Summary: {}", template_summary);
                        }
                        if let Some(template_description) = &issue_type.template_description {
                            println!("  Template Description: {}", template_description);
                        }
                        println!("  Display Order: {}", issue_type.display_order);
                    }
                    Err(e) => {
                        eprintln!("Error adding issue type: {}", e);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::IssueTypeDelete {
                project_id_or_key,
                issue_type_id,
                substitute_issue_type_id,
            } => {
                println!(
                    "Deleting issue type {} from project: {} (substitute: {})",
                    issue_type_id, project_id_or_key, substitute_issue_type_id
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let issue_type_id_val = IssueTypeId::new(issue_type_id);
                let substitute_id = IssueTypeId::new(substitute_issue_type_id);

                let params = DeleteIssueTypeParams {
                    substitute_issue_type_id: substitute_id,
                };

                match client
                    .project()
                    .delete_issue_type(proj_id_or_key, issue_type_id_val, &params)
                    .await
                {
                    Ok(issue_type) => {
                        println!("Issue type deleted successfully:");
                        println!(
                            "[{}] {} (Color: {})",
                            issue_type.id, issue_type.name, issue_type.color
                        );
                        if let Some(template_summary) = &issue_type.template_summary {
                            println!("  Template Summary: {}", template_summary);
                        }
                        if let Some(template_description) = &issue_type.template_description {
                            println!("  Template Description: {}", template_description);
                        }
                        println!("  Display Order: {}", issue_type.display_order);
                    }
                    Err(e) => {
                        eprintln!("Error deleting issue type: {}", e);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::IssueTypeUpdate {
                project_id_or_key,
                issue_type_id,
                name,
                color,
                template_summary,
                template_description,
            } => {
                println!(
                    "Updating issue type {} in project: {}",
                    issue_type_id, project_id_or_key
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let issue_type_id_val = IssueTypeId::new(issue_type_id);

                // Parse color if provided
                let parsed_color = if let Some(color_str) = color {
                    Some(color_str.parse::<IssueTypeColor>().map_err(|e| {
                        format!(
                            "Invalid color '{}': {}\nAvailable colors: {}",
                            color_str,
                            e,
                            IssueTypeColor::all_names().join(", ")
                        )
                    })?)
                } else {
                    None
                };

                let params = UpdateIssueTypeParams {
                    name: name.clone(),
                    color: parsed_color,
                    template_summary: template_summary.clone(),
                    template_description: template_description.clone(),
                };

                match client
                    .project()
                    .update_issue_type(proj_id_or_key, issue_type_id_val, &params)
                    .await
                {
                    Ok(issue_type) => {
                        println!("Issue type updated successfully:");
                        println!(
                            "[{}] {} (Color: {})",
                            issue_type.id, issue_type.name, issue_type.color
                        );
                        if let Some(template_summary) = &issue_type.template_summary {
                            println!("  Template Summary: {}", template_summary);
                        }
                        if let Some(template_description) = &issue_type.template_description {
                            println!("  Template Description: {}", template_description);
                        }
                        println!("  Display Order: {}", issue_type.display_order);
                    }
                    Err(e) => {
                        eprintln!("Error updating issue type: {}", e);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::VersionAdd {
                project_id_or_key,
                name,
                description,
                start_date,
                release_due_date,
            } => {
                println!(
                    "Adding version/milestone '{}' to project: {}",
                    name, project_id_or_key
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let params = AddVersionParams {
                    name: name.clone(),
                    description: description.clone(),
                    start_date: start_date.clone(),
                    release_due_date: release_due_date.clone(),
                };

                match client.project().add_version(proj_id_or_key, &params).await {
                    Ok(milestone) => {
                        println!("Version/milestone added successfully:");
                        println!("[{}] {}", milestone.id, milestone.name);
                        if let Some(desc) = &milestone.description {
                            println!("  Description: {}", desc);
                        }
                        if let Some(start_date) = &milestone.start_date {
                            println!("  Start Date: {}", start_date.format("%Y-%m-%d"));
                        }
                        if let Some(release_due_date) = &milestone.release_due_date {
                            println!(
                                "  Release Due Date: {}",
                                release_due_date.format("%Y-%m-%d")
                            );
                        }
                        println!("  Archived: {}", milestone.archived);
                        if let Some(display_order) = milestone.display_order {
                            println!("  Display Order: {}", display_order);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error adding version/milestone: {}", e);
                    }
                }
            }
            #[cfg(not(feature = "project_writable"))]
            ProjectCommands::CategoryAdd { .. }
            | ProjectCommands::CategoryUpdate { .. }
            | ProjectCommands::CategoryDelete { .. }
            | ProjectCommands::IssueTypeAdd { .. }
            | ProjectCommands::IssueTypeDelete { .. }
            | ProjectCommands::IssueTypeUpdate { .. }
            | ProjectCommands::VersionAdd { .. } => {
                eprintln!(
                    "Category, issue type, and version management is not available. Please build with 'project_writable' feature."
                );
            }
            ProjectCommands::PriorityList => {
                println!("Listing priorities (space-wide):");

                match client.project().get_priority_list().await {
                    Ok(priorities) => {
                        if priorities.is_empty() {
                            println!("No priorities found");
                        } else {
                            for priority in priorities {
                                println!("[{}] {}", priority.id, priority.name);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing priorities: {}", e);
                    }
                }
            }
            ProjectCommands::ResolutionList => {
                println!("Listing resolutions (space-wide):");

                match client.project().get_resolution_list().await {
                    Ok(resolutions) => {
                        if resolutions.is_empty() {
                            println!("No resolutions found");
                        } else {
                            for resolution in resolutions {
                                println!("[{}] {}", resolution.id, resolution.name);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing resolutions: {}", e);
                    }
                }
            }
            ProjectCommands::Icon {
                project_id_or_key,
                output,
            } => {
                println!("Downloading project icon to {}", output.display());

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                match client.project().get_project_icon(proj_id_or_key).await {
                    Ok(icon_bytes) => {
                        if let Err(e) = fs::write(&output, &icon_bytes).await {
                            eprintln!("Error writing icon to {}: {}", output.display(), e);
                        } else {
                            println!(
                                "Project icon downloaded successfully to: {}",
                                output.display()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error downloading project icon: {}", e);
                    }
                }
            }
        },
        Commands::User(user_args) => match user_args.command {
            UserCommands::List => {
                println!("Listing all users:");

                match client.user().get_user_list().await {
                    Ok(users) => {
                        if users.is_empty() {
                            println!("No users found");
                        } else {
                            for user in users {
                                let user_id_str = user.user_id.as_deref().unwrap_or("N/A");
                                println!("[{}] {} ({})", user.id, user.name, user_id_str);
                                if !user.mail_address.is_empty() {
                                    println!("  Email: {}", user.mail_address);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing users: {}", e);
                    }
                }
            }
            UserCommands::Me => {
                println!("Getting current user info:");

                match client.user().get_own_user().await {
                    Ok(user) => {
                        println!("User ID: {}", user.id);
                        if let Some(login_id) = &user.user_id {
                            println!("Login ID: {}", login_id);
                        }
                        println!("Name: {}", user.name);
                        if !user.mail_address.is_empty() {
                            println!("Email: {}", user.mail_address);
                        }
                        if let Some(lang) = &user.lang {
                            println!("Language: {}", lang);
                        }
                        if let Some(last_login) = &user.last_login_time {
                            println!("Last Login: {}", last_login);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting user info: {}", e);
                    }
                }
            }
            UserCommands::Icon { user_id, output } => {
                println!("Downloading user icon to {}", output.display());

                match client.user().get_user_icon(user_id).await {
                    Ok(icon_bytes) => {
                        if let Err(e) = fs::write(&output, &icon_bytes).await {
                            eprintln!("Error writing icon to {}: {}", output.display(), e);
                        } else {
                            println!("User icon downloaded successfully to: {}", output.display());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error downloading user icon: {}", e);
                    }
                }
            }
        },
    }

    Ok(())
}
