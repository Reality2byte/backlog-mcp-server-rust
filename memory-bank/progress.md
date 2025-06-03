# Progress

## Current Status
-   **Memory Bank Initialized**: All six core Memory Bank files (`projectbrief.md`, `productContext.md`, `activeContext.md`, `systemPatterns.md`, `techContext.md`, `progress.md`) have been created and populated with initial content derived from analyzing the existing project structure.
-   **Project Structure Analyzed**: The file structure of the main workspace and its member crates has been listed.
-   **Dependencies Identified**: Key dependencies have been identified from the workspace `Cargo.toml`.
-   **High-Level Code Structure Understood**: An initial understanding of the roles of different crates and some key public structs/methods has been gained.
-   **`mcp-backlog-server` Crate Exists**: A dedicated crate for the Backlog MCP server (`mcp-backlog-server`) with tools like `get_issue_details` and `get_document_details` has been found to be already implemented.
-   **`get_version_milestone_list` API Implemented**: The `get_version_milestone_list` API has been successfully implemented in the `backlog-issue` crate, including model updates and tests.
-   **`get_version_milestone_list` MCP Tool Implemented**: The `get_version_milestone_list` MCP tool has been successfully implemented in the `mcp-backlog-server` crate.
-   **`get_issues_by_milestone_name` MCP Tool Implemented**: The `get_issues_by_milestone_name` MCP tool has been successfully implemented in the `mcp-backlog-server` crate.
-   **Improved Error Messaging**: The error message for `MilestoneNotFoundByName` in `mcp-backlog-server` has been improved to suggest using `get_version_milestone_list`.
-   **Suggestion Feature for MilestoneNotFoundByName**: Implemented a feature in `mcp-backlog-server` to provide suggestions of similar milestone names if an exact match is not found. This uses preprocessing and Levenshtein distance.
-   **`update_issue` MCP Tool Implemented**: The `update_issue` MCP tool has been implemented in `mcp-backlog-server`, allowing updates to issue summary and description. This tool is available when the `issue_writable` feature is enabled.
-   **Code Quality Improved**: Addressed Clippy warnings and formatted the entire project codebase.
-   **Git and Pull Request Viewing Implemented**: Added functionality to view Git repositories and pull requests.
    -   New `backlog-git` crate created with data models (`Repository`, `PullRequest`, etc.) and API handlers (`GitHandler`).
    -   `backlog-api-client` library extended with `git()` accessor.
    -   `blg` CLI tool enhanced with `repo` and `pr` subcommands (`list`, `show`).
    -   `mcp-backlog-server` extended with tools: `get_repository_list`, `get_repository_details`, `list_pull_requests`, `get_pull_request_details`.
    -   Dependent crates (`backlog-core`) updated for `JsonSchema` support.
-   **CLI Issue Commands Implemented**: Added `issue list` and `issue show` commands to the `blg` CLI tool.
-   **`mcp-backlog-server` Refactored**: Tool helper modules (`document.rs`, `issue.rs`, `git_tools.rs` renamed to `git.rs`) moved into a new `src/tools/` subdirectory for better organization.
-   **Comprehensive Error Handling & Dependency Refactoring ("100-Point Plan")**:
    -   Unified `backlog_api_core::Error` (`ApiError`) to wrap `backlog_core::Error` (validation errors), making `ApiError` the primary error type for `backlog-api-client`.
    -   Standardized API handlers in sub-crates to return `Result<_, ApiError>`.
    -   Refactored `mcp-backlog-server`'s error handling to align with the unified `ApiError`.
    -   Expanded type re-exports from `backlog-api-client` (e.g., `BacklogCoreError`, request builders, specific ID types).
    -   Updated `mcp-backlog-server` and `blg` CLI to use these re-exported types.
    -   Removed direct dependencies of `mcp-backlog-server` on `backlog-core`, `backlog-issue`, `backlog-document`, and `backlog-git` in its `Cargo.toml`.
    -   Verified all changes with `cargo check`, `test`, `clippy`, and `fmt`.
-   **`get_comment_list` API Implemented**: The `get_comment_list` API has been successfully implemented in the `backlog-issue` crate, including models, request parameters, API client method, tests, and documentation. This also involved several workspace-wide fixes for dependencies (`schemars`), type definitions, and error handling in `mcp-backlog-server` to ensure all tests pass.
-   **`get_issue_comments` MCP Tool Implemented**: The `get_issue_comments` tool has been successfully implemented in `mcp-backlog-server`, allowing retrieval of comments for a specific issue. This included defining request structs, bridge logic, and server registration. Verified with `cargo build`, `clippy`, and `test`.
-   **Error Handling Enhancements**: Improved error handling in `client` crate and `mcp-backlog-server`.
    -   `backlog-api-core::Error` now includes `HttpStatus` variant to hold structured Backlog API errors.
    -   `client::Client` updated to parse Backlog API error responses for non-2xx statuses.
    -   `mcp-backlog-server` error conversion updated for clearer messages.

## What Works
-   The Memory Bank system is established and updated.
-   A baseline understanding of the project's architecture, technology stack, and purpose is documented.
-   **Unified Error Handling**: The `backlog-api-client` library now provides a more consistent error handling experience, with `backlog_api_core::Error` (`ApiError`) serving as the central error type. It now better captures structured API errors (via `HttpStatus` variant) from non-2xx responses, in addition to wrapping HTTP, JSON, URL, and core type validation errors.
-   **Improved API Facade**: `backlog-api-client` acts as a stronger facade, re-exporting a comprehensive set of types.
-   **Simplified Consumer Dependencies**: `mcp-backlog-server` now depends primarily on `backlog-api-client`.
-   The `backlog-api-client` library provides core functionality for Backlog API interaction, including:
    -   Git repository listing and details.
    -   Pull request listing and details.
    -   Issue listing, details, updates, and **comment listing**.
    -   Document listing and details.
    -   Project listing and details.
    -   Space details.
    -   User details.
-   The `blg` CLI tool provides commands for various operations.
-   The `mcp-backlog-server` provides a suite of MCP tools, including:
    -   `get_issue_details`, `get_document_details`, `get_version_milestone_list`, `get_issues_by_milestone_name`, `update_issue`, `get_repository_list`, `get_repository_details`, `list_pull_requests`, `get_pull_request_details`, `get_issue_comments`.
    -   Error reporting from these tools to MCP clients is now more informative.
-   The codebase is free of Clippy warnings and consistently formatted. All tests pass.

## What's Left to Build (for this task)
-   Error handling improvements are complete.
-   Potential next steps, if requested:
    -   Integrate `get_issue_comments` (or `get_comment_list` library function) into the `blg` CLI tool.
-   Complete full definitions for stubbed request parameter structs in `backlog-issue/src/requests/mod.rs`.

## Known Issues (from initialization process and ongoing work)
-   The `list_code_definition_names` tool did not find top-level definitions in the `src` directories of several module-specific crates.
-   The `download_attachment` method in `backlog-document/src/api.rs` is a placeholder.

## Evolution of Project Decisions
-   **Initial Project Setup**: Focused on creating the Backlog API client library and CLI.
-   (Previous items omitted for brevity - assume they are still relevant)
-   **`get_comment_list` Library Implementation**: User requested to implement issue comment retrieval.
    -   Defined `Comment` and related models in `backlog-issue`.
    -   Defined `GetCommentListParams` and `CommentOrder` in `backlog-issue`.
    -   Implemented `IssueApi::get_comment_list` method and tests.
    -   Updated `backlog-api-client` to re-export new types.
    -   Addressed various workspace-wide build issues.
-   **`get_issue_comments` MCP Tool Implementation**: User requested to implement an MCP tool to get issue comments.
    -   Defined `GetIssueCommentsRequest` in `mcp-backlog-server`.
    -   Implemented `get_issue_comments_impl` bridge function.
    -   Registered `get_issue_comments` tool in `mcp-backlog-server`.
    -   Updated `mcp-backlog-server/README.md`.
    -   Verified with `cargo build`, `clippy`, and `test`.
-   **Error Handling Improvement**: Based on user feedback regarding unclear error messages (especially for potential auth/config issues leading to JSON parsing errors), the error handling in the API client and MCP server was enhanced.
    -   `backlog-api-core::Error` was augmented with an `HttpStatus` variant to store structured error details from Backlog API's non-2xx responses.
    -   The `client::Client` was updated to attempt parsing these structured errors.
    -   The `mcp-backlog-server`'s error conversion logic was improved to provide more contextually helpful messages to the MCP client for `ApiError::HttpStatus` and `ApiError::Json` cases.
