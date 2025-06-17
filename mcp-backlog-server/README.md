# Backlog MCP Server (`mcp-backlog-server`)

`mcp-backlog-server` is a Model Context Protocol (MCP) server for interacting with the Backlog API.
This server allows MCP-compatible clients (such as AI assistants) to utilize Backlog functionalities.

## Key Features

- **Unified File Download**: All file download tools now support intelligent format detection, automatically handling images, text files, and raw binary data
- **Shared File Support**: Browse and download shared files from Backlog projects
- **Flexible Format Control**: Override automatic detection with explicit format specification
- **Comprehensive API Coverage**: Access documents, issues, pull requests, projects, users, and shared files

## Available Tools

The following tools are grouped by their respective modules:

### Document Tools
-   **`get_document_details`**
    -   Description: Retrieves details for a specific Backlog document. This includes the title, body in Markdown format (`plain`), body in ProseMirror JSON format (`json`), and other metadata. The `plain` (Markdown) content may contain image tags like `![alt_text](/document/backend/PROJ/DOCUMENT_ID_HEX/file/ATTACHMENT_ID)`. The `ATTACHMENT_ID` from such tags can be used with the `download_document_attachment` tool. The `alt_text` can often serve as a filename hint if needed by the client.
    -   Input: `document_id` (a 32-digit hexadecimal string)
-   **`download_document_attachment`**
    -   Description: Download a document attachment. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format.
    -   Input:
        -   `document_id` (string, required): The document ID (a 32-digit hexadecimal string).
        -   `attachment_id` (number, required): The numeric ID of the attachment to download (obtained from Markdown links in document content).
        -   `format` (string, optional): Format specification - 'image', 'text', or 'raw'. If not specified, format will be auto-detected.
    -   Output: Content via `rmcp::model::Content::image` for images, `rmcp::model::Content::text` for text files, or JSON with base64-encoded content for raw bytes.
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
-   **`download_pull_request_attachment`**
    -   Description: Download a pull request attachment. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format.
    -   Input:
        -   `project_id_or_key` (Project ID or project key)
        -   `repo_id_or_name` (Repository ID (as string) or repository name)
        -   `pr_number` (Pull request number)
        -   `attachment_id` (Numeric ID of the attachment)
        -   `format` (string, optional): Format specification - 'image', 'text', or 'raw'. If not specified, format will be auto-detected.
    -   Output: Content via `rmcp::model::Content::image` for images, `rmcp::model::Content::text` for text files, or JSON with base64-encoded content for raw bytes.

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
-   **`download_issue_attachment`**
    -   Description: Download an issue attachment. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format.
    -   Input:
        -   `issue_id_or_key` (string, required): The issue ID or issue key. Examples: "MYPROJECTKEY-123", "12345".
        -   `attachment_id` (number, required): The numeric ID of the attachment to download.
        -   `format` (string, optional): Format specification - 'image', 'text', or 'raw'. If not specified, format will be auto-detected.
    -   Output: Content via `rmcp::model::Content::image` for images, `rmcp::model::Content::text` for text files, or JSON with base64-encoded content for raw bytes.

### Project Tools
-   **`get_project_status_list`**
    -   Description: Get a list of statuses for a specified project.
    -   Input: `project_id_or_key` (Project ID or project key)

### Shared File Tools
-   **`get_shared_files_list`**
    -   Description: Get a list of shared files for a specified project directory.
    -   Input:
        -   `project_id_or_key` (string, required): The project ID or project key to retrieve shared files for. Examples: "MYPROJECTKEY", "123".
        -   `path` (string, required): The path to retrieve shared files from.
        -   `order` (string, optional): Sort order: "asc" or "desc".
        -   `offset` (number, optional): Offset for pagination.
        -   `count` (number, optional): Number of items to retrieve.
    -   Output: A JSON array of `SharedFile` objects with details like ID, name, size, type, and metadata.
-   **`download_shared_file`**
    -   Description: Download a shared file. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format.
    -   Input:
        -   `project_id_or_key` (string, required): The project ID or project key. Examples: "MYPROJECTKEY", "123".
        -   `shared_file_id` (number, required): The shared file ID to download.
        -   `format` (string, optional): Format specification - 'image', 'text', or 'raw'. If not specified, format will be auto-detected.
    -   Output: Content via `rmcp::model::Content::image` for images, `rmcp::model::Content::text` for text files, or JSON with base64-encoded content for raw bytes.

### User Tools
-   **`get_user_list`**
    -   Description: Get a list of users in the space.
    -   Input: (No parameters)

## File Download Features

All file download tools (`download_document_attachment`, `download_issue_attachment`, `download_pull_request_attachment`, and `download_shared_file`) support intelligent format detection and handling:

### Automatic Format Detection
- **Images**: Files with `image/*` content type are automatically detected and returned as base64-encoded images via `rmcp::model::Content::image`
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
