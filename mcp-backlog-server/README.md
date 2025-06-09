# Backlog MCP Server (`mcp-backlog-server`)

`mcp-backlog-server` is a Model Context Protocol (MCP) server for interacting with the Backlog API.
This server allows MCP-compatible clients (such as AI assistants) to utilize Backlog functionalities.

## Available Tools

The following tools are grouped by their respective modules:

### Document Tools
-   **`get_document_details`**
    -   Description: Retrieves details for a specific Backlog document. This includes the title, body in Markdown format (`plain`), body in ProseMirror JSON format (`json`), and other metadata. The `plain` (Markdown) content may contain image tags like `![alt_text](/document/backend/PROJ/DOCUMENT_ID_HEX/file/ATTACHMENT_ID)`. The `ATTACHMENT_ID` from such tags can be used with the `download_document_attachment_image` tool. The `alt_text` can often serve as a filename hint if needed by the client.
    -   Input: `document_id` (a 32-digit hexadecimal string)
-   **`download_document_attachment_image`**
    -   Description: Download a document attachment if it is an image. Returns the image content (base64 encoded) and its actual MIME type from the server. Returns an error if the attachment is not an image based on its `Content-Type` header.
    -   Input:
        -   `document_id` (string, required): The document ID (a 32-digit hexadecimal string).
        -   `attachment_id` (number, required): The numeric ID of the attachment to download (obtained from Markdown links in document content).
    -   Output: Image content via `rmcp::model::Content::image`.
-   **`get_document_tree`**
    -   Description: Get the document tree for a specified project. The tree structure includes document ID, name, update timestamp, emoji, and children nodes.
    -   Input: `project_id_or_key` (string, required): The project ID or project key. Examples: "MYPROJECTKEY", "123".
    -   Output: A JSON object representing the `DocumentTreeResponse`, containing `projectId`, `activeTree`, and `trashTree`. Each tree is a recursive `DocumentTreeNode` structure.

### Git Tools
-   **`get_repository_list`**
    -   Description: Get a list of Git repositories for a specified project.
    -   Input: `project_id_or_key` (Project ID or project key)
-   **`get_repository_details`**
    -   Description: Get details for a specific Git repository.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
-   **`get_pull_request_list`**
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
-   **`get_pull_request_attachment_list`**
    -   Description: Get a list of attachments for a specific pull request.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
        -   `pr_number` (Pull request number)
-   **`get_pull_request_comment_list`**
    -   Description: Get a list of comments for a specific pull request.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
        -   `pr_number` (Pull request number)
        -   `min_id` (Optional `u32`): Filter comments by minimum ID.
        -   `max_id` (Optional `u32`): Filter comments by maximum ID.
        -   `count` (Optional `u8`): Number of comments to retrieve (1-100).
        -   `order` (Optional `string`): Sort order: "asc" or "desc".
-   **`download_pull_request_attachment_raw`**
    -   Description: Download a pull request attachment as raw bytes. Returns a JSON object with filename, MIME type, and base64-encoded content.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
        -   `pr_number` (Pull request number)
        -   `attachment_id` (Numeric ID of the attachment)
-   **`download_pull_request_attachment_image`**
    -   Description: Download a pull request attachment image. Returns filename and image content as base64. Returns an error if the attachment is not an image.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
        -   `pr_number` (Pull request number)
        -   `attachment_id` (Numeric ID of the attachment)
    -   Output: Image content via `rmcp::model::Content::image`.
-   **`download_pull_request_attachment_text`**
    -   Description: Download a pull request attachment if it is a valid UTF-8 text file. Returns the text content. Returns an error if the attachment is not a valid UTF-8 text file.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
        -   `pr_number` (Pull request number)
        -   `attachment_id` (Numeric ID of the attachment)
    -   Output: Text content via `rmcp::model::Content::text`.

### Issue Tools
-   **`get_issue_details`**
    -   Description: Retrieves details for a specific Backlog issue.
    -   Input: `issue_key` (e.g., "PROJECT-123")
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
-   **`download_issue_attachment_raw`**
    -   Description: Download an issue attachment as raw bytes. Returns a JSON object with filename, MIME type, and base64-encoded content.
    -   Input:
        -   `issue_id_or_key` (string, required): The issue ID or issue key. Examples: "MYPROJECTKEY-123", "12345".
        -   `attachment_id` (number, required): The numeric ID of the attachment to download.
-   **`download_issue_attachment_image`**
    -   Description: Download an issue attachment if it is an image. Returns the image content (base64 encoded) and its MIME type. Returns an error if the attachment is not an image.
    -   Input:
        -   `issue_id_or_key` (string, required): The issue ID or issue key. Examples: "MYPROJECTKEY-123", "12345".
        -   `attachment_id` (number, required): The numeric ID of the attachment to download.
    -   Output: Image content via `rmcp::model::Content::image`.
-   **`download_issue_attachment_text`**
    -   Description: Download an issue attachment if it is a valid UTF-8 text file. Returns the text content. Returns an error if the attachment is not a valid UTF-8 text file.
    -   Input:
        -   `issue_id_or_key` (string, required): The issue ID or issue key. Examples: "MYPROJECTKEY-123", "12345".
        -   `attachment_id` (number, required): The numeric ID of the attachment to download.
    -   Output: Text content via `rmcp::model::Content::text`.

### Project Tools
-   **`get_project_status_list`**
    -   Description: Get a list of statuses for a specified project.
    -   Input: `project_id_or_key` (Project ID or project key)

### User Tools
-   **`get_user_list`**
    -   Description: Get a list of users in the space.
    -   Input: (No parameters)

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
      "autoApprove": [],
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
