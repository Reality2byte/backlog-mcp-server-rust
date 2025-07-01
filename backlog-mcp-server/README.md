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

### Document Tools
-   **`get_document_details`**: Retrieves details for a specific Backlog document
-   **`download_document_attachment`**: Download a document attachment
-   **`get_document_tree`**: Get the document tree for a specified project

### Git Tools
-   **`get_repository_list`**: Get a list of Git repositories for a specified project
-   **`get_repository_details`**: Get details for a specific Git repository
-   **`get_pull_request_list`**: Get a list of pull requests for a specified repository
-   **`get_pull_request_details`**: Get details for a specific pull request
-   **`get_pull_request_attachment_list`**: Get a list of attachments for a specific pull request
-   **`get_pull_request_comment_list`**: Get a list of comments for a specific pull request
-   **`download_pull_request_attachment`**: Download a pull request attachment
-   **`add_pull_request_comment`**: Add a comment to a specific pull request

### Issue Tools
-   **`get_issue_details`**: Retrieves details for a specific Backlog issue
-   **`get_version_milestone_list`**: Retrieves a list of versions (milestones) for a specified project
-   **`get_issues_by_milestone_name`**: Retrieves a list of issues associated with a specified milestone
-   **`update_issue`**: Updates a Backlog issue including summary, description, and custom fields
-   **`get_issue_comments`**: Gets comments for a specific issue
-   **`get_issue_attachment_list`**: Get a list of attachments for a specified issue
-   **`download_issue_attachment`**: Download an issue attachment
-   **`get_issue_shared_files`**: Get a list of shared files linked to a specified issue
-   **`update_issue_comment`**: Update an existing comment on a Backlog issue
-   **`add_issue_to_project`**: Create a new issue in a Backlog project with support for custom fields
-   **`add_comment_to_issue`**: Add a comment to a specific issue
-   **`get_priorities`**: Get a list of priority types available in the space

### Project Tools
-   **`get_project_status_list`**: Get a list of statuses for a specified project
-   **`get_project_issue_types`**: Get a list of issue types for a specified project
-   **`get_custom_field_list`**: Get a list of custom fields defined for a specified project

### Shared File Tools
-   **`get_shared_files_list`**: Get a list of shared files for a specified project directory
-   **`download_shared_file`**: Download a shared file

### User Tools
-   **`get_user_list`**: Get a list of users in the space

### Wiki Tools
-   **`get_wiki_list`**: Get a list of wiki pages
-   **`get_wiki_detail`**: Get detailed information about a specific wiki page
-   **`get_wiki_attachment_list`**: Get a list of attachments for a specified wiki page
-   **`download_wiki_attachment`**: Download an attachment from a wiki page
-   **`update_wiki`**: Update a wiki page

## File Download Features

All file download tools (`download_document_attachment`, `download_issue_attachment`, `download_pull_request_attachment`, `download_wiki_attachment`, and `download_shared_file`) support format detection and handling:

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
    -   Enables: `update_issue`, `update_issue_comment`, `add_issue_to_project`, and `add_comment_to_issue` tools
    -   Allows AI agents to create issues, modify issue content, and manage comments

-   **`git_writable`** (enabled by default)
    -   Enables: `add_pull_request_comment` tool
    -   Allows AI agents to add comments to pull requests

-   **`wiki_writable`** (enabled by default)
    -   Enables: `update_wiki` tool
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