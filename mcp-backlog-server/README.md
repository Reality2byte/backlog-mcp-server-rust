# Backlog MCP Server (`mcp-backlog-server`)

`mcp-backlog-server` is a Model Context Protocol (MCP) server for interacting with the Backlog API.
This server allows MCP-compatible clients (such as AI assistants) to utilize Backlog functionalities.

## Available Tools

The following tools are currently available:

-   **`get_issue_details`**
    -   Description: Retrieves details for a specific Backlog issue.
    -   Input: `issue_key` (e.g., "PROJECT-123")
-   **`get_document_details`**
    -   Description: Retrieves details for a specific Backlog document. This includes the title, body in Markdown format (`plain`), body in ProseMirror JSON format (`json`), and other metadata.
    -   Input: `document_id` (a 32-digit hexadecimal string)
-   **`get_version_milestone_list`**
    -   Description: Retrieves a list of versions (milestones) for a specified project.
    -   Input: `project_id_or_key` (Project ID or project key, e.g., "MYPROJECTKEY", "123")
-   **`get_issues_by_milestone_name`**
    -   Description: Retrieves a list of issues associated with a specified milestone name within a project.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `milestone_name` (Milestone name)
-   **`update_issue`**
    -   Description: Updates the summary (title) and/or description of a Backlog issue. This tool is only available if the `issue_writable` feature is enabled.
    -   Input:
        -   `issue_id_or_key` (Issue ID or issue key, e.g., "MYPROJECTKEY-123", "12345")
        -   `summary` (Optional: New summary)
        -   `description` (Optional: New description)
-   **`get_issue_comments`**
    -   Description: Gets comments for a specific issue.
    -   Input:
        -   `issue_id_or_key` (string, required): The issue ID or issue key.
        -   `min_id` (Optional `u64`): Filter comments by minimum ID.
        -   `max_id` (Optional `u64`): Filter comments by maximum ID.
        -   `count` (Optional `u8`): Number of comments to retrieve (1-100).
        -   `order` (Optional `string`): Sort order: "asc" or "desc".
-   **`get_issue_attachment_list`**
    -   Description: Get a list of attachments for a specified issue.
    -   Input: `issue_id_or_key` (Issue ID or issue key, e.g., "MYPROJECTKEY-123", "12345")
-   **`get_repository_list`**
    -   Description: Get a list of Git repositories for a specified project.
    -   Input: `project_id_or_key` (Project ID or project key)
-   **`get_repository_details`**
    -   Description: Get details for a specific Git repository.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
-   **`list_pull_requests`**
    -   Description: Get a list of pull requests for a specified repository.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
-   **`get_pull_request_details`**
    -   Description: Get details for a specific pull request.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
        -   `pr_number` (Pull request number)
-   **`get_project_status_list`**
    -   Description: Get a list of statuses for a specified project.
    -   Input: `projectIdOrKey` (Project ID or project key)

## Feature Flags

-   **`issue_writable`**
    -   Enabling this feature flag makes the `update_issue` tool available.
    -   It is enabled by default.
    -   To control features during build:
        -   Disable: `cargo build --no-default-features`
        -   Enable explicitly: `cargo build --features issue_writable`

## Configuration

To run this server, the following environment variables must be set:

-   `BACKLOG_BASE_URL`: The URL of your Backlog space (e.g., `https://your-space.backlog.jp`)
-   `BACKLOG_API_KEY`: Your Backlog API key. You can issue one from your personal settings page in Backlog.

These environment variables are expected to be passed by the MCP client system when launching the server.

## Building and Running

### Build

Run the following command in the project root directory:

```bash
cargo build --package mcp-backlog-server
```

To build with specific features:

```bash
cargo build --package mcp-backlog-server --no-default-features
cargo build --package mcp-backlog-server --features issue_writable
```

### Run (for local testing)

After setting the environment variables, you can run the server directly with the following command:

```bash
BACKLOG_BASE_URL="your_backlog_base_url" \
BACKLOG_API_KEY="your_backlog_api_key" \
cargo run --package mcp-backlog-server
```

The server will listen for MCP client requests on standard input/output.

## Example Configuration for MCP server
```
  "mcpServers": {
    "backlog_mcp_server": {
      "autoApprove": [
        "get_document_details",
        "get_issue_details"
      ],
      "disabled": false,
      "timeout": 60,
      "command": "/path/to/mcp-backlog-server",
      "args": [],
      "env": {
        "BACKLOG_BASE_URL": "https://example.backlog.com",
        "BACKLOG_API_KEY": "YOUR_API_KEY"
      },
      "transportType": "stdio"
    }
  }
