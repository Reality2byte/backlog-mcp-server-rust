# Backlog MCP Server (`mcp-backlog-server`)

`mcp-backlog-server` is a Model Context Protocol (MCP) server for interacting with the Backlog API.
This server allows MCP-compatible clients (such as AI assistants) to utilize Backlog functionalities.

## Example Configuration for MCP Client

### Claude Desktop Configuration

Add the following to your Claude Desktop MCP configuration:

```json
{
  "mcpServers": {
    "backlog_mcp_server": {
      "autoApprove": [],
      "disabled": false,
      "timeout": 60,
      "command": "/path/to/target/release/mcp-backlog-server",
      "args": [],
      "env": {
        "BACKLOG_BASE_URL": "https://your-space.backlog.com",
        "BACKLOG_API_KEY": "YOUR_BACKLOG_API_KEY",
        "BACKLOG_PROJECTS": "PROJ,DEMO"
      },
      "transportType": "stdio"
    }
  }
}
```

### Gemini CLI

`~/.gemini/settings.json`:
```

{
  "mcpServers": {
    "backlog_mcp_server": {
      "command": "/path/to/target/release/mcp-backlog-server",
      "timeout": 10000,
      "args": [],
      "env": {
        "BACKLOG_BASE_URL": "https://your-space.backlog.com",
        "BACKLOG_API_KEY": "YOUR_BACKLOG_API_KEY",
        "BACKLOG_PROJECTS": "PROJ,DEMO"
      }
    }
  }
}
```

Note: Domain name must be: `backlog.com`, `backlog.jp` or `backlogtool.com`

## Available Tools

The following tools are grouped by their respective modules:

### Tool Summary

With the default configuration, you have access to **34 tools** for Backlog automation:

- **Documents** (3 tools): View document trees, get details, download attachments
- **Git/Pull Requests** (8 tools): Manage repositories, PRs, comments, and attachments
- **Issues** (12 tools): View, create, update issues, manage comments, attachments, shared files, and priorities
- **Projects** (3 tools): Get project status, issue types, and custom field definitions
- **Shared Files** (2 tools): Browse and download project shared files
- **Users** (1 tool): List space users
- **Wikis** (5 tools): Manage wiki pages, attachments, and content updates

The server includes both **read operations** for information gathering and **write operations** for taking actions.

**Note**: Tool names follow a `category_resource_action` pattern (e.g., `issue_details_get`, `wiki_update`) to enable category-based filtering with `--allowedTools` (e.g., `claude --allowedTools "mcp__backlog__issue_*"`).

### Document Tools
-   **`document_details_get`**: Retrieves details for a specific Backlog document
-   **`document_attachment_download`**: Download a document attachment
-   **`document_tree_get`**: Get the document tree for a specified project

### Git Tools
-   **`git_repository_list_get`**: Get a list of Git repositories for a specified project
-   **`git_repository_details_get`**: Get details for a specific Git repository
-   **`git_pr_list_get`**: Get a list of pull requests for a specified repository
-   **`git_pr_details_get`**: Get details for a specific pull request
-   **`git_pr_attachment_list_get`**: Get a list of attachments for a specific pull request
-   **`git_pr_comment_list_get`**: Get a list of comments for a specific pull request
-   **`git_pr_attachment_download`**: Download a pull request attachment
-   **`git_pr_comment_add`**: Add a comment to a specific pull request

### Issue Tools
-   **`issue_details_get`**: Retrieves details for a specific Backlog issue
-   **`issue_milestone_list_get`**: Retrieves a list of versions (milestones) for a specified project
-   **`issue_list_by_milestone_get`**: Retrieves a list of issues associated with a specified milestone
-   **`issue_update`**: Updates a Backlog issue including summary, description, and custom fields
-   **`issue_comment_list_get`**: Gets comments for a specific issue
-   **`issue_attachment_list_get`**: Get a list of attachments for a specified issue
-   **`issue_attachment_download`**: Download an issue attachment
-   **`issue_shared_file_list_get`**: Get a list of shared files linked to a specified issue
-   **`issue_comment_update`**: Update an existing comment on a Backlog issue
-   **`issue_add`**: Create a new issue in a Backlog project with support for custom fields
-   **`issue_comment_add`**: Add a comment to a specific issue
-   **`issue_priority_list_get`**: Get a list of priority types available in the space

### Project Tools
-   **`project_status_list_get`**: Get a list of statuses for a specified project
-   **`project_issue_type_list_get`**: Get a list of issue types for a specified project
-   **`project_custom_field_list_get`**: Get a list of custom fields defined for a specified project

### Shared File Tools
-   **`file_shared_list_get`**: Get a list of shared files for a specified project directory
-   **`file_shared_download`**: Download a shared file

### User Tools
-   **`user_list_get`**: Get a list of users in the space

### Activity Tools (Not yet implemented in MCP server)
-   **`activity_details_get`**: Get details of a specific activity by ID
-   **`activity_project_list_get`**: Get recent activities in a project
-   **`activity_space_list_get`**: Get recent activities across the entire space
-   **`activity_user_list_get`**: Get recent activities by a specific user

### Wiki Tools
-   **`wiki_list_get`**: Get a list of wiki pages
-   **`wiki_details_get`**: Get detailed information about a specific wiki page
-   **`wiki_attachment_list_get`**: Get a list of attachments for a specified wiki page
-   **`wiki_attachment_download`**: Download an attachment from a wiki page
-   **`wiki_update`**: Update a wiki page

## File Download Features

All file download tools (`document_attachment_download`, `issue_attachment_download`, `git_pr_attachment_download`, `wiki_attachment_download`, and `file_shared_download`) support format detection and handling:

### Format Detection
- **Images**: Files with `image/*` content type are detected and returned as base64-encoded images via `rmcp::model::Content::image`
- **Text**: Files with text-based content types (`text/*`, `application/json`, `application/xml`, etc.) or files that contain valid UTF-8 text are returned as plain text via `rmcp::model::Content::text`
- **Raw bytes**: All other files are returned as JSON objects with base64-encoded content, filename, and MIME type

### Manual Format Override
You can explicitly specify the format using the optional `format` parameter:
- `"image"`: Force treatment as an image (validates content type)
- `"text"`: Force treatment as text (validates UTF-8 encoding)
- `"raw"`: Force treatment as raw bytes (no validation)

### Content Type Detection
The system uses multiple strategies to determine if a file is text:
- Content-Type header analysis
- UTF-8 validity checking
- Character composition analysis (graphic, whitespace, and valid UTF-8 characters)

## How to Build

```bash
# Default build (includes all writable features)
cargo build --package mcp-backlog-server
```

### Feature Flags

The MCP server supports multiple feature flags to enable different write operations:

-   **`issue_writable`** (enabled by default)
    -   Enables: `issue_update`, `issue_comment_update`, `issue_add`, and `issue_comment_add` tools
    -   Allows AI agents to create issues, modify issue content, and manage comments

-   **`git_writable`** (enabled by default)
    -   Enables: `git_pr_comment_add` tool
    -   Allows AI agents to add comments to pull requests

-   **`wiki_writable`** (enabled by default)
    -   Enables: `wiki_update` tool
    -   Allows AI agents to update wiki page content, names, and notification settings

### Build Configuration

```bash
# Read-only mode (no write operations)
cargo build --package mcp-backlog-server --no-default-features

# Selective features
cargo build --package mcp-backlog-server --features issue_writable
cargo build --package mcp-backlog-server --features "issue_writable,git_writable"
cargo build --package mcp-backlog-server --features "issue_writable,git_writable,wiki_writable"
```

## Configuration

To run this server, the following environment variables must be set:

-   `BACKLOG_BASE_URL`: The URL of your Backlog space (e.g., `https://your-space.backlog.com`)
-   `BACKLOG_API_KEY`: Your Backlog API key. You can issue one from your personal settings page in Backlog.

Optional environment variables:

-   `BACKLOG_PROJECTS`: Comma-separated list of allowed project keys (e.g., `MFP,DEMO,TEST`). When set, the server will only allow access to the specified projects. If not set, all projects accessible with the API key are available.

These environment variables are expected to be passed by the MCP client system when launching the server.

### Run (for local testing)

After setting the environment variables, you can run the server directly with the following command:

```bash
# Default run with all features
BACKLOG_BASE_URL="https://your-space.backlog.com" \
BACKLOG_API_KEY="your_backlog_api_key" \
cargo run --package mcp-backlog-server
```

## CLI Tool (`blg`)

The `blg` command-line tool provides direct access to Backlog API functionality.

### Installation

```bash
# Install with all features
cargo install --path cli

# Install with specific features
cargo install --path cli --features "issue project space rate-limit"
```

### Configuration

Set the following environment variables:

```bash
export BACKLOG_BASE_URL="https://your-space.backlog.com"
export BACKLOG_API_KEY="your_api_key"
```

### Example Commands

```bash
# View rate limit information
blg rate-limit get

# List issues
blg issue list --project-id PROJECT_KEY

# Show issue details
blg issue show --issue-id-or-key ISSUE-123

# Create an issue
blg issue add --project-id PROJECT_ID --issue-type-id 1 --summary "New Issue" --priority-id 3

# Add a comment to an issue
blg issue comment-add --issue-id-or-key ISSUE-123 --content "This is a comment"

# List projects
blg project list

# View project details
blg project show --project-id-or-key PROJECT_KEY

# Get space information
blg space info

# List teams
blg team list

# View team details
blg team get --team-id 123
```

### Rate Limit Command

The rate limit command displays current API usage information:

```bash
$ blg rate-limit get

Rate Limit Information:
======================

Read Operations:
  Limit:     600
  Remaining: 598
  Reset:     2024-01-15 12:30:00 JST

Update Operations:
  Limit:     150
  Remaining: 149
  Reset:     2024-01-15 12:30:00 JST

Search Operations:
  Limit:     150
  Remaining: 150
  Reset:     2024-01-15 12:30:00 JST

Icon Operations:
  Limit:     60
  Remaining: 59
  Reset:     2024-01-15 12:30:00 JST
```
