#[cfg(feature = "git_writable")]
use backlog_api_client::AddPullRequestParams;
#[cfg(feature = "issue_writable")]
use backlog_api_client::LinkSharedFilesToIssueParamsBuilder;
#[cfg(feature = "git_writable")]
#[allow(unused_imports)]
use backlog_api_client::UpdatePullRequestCommentParams;
#[cfg(feature = "git_writable")]
#[allow(unused_imports)]
use backlog_api_client::UpdatePullRequestParams;
use backlog_api_client::{
    AddCommentParamsBuilder, AttachmentId, GetIssueListParamsBuilder, GetPullRequestCountParams,
    IssueIdOrKey, ProjectId, ProjectIdOrKey, PullRequestAttachmentId, PullRequestCommentId,
    PullRequestNumber, RepositoryIdOrName, StatusId, UserId, WikiId, backlog_issue,
    client::BacklogApiClient,
};
#[cfg(any(feature = "git_writable", feature = "issue_writable"))]
use backlog_core::identifier::IssueId;
#[cfg(feature = "issue_writable")]
use backlog_core::identifier::SharedFileId;
#[cfg(feature = "wiki")]
use backlog_core::identifier::WikiAttachmentId;
use backlog_core::identifier::{CommentId, Identifier};
#[cfg(any(feature = "issue_writable", feature = "project_writable"))]
use backlog_core::{
    ApiDate, IssueKey,
    identifier::{CategoryId, IssueTypeId, MilestoneId, PriorityId, ResolutionId},
};
#[cfg(feature = "project_writable")]
use backlog_domain_models::{IssueTypeColor, StatusColor};
#[cfg(feature = "issue_writable")]
use backlog_issue::DeleteCommentParams;
#[cfg(feature = "issue_writable")]
use backlog_issue::{AddIssueParamsBuilder, UpdateIssueParamsBuilder};
use backlog_project::GetProjectListParams;
#[cfg(feature = "project_writable")]
use backlog_project::{
    AddCategoryParams, AddIssueTypeParams, AddMilestoneParams, AddStatusParams,
    DeleteCategoryParams, DeleteIssueTypeParams, DeleteStatusParams, DeleteVersionParams,
    UpdateCategoryParams, UpdateIssueTypeParams, UpdateStatusOrderParams, UpdateStatusParams,
    UpdateVersionParams,
};
use backlog_space::GetSpaceLogoParams;
use backlog_user::GetOwnUserParams;
use backlog_user::GetUserIconParams;
use backlog_user::GetUserListParams;
use backlog_user::GetUserParams;
#[cfg(feature = "wiki_writable")]
use backlog_wiki::UpdateWikiParams;
#[cfg(feature = "project_writable")]
use chrono::{DateTime, Utc};
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
    /// Manage wikis
    #[cfg(feature = "wiki")]
    Wiki(WikiArgs),
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
        #[clap(short = 'n', long)]
        pr_number: u64,
    },
    /// Download a pull request attachment
    #[command(about = "Download a pull request attachment")]
    DownloadAttachment(DownloadPrAttachmentArgs),
    /// Delete a pull request attachment
    #[cfg(feature = "git_writable")]
    #[command(about = "Delete a pull request attachment")]
    DeleteAttachment(DeletePrAttachmentArgs),
    /// Update a pull request
    #[cfg(feature = "git_writable")]
    Update {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull Request number
        #[clap(long)]
        pr_number: u64,
        /// Update summary (title)
        #[clap(long)]
        summary: Option<String>,
        /// Update description
        #[clap(long)]
        description: Option<String>,
        /// Related issue ID
        #[clap(long)]
        issue_id: Option<u32>,
        /// Assignee user ID
        #[clap(long)]
        assignee_id: Option<u32>,
        /// Notification user IDs (comma-separated)
        #[clap(long, value_delimiter = ',')]
        notify_user_ids: Option<Vec<u32>>,
        /// Comment to add with the update
        #[clap(long)]
        comment: Option<String>,
    },
    /// Update a pull request comment
    #[cfg(feature = "git_writable")]
    CommentUpdate {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull Request number
        #[clap(long)]
        pr_number: u64,
        /// Comment ID to update
        #[clap(long)]
        comment_id: u32,
        /// New content for the comment
        #[clap(short, long)]
        content: String,
    },
    /// Get the number of comments on a pull request
    CommentCount {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull Request number
        #[clap(long)]
        pr_number: u64,
    },
    /// Get the number of pull requests in a repository
    Count {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Filter by status IDs (comma-separated, e.g., "1,2,3")
        #[clap(long)]
        status_ids: Option<String>,
        /// Filter by assignee user IDs (comma-separated, e.g., "100,200")
        #[clap(long)]
        assignee_ids: Option<String>,
        /// Filter by issue IDs (comma-separated, e.g., "1000,2000")
        #[clap(long)]
        issue_ids: Option<String>,
        /// Filter by created user IDs (comma-separated, e.g., "300,400")
        #[clap(long)]
        created_user_ids: Option<String>,
        /// Offset for pagination
        #[clap(long)]
        offset: Option<u32>,
        /// Number of pull requests to count (1-100, default 20)
        #[clap(long)]
        count: Option<u8>,
    },
    /// Create a new pull request
    #[cfg(feature = "git_writable")]
    Create {
        /// Project ID or Key
        #[clap(short, long)]
        project_id: String,
        /// Repository ID or Name
        #[clap(short, long)]
        repo_id: String,
        /// Pull request title
        #[clap(short, long)]
        summary: String,
        /// Pull request description
        #[clap(short, long)]
        description: String,
        /// Target merge branch
        #[clap(short, long)]
        base: String,
        /// Source branch to be merged
        #[clap(short = 'B', long)]
        branch: String,
        /// Related issue ID
        #[clap(long)]
        issue_id: Option<u32>,
        /// Assignee user ID
        #[clap(long)]
        assignee_id: Option<u32>,
        /// User IDs to notify (comma-separated, e.g., "123,456")
        #[clap(long)]
        notify_user_ids: Option<String>,
        /// Attachment IDs (comma-separated, e.g., "789,101112")
        #[clap(long)]
        attachment_ids: Option<String>,
    },
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

#[cfg(feature = "git_writable")]
#[derive(Args, Debug)]
struct DeletePrAttachmentArgs {
    /// Project ID or Key
    #[clap(short = 'p', long)]
    project_id: String,
    /// Repository ID or Name
    #[clap(short = 'r', long)]
    repo_id: String,
    /// Pull Request number
    #[clap(short = 'n', long)]
    pr_number: u64,
    /// The numeric ID of the attachment to delete
    #[clap(short = 'a', long)]
    attachment_id: u32,
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
    /// Update an existing comment
    #[cfg(feature = "issue_writable")]
    #[command(about = "Update an existing comment")]
    UpdateComment(UpdateCommentArgs),
    /// Delete a comment from an issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Delete a comment from an issue")]
    DeleteComment(DeleteCommentArgs),
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
    /// List shared files linked to an issue
    #[command(about = "List shared files linked to an issue")]
    ListSharedFiles {
        /// Issue ID or Key (e.g., "PROJECT-123" or "12345")
        #[clap(name = "ISSUE_ID_OR_KEY")]
        issue_id_or_key: String,
    },
    /// Link shared files to an issue
    #[cfg(feature = "issue_writable")]
    #[command(about = "Link shared files to an issue")]
    LinkSharedFiles {
        /// Issue ID or Key (e.g., "PROJECT-123" or "12345")
        #[clap(name = "ISSUE_ID_OR_KEY")]
        issue_id_or_key: String,
        /// Shared file IDs to link (comma-separated)
        #[clap(short, long, value_delimiter = ',')]
        file_ids: Vec<u32>,
    },
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

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
struct UpdateCommentArgs {
    /// Issue ID or key (e.g., 'PROJECT-123')
    #[clap(short, long)]
    issue_id: String,

    /// Comment ID to update
    #[clap(short = 'c', long)]
    comment_id: u32,

    /// New content for the comment
    #[clap(short = 'n', long)]
    content: String,
}

#[cfg(feature = "issue_writable")]
#[derive(Args, Debug)]
struct DeleteCommentArgs {
    /// Issue ID or key (e.g., 'PROJECT-123')
    #[clap(short, long)]
    issue_id: String,

    /// Comment ID to delete
    #[clap(short = 'c', long)]
    comment_id: u32,
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
    /// Update a version/milestone in a project
    #[cfg(feature = "project_writable")]
    VersionUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Version ID to update
        #[clap(long)]
        version_id: u32,
        /// New version name
        #[clap(short, long)]
        name: String,
        /// New version description
        #[clap(short, long)]
        description: Option<String>,
        /// New start date (YYYY-MM-DD)
        #[clap(long)]
        start_date: Option<String>,
        /// New release due date (YYYY-MM-DD)
        #[clap(long)]
        release_due_date: Option<String>,
        /// Archive the version
        #[clap(long)]
        archived: Option<bool>,
    },
    /// Delete a version/milestone from a project
    #[cfg(feature = "project_writable")]
    VersionDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Version ID to delete
        #[clap(long)]
        version_id: u32,
    },
    /// Add a status to a project
    #[cfg(feature = "project_writable")]
    StatusAdd {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status name
        #[clap(short, long)]
        name: String,
        /// Status color (red, coral, pink, light-purple, blue, green, light-green, orange, magenta, dark-gray)
        #[clap(short, long)]
        color: String,
    },
    /// Update a status in a project
    #[cfg(feature = "project_writable")]
    StatusUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status ID to update
        #[clap(long)]
        status_id: u32,
        /// New status name (optional)
        #[clap(short, long)]
        name: Option<String>,
        /// New status color (optional: red, coral, pink, light-purple, blue, green, light-green, orange, magenta, dark-gray)
        #[clap(short, long)]
        color: Option<String>,
    },
    /// Delete a status from a project
    #[cfg(feature = "project_writable")]
    StatusDelete {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status ID to delete
        #[clap(long)]
        status_id: u32,
        /// Substitute status ID for existing issues
        #[clap(long)]
        substitute_status_id: u32,
    },
    /// Update the display order of statuses in a project
    #[cfg(feature = "project_writable")]
    StatusOrderUpdate {
        /// Project ID or Key
        #[clap(name = "PROJECT_ID_OR_KEY")]
        project_id_or_key: String,
        /// Status IDs in desired display order (comma-separated)
        #[clap(long)]
        status_ids: String,
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
    /// Show user details
    Show {
        /// User ID
        #[clap(name = "USER_ID")]
        user_id: u32,
    },
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

#[cfg(feature = "wiki")]
#[derive(Parser)]
struct WikiArgs {
    #[clap(subcommand)]
    command: WikiCommands,
}

#[cfg(feature = "wiki")]
#[derive(Parser)]
enum WikiCommands {
    /// List attachments for a wiki page
    ListAttachments {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
    },
    /// Download an attachment from a wiki page
    DownloadAttachment {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// Attachment ID
        #[clap(name = "ATTACHMENT_ID")]
        attachment_id: u32,
        /// Output file path (if not specified, use original filename)
        #[clap(short, long)]
        output: Option<String>,
    },
    /// Update a wiki page
    #[cfg(feature = "wiki_writable")]
    Update {
        /// Wiki ID
        #[clap(name = "WIKI_ID")]
        wiki_id: u32,
        /// New wiki page name
        #[clap(long)]
        name: Option<String>,
        /// New wiki page content
        #[clap(long)]
        content: Option<String>,
        /// Send email notification of update
        #[clap(long)]
        mail_notify: Option<bool>,
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
                let params = backlog_api_client::GetRepositoryListParams::new(proj_id_or_key);
                let repos = client.git().get_repository_list(params).await?;
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
                let params =
                    backlog_api_client::GetRepositoryParams::new(proj_id_or_key, repo_id_or_name);
                let repo = client.git().get_repository(params).await?;
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
                let params = backlog_api_client::GetPullRequestListParams::new(
                    proj_id_or_key,
                    repo_id_or_name,
                );
                let prs = client.git().get_pull_request_list(params).await?;
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

                let params = backlog_api_client::GetPullRequestParams::new(
                    proj_id_or_key,
                    repo_id_or_name,
                    pr_num,
                );
                let pr = client.git().get_pull_request(params).await?;
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

                let params = backlog_api_client::DownloadPullRequestAttachmentParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                    parsed_attachment_id,
                );
                match client.git().download_pull_request_attachment(params).await {
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
            #[cfg(feature = "git_writable")]
            PrCommands::DeleteAttachment(del_args) => {
                println!(
                    "Deleting attachment {} from PR #{} in repo {} (project {})",
                    del_args.attachment_id,
                    del_args.pr_number,
                    del_args.repo_id,
                    del_args.project_id
                );

                let parsed_project_id =
                    ProjectIdOrKey::from_str(&del_args.project_id).map_err(|e| {
                        format!(
                            "Failed to parse project_id '{}': {}",
                            del_args.project_id, e
                        )
                    })?;
                let parsed_repo_id =
                    RepositoryIdOrName::from_str(&del_args.repo_id).map_err(|e| {
                        format!("Failed to parse repo_id '{}': {}", del_args.repo_id, e)
                    })?;
                let parsed_attachment_id = PullRequestAttachmentId::new(del_args.attachment_id);
                let parsed_pr_number = PullRequestNumber::from(del_args.pr_number);

                let params = backlog_api_client::DeletePullRequestAttachmentParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                    parsed_attachment_id,
                );
                match client.git().delete_pull_request_attachment(params).await {
                    Ok(deleted_attachment) => {
                        println!("✅ Attachment deleted successfully");
                        println!("Deleted attachment ID: {}", deleted_attachment.id.value());
                        println!("Name: {}", deleted_attachment.name);
                        println!("Size: {} bytes", deleted_attachment.size);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete PR attachment: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "git_writable")]
            PrCommands::Update {
                project_id,
                repo_id,
                pr_number,
                summary,
                description,
                issue_id,
                assignee_id,
                notify_user_ids,
                comment,
            } => {
                println!(
                    "Updating PR #{} in repo {} (project {})",
                    pr_number, repo_id, project_id
                );

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{}': {}", project_id, e))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{}': {}", repo_id, e))?;
                let parsed_pr_number = PullRequestNumber::from(pr_number);

                let mut params = UpdatePullRequestParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                );

                if let Some(summary) = summary {
                    params = params.summary(summary.clone());
                }

                if let Some(description) = description {
                    params = params.description(description.clone());
                }

                if let Some(issue_id) = issue_id {
                    params = params.issue_id(IssueId::new(issue_id));
                }

                if let Some(assignee_id) = assignee_id {
                    params = params.assignee_id(UserId::new(assignee_id));
                }

                if let Some(notify_user_ids) = notify_user_ids {
                    let user_ids: Vec<UserId> =
                        notify_user_ids.iter().map(|&id| UserId::new(id)).collect();
                    params = params.notified_user_ids(user_ids);
                }

                if let Some(comment) = comment {
                    params = params.comment(comment.clone());
                }

                match client.git().update_pull_request(params).await {
                    Ok(pull_request) => {
                        println!("✅ Pull request updated successfully");
                        println!("ID: {}", pull_request.id.value());
                        println!("Number: {}", pull_request.number.value());
                        println!("Summary: {}", pull_request.summary);
                        if let Some(description) = &pull_request.description {
                            println!("Description: {}", description);
                        }
                        if let Some(assignee) = &pull_request.assignee {
                            println!("Assignee: {} (ID: {})", assignee.name, assignee.id.value());
                        }
                        if let Some(issue) = &pull_request.related_issue {
                            println!("Related Issue ID: {}", issue.id.value());
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update pull request: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "git_writable")]
            PrCommands::CommentUpdate {
                project_id,
                repo_id,
                pr_number,
                comment_id,
                content,
            } => {
                println!(
                    "Updating comment {} for PR #{} in repo {} (project {})",
                    comment_id, pr_number, repo_id, project_id
                );

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{}': {}", project_id, e))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{}': {}", repo_id, e))?;
                let parsed_pr_number = PullRequestNumber::from(pr_number);
                let parsed_comment_id = PullRequestCommentId::new(comment_id);

                let params = backlog_api_client::UpdatePullRequestCommentParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                    parsed_comment_id,
                    &content,
                );

                match client.git().update_pull_request_comment(params).await {
                    Ok(comment) => {
                        println!("✅ Pull request comment updated successfully");
                        println!("Comment ID: {}", comment.id.value());
                        println!("Content: {}", comment.content);
                        println!(
                            "Created by: {} (ID: {})",
                            comment.created_user.name,
                            comment.created_user.id.value()
                        );
                        println!("Created: {}", comment.created);
                        println!("Updated: {}", comment.updated);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update pull request comment: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            PrCommands::CommentCount {
                project_id,
                repo_id,
                pr_number,
            } => {
                println!(
                    "Getting comment count for PR #{} in repo {} (project {})",
                    pr_number, repo_id, project_id
                );

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{}': {}", project_id, e))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{}': {}", repo_id, e))?;
                let parsed_pr_number = PullRequestNumber::from(pr_number);

                let params = backlog_api_client::GetPullRequestCommentCountParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    parsed_pr_number,
                );
                match client.git().get_pull_request_comment_count(params).await {
                    Ok(count_response) => {
                        println!("✅ Pull request comment count retrieved successfully");
                        println!("Comment count: {}", count_response.count);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get pull request comment count: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            PrCommands::Count {
                project_id,
                repo_id,
                status_ids,
                assignee_ids,
                issue_ids,
                created_user_ids,
                offset: _,
                count: _,
            } => {
                println!(
                    "Getting pull request count for repo {} (project {})",
                    repo_id, project_id
                );

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{}': {}", project_id, e))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{}': {}", repo_id, e))?;

                // Parse filter parameters
                let mut params = GetPullRequestCountParams::new(parsed_project_id, parsed_repo_id);

                // Parse status IDs
                if let Some(status_ids_str) = status_ids {
                    let status_ids: Result<Vec<StatusId>, _> = status_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(StatusId::new))
                        .collect();
                    match status_ids {
                        Ok(ids) => params = params.status_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse status_ids: {}", e);
                            std::process::exit(1);
                        }
                    };
                }

                // Parse assignee IDs
                if let Some(assignee_ids_str) = assignee_ids {
                    let assignee_ids: Result<Vec<UserId>, _> = assignee_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(UserId::new))
                        .collect();
                    match assignee_ids {
                        Ok(ids) => params = params.assignee_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse assignee_ids: {}", e);
                            std::process::exit(1);
                        }
                    };
                }

                // Parse issue IDs
                if let Some(issue_ids_str) = issue_ids {
                    let issue_ids: Result<Vec<IssueId>, _> = issue_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(IssueId::new))
                        .collect();
                    match issue_ids {
                        Ok(ids) => params = params.issue_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse issue_ids: {}", e);
                            std::process::exit(1);
                        }
                    };
                }

                // Parse created user IDs
                if let Some(created_user_ids_str) = created_user_ids {
                    let created_user_ids: Result<Vec<UserId>, _> = created_user_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(UserId::new))
                        .collect();
                    match created_user_ids {
                        Ok(ids) => params = params.created_user_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse created_user_ids: {}", e);
                            std::process::exit(1);
                        }
                    };
                }

                match client.git().get_pull_request_count(params).await {
                    Ok(count_response) => {
                        println!("✅ Pull request count retrieved successfully");
                        println!("Pull request count: {}", count_response.count);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to get pull request count: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "git_writable")]
            PrCommands::Create {
                project_id,
                repo_id,
                summary,
                description,
                base,
                branch,
                issue_id,
                assignee_id,
                notify_user_ids,
                attachment_ids,
            } => {
                println!(
                    "Creating pull request in repo {} (project {})",
                    repo_id, project_id
                );

                let parsed_project_id = ProjectIdOrKey::from_str(&project_id)
                    .map_err(|e| format!("Failed to parse project_id '{}': {}", project_id, e))?;
                let parsed_repo_id = RepositoryIdOrName::from_str(&repo_id)
                    .map_err(|e| format!("Failed to parse repo_id '{}': {}", repo_id, e))?;

                // Build parameters
                let mut params = AddPullRequestParams::new(
                    parsed_project_id,
                    parsed_repo_id,
                    summary.clone(),
                    description.clone(),
                    base.clone(),
                    branch.clone(),
                );

                // Parse optional issue ID
                if let Some(issue_id) = issue_id {
                    params = params.issue_id(backlog_core::identifier::IssueId::new(issue_id));
                }

                // Parse optional assignee ID
                if let Some(assignee_id) = assignee_id {
                    params = params.assignee_id(UserId::new(assignee_id));
                }

                // Parse notify user IDs
                if let Some(notify_user_ids_str) = notify_user_ids {
                    let notify_user_ids: Result<Vec<UserId>, _> = notify_user_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(UserId::new))
                        .collect();
                    match notify_user_ids {
                        Ok(ids) => params = params.notified_user_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse notify_user_ids: {}", e);
                            std::process::exit(1);
                        }
                    };
                }

                // Parse attachment IDs
                if let Some(attachment_ids_str) = attachment_ids {
                    let attachment_ids: Result<Vec<AttachmentId>, _> = attachment_ids_str
                        .split(',')
                        .map(|s| s.trim().parse::<u32>().map(AttachmentId::new))
                        .collect();
                    match attachment_ids {
                        Ok(ids) => params = params.attachment_ids(ids),
                        Err(e) => {
                            eprintln!("❌ Failed to parse attachment_ids: {}", e);
                            std::process::exit(1);
                        }
                    };
                }

                match client.git().add_pull_request(params).await {
                    Ok(pull_request) => {
                        println!("✅ Pull request created successfully");
                        println!("ID: {}", pull_request.id.value());
                        println!("Number: {}", pull_request.number.value());
                        println!("Summary: {}", pull_request.summary);
                        if let Some(description) = &pull_request.description {
                            println!("Description: {}", description);
                        }
                        println!("Base: {}", pull_request.base);
                        println!("Branch: {}", pull_request.branch);
                        if let Some(assignee) = &pull_request.assignee {
                            println!("Assignee: {} (ID: {})", assignee.name, assignee.id.value());
                        }
                        if let Some(issue) = &pull_request.related_issue {
                            println!("Related Issue ID: {}", issue.id.value());
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to create pull request: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },
        Commands::Issue(issue_args) => match issue_args.command {
            IssueCommands::Show { issue_id_or_key } => {
                println!("Showing issue: {}", issue_id_or_key);
                let parsed_issue_id_or_key = issue_id_or_key.parse::<IssueIdOrKey>()?;
                let issue = client
                    .issue()
                    .get_issue(backlog_issue::GetIssueParams::new(parsed_issue_id_or_key))
                    .await?;
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

                let params = backlog_issue::GetAttachmentFileParams::new(
                    parsed_issue_id_or_key,
                    parsed_attachment_id,
                );
                match client.issue().get_attachment_file(params).await {
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
                builder.issue_id_or_key(parsed_issue_id_or_key);
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

                match client.issue().add_comment(params).await {
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
            IssueCommands::UpdateComment(args) => {
                use backlog_core::identifier::CommentId;
                use backlog_issue::UpdateCommentParams;

                let params = UpdateCommentParams {
                    issue_id_or_key: args.issue_id.parse::<IssueKey>()?.into(),
                    comment_id: CommentId::new(args.comment_id),
                    content: args.content,
                };

                match client.issue().update_comment(params).await {
                    Ok(comment) => {
                        println!("✅ Comment updated successfully");
                        println!("Comment ID: {}", comment.id);
                        println!("Content: {}", comment.content.unwrap_or_default());
                        println!("Updated: {}", comment.updated);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update comment: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::DeleteComment(args) => {
                use backlog_core::identifier::CommentId;

                let params = DeleteCommentParams {
                    issue_id_or_key: args.issue_id.parse::<IssueKey>()?.into(),
                    comment_id: CommentId::new(args.comment_id),
                };

                match client.issue().delete_comment(params).await {
                    Ok(comment) => {
                        println!("✅ Comment deleted successfully");
                        println!("Deleted Comment ID: {}", comment.id);
                        println!("Deleted Content: {}", comment.content.unwrap_or_default());
                        println!("Originally Created: {}", comment.created);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete comment: {}", e);
                        std::process::exit(1);
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
                builder.issue_id_or_key(issue_id_or_key);

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

                match client.issue().update_issue(params).await {
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

                match client
                    .issue()
                    .delete_issue(backlog_issue::DeleteIssueParams::new(issue_key))
                    .await
                {
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

                match client
                    .issue()
                    .count_comment(backlog_issue::CountCommentParams::new(
                        parsed_issue_id_or_key,
                    ))
                    .await
                {
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
                    .get_comment(backlog_issue::GetCommentParams::new(
                        parsed_issue_id_or_key,
                        comment_id,
                    ))
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
            IssueCommands::ListSharedFiles { issue_id_or_key } => {
                println!("Listing shared files for issue: {}", issue_id_or_key);

                let parsed_issue_id_or_key =
                    IssueIdOrKey::from_str(&issue_id_or_key).map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            issue_id_or_key, e
                        )
                    })?;

                match client
                    .issue()
                    .get_shared_file_list(backlog_issue::GetSharedFileListParams::new(
                        parsed_issue_id_or_key,
                    ))
                    .await
                {
                    Ok(shared_files) => {
                        if shared_files.is_empty() {
                            println!("No shared files found for this issue.");
                        } else {
                            println!("Found {} shared file(s):", shared_files.len());
                            println!();

                            for (index, file) in shared_files.iter().enumerate() {
                                println!("{}. {}", index + 1, file.name);
                                println!("   ID: {}", file.id);
                                println!("   Directory: {}", file.dir);
                                match &file.content {
                                    backlog_issue::models::FileContent::File { size } => {
                                        println!("   Type: File");
                                        println!("   Size: {} bytes", size);
                                    }
                                    backlog_issue::models::FileContent::Directory => {
                                        println!("   Type: Directory");
                                    }
                                }
                                println!("   Created by: {}", file.created_user.name);
                                println!("   Created at: {}", file.created);
                                if let Some(updated_user) = &file.updated_user {
                                    println!("   Updated by: {}", updated_user.name);
                                }
                                if let Some(updated) = &file.updated {
                                    println!("   Updated at: {}", updated);
                                }
                                println!();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing shared files: {}", e);
                    }
                }
            }
            #[cfg(feature = "issue_writable")]
            IssueCommands::LinkSharedFiles {
                issue_id_or_key,
                file_ids,
            } => {
                println!(
                    "Linking {} shared file(s) to issue: {}",
                    file_ids.len(),
                    issue_id_or_key
                );

                let parsed_issue_id_or_key =
                    IssueIdOrKey::from_str(&issue_id_or_key).map_err(|e| {
                        format!(
                            "Failed to parse issue_id_or_key '{}': {}",
                            issue_id_or_key, e
                        )
                    })?;

                let shared_file_ids: Vec<SharedFileId> =
                    file_ids.iter().map(|&id| SharedFileId::new(id)).collect();

                let params = LinkSharedFilesToIssueParamsBuilder::default()
                    .issue_id_or_key(parsed_issue_id_or_key)
                    .shared_file_ids(shared_file_ids)
                    .build()
                    .map_err(|e| format!("Failed to build parameters: {}", e))?;

                match client.issue().link_shared_files_to_issue(params).await {
                    Ok(linked_files) => {
                        println!(
                            "✅ Successfully linked {} shared file(s) to the issue!",
                            linked_files.len()
                        );
                        println!();

                        for (index, file) in linked_files.iter().enumerate() {
                            println!("{}. {}", index + 1, file.name);
                            println!("   ID: {}", file.id);
                            println!("   Directory: {}", file.dir);
                            match &file.content {
                                backlog_issue::models::FileContent::File { size } => {
                                    println!("   Type: File");
                                    println!("   Size: {} bytes", size);
                                }
                                backlog_issue::models::FileContent::Directory => {
                                    println!("   Type: Directory");
                                }
                            }
                            println!("   Created by: {}", file.created_user.name);
                            println!("   Created at: {}", file.created);
                            println!();
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to link shared files to issue: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "issue_writable"))]
            IssueCommands::Create(_)
            | IssueCommands::Update(_)
            | IssueCommands::Delete(_)
            | IssueCommands::LinkSharedFiles { .. } => {
                eprintln!(
                    "Issue creation, update, and deletion are not available. Please build with 'issue_writable' feature."
                );
            }
        },
        Commands::Space(space_args) => match space_args.command {
            SpaceCommands::Logo { output } => {
                println!("Downloading space logo to {}", output.display());

                match client
                    .space()
                    .get_space_logo(GetSpaceLogoParams::new())
                    .await
                {
                    Ok(downloaded_file) => {
                        if let Err(e) = fs::write(&output, &downloaded_file.bytes).await {
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

                let params = GetProjectListParams {
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
                let params = backlog_project::GetProjectDetailParams::new(proj_id_or_key);
                match client.project().get_project(params).await {
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
                let params = backlog_project::GetStatusListParams::new(proj_id_or_key);
                match client.project().get_status_list(params).await {
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
                    .get_version_milestone_list(backlog_project::GetMilestoneListParams::new(
                        proj_id_or_key,
                    ))
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
                let params = backlog_project::GetIssueTypeListParams::new(proj_id_or_key);
                match client.project().get_issue_type_list(params).await {
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
                let params = backlog_project::GetCategoryListParams::new(proj_id_or_key);
                match client.project().get_category_list(params).await {
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
                let params = AddCategoryParams::new(proj_id_or_key, name.clone());

                match client.project().add_category(params).await {
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
                let params = UpdateCategoryParams::new(proj_id_or_key, cat_id, name.clone());

                match client.project().update_category(params).await {
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
                    .delete_category(DeleteCategoryParams::new(proj_id_or_key, cat_id))
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

                let mut params = AddIssueTypeParams::new(proj_id_or_key, &name, parsed_color);
                params.template_summary = template_summary.clone();
                params.template_description = template_description.clone();

                match client.project().add_issue_type(params).await {
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

                let params =
                    DeleteIssueTypeParams::new(proj_id_or_key, issue_type_id_val, substitute_id);

                match client.project().delete_issue_type(params).await {
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

                let mut params = UpdateIssueTypeParams::new(proj_id_or_key, issue_type_id_val);
                params.name = name.clone();
                params.color = parsed_color;
                params.template_summary = template_summary.clone();
                params.template_description = template_description.clone();

                match client.project().update_issue_type(params).await {
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
                let mut params = AddMilestoneParams::new(proj_id_or_key, &name);
                params.description = description.clone();
                params.start_date = start_date.as_ref().map(|d| {
                    DateTime::parse_from_str(&format!("{}T00:00:00Z", d), "%Y-%m-%dT%H:%M:%SZ")
                        .map(|dt| ApiDate::from(dt.with_timezone(&Utc)))
                        .unwrap_or_else(|_| panic!("Invalid date format: {}", d))
                });
                params.release_due_date = release_due_date.as_ref().map(|d| {
                    DateTime::parse_from_str(&format!("{}T00:00:00Z", d), "%Y-%m-%dT%H:%M:%SZ")
                        .map(|dt| ApiDate::from(dt.with_timezone(&Utc)))
                        .unwrap_or_else(|_| panic!("Invalid date format: {}", d))
                });

                match client.project().add_version(params).await {
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
            #[cfg(feature = "project_writable")]
            ProjectCommands::VersionUpdate {
                project_id_or_key,
                version_id,
                name,
                description,
                start_date,
                release_due_date,
                archived,
            } => {
                println!(
                    "Updating version/milestone {} in project: {}",
                    version_id, project_id_or_key
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let version_id_val = MilestoneId::new(version_id);
                let mut params = UpdateVersionParams::new(proj_id_or_key, version_id_val, &name);
                params.description = description.clone();
                params.start_date = start_date.as_ref().map(|d| {
                    DateTime::parse_from_str(&format!("{}T00:00:00Z", d), "%Y-%m-%dT%H:%M:%SZ")
                        .map(|dt| ApiDate::from(dt.with_timezone(&Utc)))
                        .unwrap_or_else(|_| panic!("Invalid date format: {}", d))
                });
                params.release_due_date = release_due_date.as_ref().map(|d| {
                    DateTime::parse_from_str(&format!("{}T00:00:00Z", d), "%Y-%m-%dT%H:%M:%SZ")
                        .map(|dt| ApiDate::from(dt.with_timezone(&Utc)))
                        .unwrap_or_else(|_| panic!("Invalid date format: {}", d))
                });
                params.archived = archived;

                match client.project().update_version(params).await {
                    Ok(milestone) => {
                        println!("Version/milestone updated successfully:");
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
                        eprintln!("Error updating version/milestone: {}", e);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::VersionDelete {
                project_id_or_key,
                version_id,
            } => {
                println!(
                    "Deleting version/milestone {} from project: {}",
                    version_id, project_id_or_key
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let version_id_val = MilestoneId::new(version_id);
                let params = DeleteVersionParams::new(proj_id_or_key, version_id_val);

                match client.project().delete_version(params).await {
                    Ok(milestone) => {
                        println!("Version/milestone deleted successfully:");
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
                        eprintln!("Error deleting version/milestone: {}", e);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::StatusAdd {
                project_id_or_key,
                name,
                color,
            } => {
                println!("Adding status '{}' to project: {}", name, project_id_or_key);

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let parsed_color = StatusColor::from_str(&color)?;

                let params = AddStatusParams::new(proj_id_or_key, &name, parsed_color);

                match client.project().add_status(params).await {
                    Ok(status) => {
                        println!("✅ Status added successfully:");
                        println!("ID: {}", status.id);
                        println!("Name: {}", status.name);
                        println!("Color: {}", status.color);
                        println!("Display Order: {}", status.display_order);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to add status: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::StatusUpdate {
                project_id_or_key,
                status_id,
                name,
                color,
            } => {
                println!(
                    "Updating status {} in project: {}",
                    status_id, project_id_or_key
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let status_id_val = StatusId::new(status_id);

                let parsed_color = if let Some(color_str) = &color {
                    Some(StatusColor::from_str(color_str)?)
                } else {
                    None
                };

                let mut params = UpdateStatusParams::new(proj_id_or_key, status_id_val);

                if let Some(name) = name {
                    params = params.name(name);
                }

                if let Some(color) = parsed_color {
                    params = params.color(color);
                }

                match client.project().update_status(params).await {
                    Ok(status) => {
                        println!("✅ Status updated successfully:");
                        println!("ID: {}", status.id);
                        println!("Name: {}", status.name);
                        println!("Color: {}", status.color);
                        println!("Display Order: {}", status.display_order);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update status: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::StatusDelete {
                project_id_or_key,
                status_id,
                substitute_status_id,
            } => {
                println!(
                    "Deleting status {} from project: {} (substitute: {})",
                    status_id, project_id_or_key, substitute_status_id
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;
                let status_id_val = StatusId::new(status_id);
                let substitute_id = StatusId::new(substitute_status_id);

                let params = DeleteStatusParams::new(proj_id_or_key, status_id_val, substitute_id);

                match client.project().delete_status(params).await {
                    Ok(status) => {
                        println!("✅ Status deleted successfully:");
                        println!("ID: {}", status.id);
                        println!("Name: {}", status.name);
                        println!("Color: {}", status.color);
                        println!("Display Order: {}", status.display_order);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to delete status: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "project_writable")]
            ProjectCommands::StatusOrderUpdate {
                project_id_or_key,
                status_ids,
            } => {
                println!(
                    "Updating status order in project: {} with IDs: {}",
                    project_id_or_key, status_ids
                );

                let proj_id_or_key = project_id_or_key.parse::<ProjectIdOrKey>()?;

                // Parse comma-separated status IDs
                let parsed_status_ids: Result<Vec<StatusId>, _> = status_ids
                    .split(',')
                    .map(|s| s.trim().parse::<u32>().map(StatusId::new))
                    .collect();

                let status_id_vec = match parsed_status_ids {
                    Ok(ids) => ids,
                    Err(e) => {
                        eprintln!("❌ Error parsing status IDs '{}': {}", status_ids, e);
                        std::process::exit(1);
                    }
                };

                let params = UpdateStatusOrderParams::new(proj_id_or_key, status_id_vec);

                match client.project().update_status_order(params).await {
                    Ok(statuses) => {
                        println!("✅ Status order updated successfully:");
                        for (index, status) in statuses.iter().enumerate() {
                            println!(
                                "{}. [{}] {} (Color: {})",
                                index + 1,
                                status.id,
                                status.name,
                                status.color
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update status order: {}", e);
                        std::process::exit(1);
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
            | ProjectCommands::VersionAdd { .. }
            | ProjectCommands::VersionUpdate { .. }
            | ProjectCommands::VersionDelete { .. }
            | ProjectCommands::StatusAdd { .. }
            | ProjectCommands::StatusUpdate { .. }
            | ProjectCommands::StatusDelete { .. }
            | ProjectCommands::StatusOrderUpdate { .. } => {
                eprintln!(
                    "Category, issue type, version, and status management is not available. Please build with 'project_writable' feature."
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
                let params = backlog_project::GetProjectIconParams::new(proj_id_or_key);
                match client.project().get_project_icon(params).await {
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

                match client.user().get_user_list(GetUserListParams::new()).await {
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

                match client.user().get_own_user(GetOwnUserParams::new()).await {
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
            UserCommands::Show { user_id } => {
                println!("Getting user info for user ID: {}", user_id);

                match client.user().get_user(GetUserParams::new(user_id)).await {
                    Ok(user) => {
                        println!("✅ User found");
                        println!("ID: {}", user.id);
                        if let Some(login_id) = &user.user_id {
                            println!("Login ID: {}", login_id);
                        }
                        println!("Name: {}", user.name);
                        println!("Role: {}", user.role_type);
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
                        eprintln!("❌ Failed to get user: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            UserCommands::Icon { user_id, output } => {
                println!("Downloading user icon to {}", output.display());

                match client
                    .user()
                    .get_user_icon(GetUserIconParams::new(user_id))
                    .await
                {
                    Ok(file) => {
                        let icon_bytes = file.bytes;
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
        #[cfg(feature = "wiki")]
        Commands::Wiki(wiki_args) => match wiki_args.command {
            WikiCommands::ListAttachments { wiki_id } => {
                println!("Listing attachments for wiki ID: {}", wiki_id);

                match client
                    .wiki()
                    .get_wiki_attachment_list(backlog_wiki::GetWikiAttachmentListParams::new(
                        WikiId::new(wiki_id),
                    ))
                    .await
                {
                    Ok(attachments) => {
                        if attachments.is_empty() {
                            println!("No attachments found for this wiki page");
                        } else {
                            println!("Found {} attachment(s):", attachments.len());
                            for attachment in attachments {
                                println!(
                                    "[{}] {} ({} bytes)",
                                    attachment.id.value(),
                                    attachment.name,
                                    attachment.size
                                );
                                println!(
                                    "  Created by: {} at {}",
                                    attachment.created_user.name,
                                    attachment.created.format("%Y-%m-%d %H:%M:%S")
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to list wiki attachments: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            WikiCommands::DownloadAttachment {
                wiki_id,
                attachment_id,
                output,
            } => {
                println!(
                    "Downloading attachment {} from wiki ID: {}",
                    attachment_id, wiki_id
                );

                match client
                    .wiki()
                    .download_wiki_attachment(backlog_wiki::DownloadWikiAttachmentParams::new(
                        WikiId::new(wiki_id),
                        WikiAttachmentId::new(attachment_id),
                    ))
                    .await
                {
                    Ok(downloaded_file) => {
                        let filename = output.unwrap_or(downloaded_file.filename.clone());

                        match tokio::fs::write(&filename, &downloaded_file.bytes).await {
                            Ok(_) => {
                                println!("✅ Successfully downloaded to: {}", filename);
                                println!("   Content-Type: {}", downloaded_file.content_type);
                                println!("   File size: {} bytes", downloaded_file.bytes.len());
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to write file '{}': {}", filename, e);
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to download wiki attachment: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(feature = "wiki_writable")]
            WikiCommands::Update {
                wiki_id,
                name,
                content,
                mail_notify,
            } => {
                println!("Updating wiki ID: {}", wiki_id);

                // Create params with provided options
                let mut params = UpdateWikiParams::new(WikiId::new(wiki_id));

                if let Some(name) = name {
                    params = params.name(name);
                }

                if let Some(content) = content {
                    params = params.content(content);
                }

                if let Some(mail_notify) = mail_notify {
                    params = params.mail_notify(mail_notify);
                }

                match client.wiki().update_wiki(params).await {
                    Ok(wiki_detail) => {
                        println!("✅ Wiki updated successfully");
                        println!("ID: {}", wiki_detail.id.value());
                        println!("Name: {}", wiki_detail.name);
                        println!("Project ID: {}", wiki_detail.project_id.value());
                        println!("Updated by: {}", wiki_detail.updated_user.name);
                        println!(
                            "Updated at: {}",
                            wiki_detail.updated.format("%Y-%m-%d %H:%M:%S")
                        );

                        if !wiki_detail.tags.is_empty() {
                            let tag_names: Vec<String> = wiki_detail
                                .tags
                                .iter()
                                .map(|tag| tag.name.clone())
                                .collect();
                            println!("Tags: {}", tag_names.join(", "));
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to update wiki: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },
    }

    Ok(())
}
