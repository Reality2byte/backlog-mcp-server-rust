# Backlog API Client CLI (`blg`)

`blg` is a command-line interface (CLI) tool for interacting with the Backlog API. It is built using the `backlog-api-client` Rust library.

## Building `blg`

### Prerequisites

-   Rust and Cargo: Ensure you have a recent version of Rust and Cargo installed. You can get them from [rustup.rs](https://rustup.rs/).

### Build Command

To build the `blg` executable, navigate to the workspace root (`/Users/mac/src/_mydev/backlog-api-client`) and run:

```bash
cargo build --package blg --features "git issue project space"
```

For write operations (creating/updating/deleting resources), add the corresponding writable features:

```bash
cargo build --package blg --features "git git_writable issue issue_writable project project_writable space wiki wiki_writable"
```

Alternatively, if you are in the `cli` directory:

```bash
cargo build --features "git issue project space" 
# Or with writable features:
cargo build --features "git git_writable issue issue_writable project project_writable space wiki wiki_writable" 
```

The `git`, `issue`, `project`, `space`, and `wiki` features are required to build the `blg` binary with functionality. Add `project_writable` for project management operations like category creation/deletion, `issue_writable` for issue operations like linking shared files, `git_writable` for pull request update operations, and `wiki_writable` for wiki page update operations. The executable will be located at `target/debug/blg` (or `target/release/blg` if you add `--release`).

## Configuration

`blg` requires two environment variables to be set for authentication:

-   `BACKLOG_BASE_URL`: The base URL of your Backlog space (e.g., `https://your-space.backlog.jp`).
-   `BACKLOG_API_KEY`: Your Backlog API key. You can generate one from your personal settings page in Backlog.

Example:

```bash
export BACKLOG_BASE_URL="https://yourspace.backlog.jp"
export BACKLOG_API_KEY="yourgeneratedapikey"
```

## Basic Usage

The general syntax for `blg` is:

```bash
blg <COMMAND> <SUBCOMMAND> [OPTIONS]
```

You can get help for any command or subcommand by appending `--help`.

### Examples:

**Project Management:**
```bash
# List all projects
blg project list

# Show details of a specific project
blg project show MFP

# List statuses for a project
blg project status-list MFP

# List categories for a project
blg project category-list MFP

# List custom fields for a project
blg project custom-field-list MFP

# Add a category to a project (requires project_writable feature)
blg project category-add MFP --name "New Category"

# Update a category in a project (requires project_writable feature)
blg project category-update MFP --category-id 12345 --name "Updated Category"

# Delete a category from a project (requires project_writable feature)
blg project category-delete MFP --category-id 12345

# Add an issue type to a project (requires project_writable feature)
blg project issue-type-add MFP --name "Bug Report" --color "dark-red" --template-summary "Bug: [Title]" --template-description "## Description\n\n## Steps to reproduce"

# Update an issue type in a project (requires project_writable feature)
blg project issue-type-update MFP --issue-type-id 12345 --name "Updated Bug Report" --color "red" --template-summary "Bug: [Updated Title]"

# Delete an issue type from a project (requires project_writable feature)
blg project issue-type-delete MFP --issue-type-id 12345 --substitute-issue-type-id 67890

# Available colors: red, dark-red, purple, violet, blue, teal, green, orange, pink, gray
# You can also use hex codes: #e30000, #990000, #934981, #814fbc, #2779ca, #007e9a, #7ea800, #ff9200, #ff3265, #666665
```

**Space Management:**
```bash
# Download space logo
blg space logo --output logo.png
```

**Wiki Management:**
```bash
# List attachments for a wiki page
blg wiki list-attachments 12345

# Download an attachment from a wiki page with original filename
blg wiki download-attachment 12345 67890

# Download an attachment from a wiki page with custom filename
blg wiki download-attachment 12345 67890 --output custom_name.png

# Update a wiki page name
blg wiki update 12345 --name "New Wiki Title"

# Update a wiki page content
blg wiki update 12345 --content "Updated content for the wiki page"

# Update both name and content with email notification
blg wiki update 12345 --name "Updated Title" --content "Updated content" --mail-notify true

# Update content without email notification
blg wiki update 12345 --content "Silent update" --mail-notify false
```

**Issue Management:**
```bash
# List issues for a project
blg issue list --project-id MYPROJ 

# Show details of a specific issue
blg issue show MYPROJ-101

# Add a comment to an issue
blg issue add-comment MYPROJ-101 --content "This is a comment"

# Download an issue attachment
blg issue download-attachment MYPROJ-101 12345 --output downloaded_file.dat

# List shared files linked to an issue
blg issue list-shared-files MYPROJ-101

# Link shared files to an issue (requires issue_writable feature)
blg issue link-shared-files MYPROJ-101 --file-ids 123,456,789
```

**Repository Management:**
```bash
# List repositories in a project
blg repo list --project-id MYPROJ

# Show details of a specific repository
blg repo show --project-id MYPROJ --repo-id my-repo
```

**Pull Request Management:**
```bash
# List pull requests in a repository
blg pr list --project-id MYPROJ --repo-id my-repo

# Show details of a specific pull request
blg pr show --project-id MYPROJ --repo-id my-repo --pr-number 42

# Download a pull request attachment
blg pr download-attachment -p MYPROJ -r my-repo -n 42 -a 56789 -o pr_attachment.zip

# Update a pull request (requires git_writable feature)
blg pr update -p MYPROJ -r my-repo --pr-number 42 --summary "Updated PR Title" --description "Updated description" --comment "Updated via CLI"

# Update with issue assignment and notifications
blg pr update -p MYPROJ -r my-repo --pr-number 42 --assignee-id 12345 --issue-id 67890 --notify-user-ids 111,222,333
```

### Getting Help

-   For a list of all top-level commands:
    ```bash
    blg --help
    ```
-   For help with a specific command (e.g., `project`):
    ```bash
    blg project --help
    ```
-   For help with a specific subcommand (e.g., `project list`):
    ```bash
    blg project list --help
    ```

## Available Commands

The `blg` CLI currently supports the following commands:

### Project Commands
- `project list` - List all projects in the Backlog space
- `project show <PROJECT_ID_OR_KEY>` - Show detailed information about a specific project
- `project status-list <PROJECT_ID_OR_KEY>` - List all statuses for a specific project
- `project milestone-list <PROJECT_ID_OR_KEY>` - List milestones for a specific project
- `project issue-type-list <PROJECT_ID_OR_KEY>` - List issue types for a specific project
- `project category-list <PROJECT_ID_OR_KEY>` - List categories for a specific project
- `project priority-list` - List priorities (space-wide)
- `project resolution-list` - List resolutions (space-wide)
- `project icon <PROJECT_ID_OR_KEY> --output <FILE_PATH>` - Download project icon
- `project category-add <PROJECT_ID_OR_KEY> --name <CATEGORY_NAME>` - Add a category to a project (requires `project_writable` feature)
- `project category-update <PROJECT_ID_OR_KEY> --category-id <CATEGORY_ID> --name <NEW_NAME>` - Update a category in a project (requires `project_writable` feature)
- `project category-delete <PROJECT_ID_OR_KEY> --category-id <CATEGORY_ID>` - Delete a category from a project (requires `project_writable` feature)
- `project issue-type-add <PROJECT_ID_OR_KEY> --name <ISSUE_TYPE_NAME> --color <COLOR> [--template-summary <SUMMARY>] [--template-description <DESCRIPTION>]` - Add an issue type to a project (requires `project_writable` feature). COLOR can be a name (red, dark-red, purple, violet, blue, teal, green, orange, pink, gray) or hex code.
- `project issue-type-update <PROJECT_ID_OR_KEY> --issue-type-id <ISSUE_TYPE_ID> [--name <NEW_NAME>] [--color <NEW_COLOR>] [--template-summary <NEW_SUMMARY>] [--template-description <NEW_DESCRIPTION>]` - Update an issue type in a project (requires `project_writable` feature). All parameters except issue-type-id are optional.
- `project issue-type-delete <PROJECT_ID_OR_KEY> --issue-type-id <ISSUE_TYPE_ID> --substitute-issue-type-id <SUBSTITUTE_ID>` - Delete an issue type from a project (requires `project_writable` feature). Existing issues will be moved to the substitute issue type.

### Space Commands
- `space logo --output <FILE_PATH>` - Download the space logo

### Issue Commands
- `issue list [OPTIONS]` - List issues with optional filters
- `issue show <ISSUE_ID_OR_KEY>` - Show detailed information about a specific issue
- `issue add-comment <ISSUE_ID_OR_KEY> --content <CONTENT>` - Add a comment to an issue
- `issue update-comment --issue-id <ISSUE_ID_OR_KEY> --comment-id <COMMENT_ID> --content <NEW_CONTENT>` - Update an existing comment (requires `issue_writable` feature)
- `issue delete-comment --issue-id <ISSUE_ID_OR_KEY> --comment-id <COMMENT_ID>` - Delete a comment from an issue (requires `issue_writable` feature)
- `issue download-attachment <ISSUE_ID_OR_KEY> <ATTACHMENT_ID> --output <FILE_PATH>` - Download an issue attachment
- `issue list-shared-files <ISSUE_ID_OR_KEY>` - List shared files linked to an issue
- `issue link-shared-files <ISSUE_ID_OR_KEY> --file-ids <FILE_ID1,FILE_ID2>` - Link shared files to an issue (requires `issue_writable` feature)

### Repository Commands
- `repo list --project-id <PROJECT_ID_OR_KEY>` - List repositories in a project
- `repo show --project-id <PROJECT_ID_OR_KEY> --repo-id <REPO_ID_OR_NAME>` - Show repository details

### Pull Request Commands
- `pr list --project-id <PROJECT_ID_OR_KEY> --repo-id <REPO_ID_OR_NAME>` - List pull requests in a repository
- `pr show --project-id <PROJECT_ID_OR_KEY> --repo-id <REPO_ID_OR_NAME> --pr-number <NUMBER>` - Show pull request details
- `pr download-attachment -p <PROJECT_ID> -r <REPO_ID> -n <PR_NUMBER> -a <ATTACHMENT_ID> -o <FILE_PATH>` - Download a pull request attachment
- `pr update -p <PROJECT_ID> -r <REPO_ID> --pr-number <NUMBER> [OPTIONS]` - Update a pull request (requires `git_writable` feature)
  - `--summary <TITLE>` - Update pull request title
  - `--description <DESC>` - Update pull request description  
  - `--issue-id <ID>` - Link to a related issue
  - `--assignee-id <ID>` - Assign to a user
  - `--notify-user-ids <ID1,ID2>` - Notify users (comma-separated)
  - `--comment <TEXT>` - Add a comment with the update

### User Commands
- `user list` - List all users in the space
- `user me` - Get information about the authenticated user
- `user show <USER_ID>` - Show detailed information about a specific user (requires `user` feature)
- `user icon <USER_ID> --output <FILE_PATH>` - Download user icon to a file

### Wiki Commands
- `wiki list-attachments <WIKI_ID>` - List attachments for a specific wiki page
- `wiki download-attachment <WIKI_ID> <ATTACHMENT_ID> [--output <FILE_PATH>]` - Download an attachment from a wiki page
- `wiki update <WIKI_ID> [--name <NEW_NAME>] [--content <NEW_CONTENT>] [--mail-notify <true|false>]` - Update a wiki page (requires `wiki_writable` feature)