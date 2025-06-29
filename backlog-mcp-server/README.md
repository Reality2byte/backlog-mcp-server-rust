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
        "BACKLOG_API_KEY": "YOUR_BACKLOG_API_KEY"
      },
      "transportType": "stdio"
    }
  }
}
```

Note: make sure your domain name: `backlog.com`, `backlog.jp` or `backlogtool.com`

## Available Tools

The following tools are grouped by their respective modules:

### Tool Summary

With the default configuration, you'll have access to **34 tools** for comprehensive Backlog automation:

- **Documents** (3 tools): View document trees, get details, download attachments
- **Git/Pull Requests** (8 tools): Manage repositories, PRs, comments, and attachments
- **Issues** (12 tools): View, create, update issues, manage comments, attachments, shared files, and priorities
- **Projects** (3 tools): Get project status, issue types, and custom field definitions
- **Shared Files** (2 tools): Browse and download project shared files
- **Users** (1 tool): List space users
- **Wikis** (5 tools): Manage wiki pages, attachments, and content updates

The server includes both **read operations** for information gathering and **write operations** for taking actions, making it ideal for AI-powered Backlog automation workflows.

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
-   **`add_pull_request_comment`**
    -   Description: Add a comment to a specific pull request. Optionally notify specified users. This tool is only available if the `git_writable` feature is enabled.
    -   Input:
        -   `project_id_or_key` (string, required): The project ID or project key
        -   `repo_id_or_name` (string, required): The repository ID (as string) or repository name
        -   `pr_number` (number, required): The pull request number
        -   `content` (string, required): The content of the comment
        -   `notified_user_ids` (array of numbers, optional): List of user IDs to notify about this comment

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
    -   Description: Updates a Backlog issue including summary, description, and custom fields. This tool is only available if the `issue_writable` feature is enabled.
    -   Input:
        -   `issue_id_or_key` (Issue ID or issue key, e.g., "MYPROJECTKEY-123", "12345")
        -   `summary` (Optional: New summary)
        -   `description` (Optional: New description)
        -   `custom_fields` (object, optional): Custom field values in AI-friendly format. Each key is the custom field ID, and the value is an object containing the field value in a simplified format.
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
-   **`get_issue_shared_files`**
    -   Description: Get a list of shared files linked to a specified issue.
    -   Input: `issue_id_or_key` (string, required): The issue ID or issue key for which to retrieve shared files. Examples: "MYPROJECTKEY-123", "12345".
    -   Output: A JSON array of `SharedFile` objects with details like ID, name, directory, size, type, and metadata.
-   **`update_issue_comment`**
    -   Description: Update an existing comment on a Backlog issue. This tool is only available if the `issue_writable` feature is enabled.
    -   Input:
        -   `issue_id_or_key` (string, required): The issue ID or issue key. Examples: "MYPROJECTKEY-123", "12345".
        -   `comment_id` (number, required): The ID of the comment to update.
        -   `content` (string, required): The new content for the comment.
-   **`add_issue_to_project`**
    -   Description: Create a new issue in a Backlog project with support for custom fields. This tool is only available if the `issue_writable` feature is enabled.
    -   Input:
        -   `project_id` (string, required): The project ID or project key.
        -   `summary` (string, required): The issue title/summary.
        -   `issue_type_id` (number, required): The issue type ID.
        -   `priority_id` (number, required): The priority ID.
        -   `description` (string, optional): The issue description.
        -   `start_date` (string, optional): The start date (format: YYYY-MM-DD).
        -   `due_date` (string, optional): The due date (format: YYYY-MM-DD).
        -   `assignee_id` (number, optional): The assignee user ID.
        -   `category_ids` (array of numbers, optional): List of category IDs.
        -   `version_ids` (array of numbers, optional): List of version/milestone IDs.
        -   `milestone_ids` (array of numbers, optional): List of milestone IDs.
        -   `parent_issue_id` (number, optional): The parent issue ID.
        -   `notified_user_ids` (array of numbers, optional): List of user IDs to notify.
        -   `custom_fields` (object, optional): Custom field values in AI-friendly format. Each key is the custom field ID, and the value is an object containing the field value in a simplified format.
-   **`add_comment_to_issue`**
    -   Description: Add a comment to a specific issue. This tool is only available if the `issue_writable` feature is enabled.
    -   Input:
        -   `issue_id_or_key` (string, required): The issue ID or issue key.
        -   `content` (string, required): The content of the comment.
        -   `notified_user_ids` (array of numbers, optional): List of user IDs to notify about this comment.
-   **`get_priorities`**
    -   Description: Get a list of priority types available in the space.
    -   Input: (No parameters)

### Project Tools
-   **`get_project_status_list`**
    -   Description: Get a list of statuses for a specified project.
    -   Input: `project_id_or_key` (Project ID or project key)
-   **`get_project_issue_types`**
    -   Description: Get a list of issue types for a specified project.
    -   Input: `project_id_or_key` (Project ID or project key)
-   **`get_custom_field_list`**
    -   Description: Get a list of custom fields defined for a specified project. Returns details about each custom field including its ID, name, type, and available options.
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

### Wiki Tools
-   **`get_wiki_list`**
    -   Description: Get a list of wiki pages. Can be filtered by project and keyword.
    -   Input:
        -   `project_id_or_key` (string, optional): The project ID or project key to filter wikis by. Examples: "MYPROJECTKEY", "123".
        -   `keyword` (string, optional): Filter wiki pages by keyword in title or content.
-   **`get_wiki_detail`**
    -   Description: Get detailed information about a specific wiki page including content, attachments, shared files, and stars.
    -   Input: `wiki_id` (number, required): Wiki page ID to retrieve details for. Must be a positive integer.
-   **`get_wiki_attachment_list`**
    -   Description: Get a list of attachments for a specified wiki page.
    -   Input: `wiki_id` (number, required): Wiki page ID to retrieve attachments for. Must be a positive integer.
-   **`download_wiki_attachment`**
    -   Description: Download an attachment from a wiki page. Automatically detects format (image, text, or raw bytes) or you can specify the format parameter. Returns the file content in the appropriate format.
    -   Input:
        -   `wiki_id` (number, required): Wiki page ID to download attachment from. Must be a positive integer.
        -   `attachment_id` (number, required): Attachment ID to download. Must be a positive integer.
        -   `format` (string, optional): Format specification - 'image', 'text', or 'raw'. If not specified, format will be auto-detected.
    -   Output: Content via `rmcp::model::Content::image` for images, `rmcp::model::Content::text` for text files, or JSON with base64-encoded content for raw bytes.
-   **`update_wiki`**
    -   Description: Update a wiki page. You can update the page name, content, and/or email notification settings. This tool is only available if the `wiki_writable` feature is enabled.
    -   Input:
        -   `wiki_id` (number, required): Wiki page ID to update. Must be a positive integer.
        -   `name` (string, optional): Optional new page name.
        -   `content` (string, optional): Optional new page content.
        -   `mail_notify` (boolean, optional): Optional whether to send email notification of update.

## File Download Features

All file download tools (`download_document_attachment`, `download_issue_attachment`, `download_pull_request_attachment`, `download_wiki_attachment`, and `download_shared_file`) support intelligent format detection and handling:

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
# Default run with all features (recommended)
BACKLOG_BASE_URL="https://your-space.backlog.com" \
BACKLOG_API_KEY="your_backlog_api_key" \
cargo run --package mcp-backlog-server
```
