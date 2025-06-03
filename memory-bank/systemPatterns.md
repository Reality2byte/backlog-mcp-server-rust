# System Patterns

## Overall Architecture
The project is a Rust workspace composed of multiple crates, designed to provide a client library, a CLI tool, and an MCP server for the Backlog API.

```mermaid
graph TD
    subgraph "User Interfaces"
        CLI_Tool["backlog-api-client (binary: blg)"]
        MCP_Srv["mcp-backlog-server (binary)"]
    end

    subgraph "Core Library & API Modules"
        LibClient["backlog-api-client (library)"]
        API_Issue["backlog-issue"]
        API_Project["backlog-project"]
        API_Space["backlog-space"]
        API_User["backlog-user"]
        API_Document["backlog-document"]
        API_Git["backlog-git"]
        HTTP_Client["client (generic HTTP)"]
        API_Core_Util["backlog-api-core (utilities)"]
        Core_Types_Models["backlog-core (data models)"]
    end

    subgraph "External Dependencies & Protocols"
        RMCP_SDK["rmcp SDK (MCP)"]
        BacklogAPI["Backlog API (Remote)"]
    end

    CLI_Tool --> LibClient
    MCP_Srv --> LibClient
    MCP_Srv --> RMCP_SDK

    LibClient --> API_Issue
    LibClient --> API_Project
    LibClient --> API_Space
    LibClient --> API_User
    LibClient --> API_Document
    LibClient --> API_Git

    API_Issue --> HTTP_Client
    API_Project --> HTTP_Client
    API_Space --> HTTP_Client
    API_User --> HTTP_Client
    API_Document --> HTTP_Client
    API_Git --> HTTP_Client

    HTTP_Client --> API_Core_Util
    HTTP_Client --> BacklogAPI

    API_Issue --> Core_Types_Models
    API_Project --> Core_Types_Models
    API_Space --> Core_Types_Models
    API_User --> Core_Types_Models
    API_Document --> Core_Types_Models
    API_Git --> Core_Types_Models
    API_Core_Util --> Core_Types_Models
    LibClient --> Core_Types_Models
    API_Core_Util --> Core_Types_Models # backlog-api-core now depends on backlog-core for error wrapping
    # MCP_Srv no longer directly depends on Core_Types_Models for types; it gets them via LibClient.
```

## Key Crates and Their Roles

1.  **`client` (crate)**:
    *   Provides a generic, low-level HTTP client (`struct Client`).
    *   Handles making HTTP requests (GET, POST, PATCH, DELETE) and deserializing responses.
    *   Manages base URL and authentication details (API key, token).
    *   Used by the specific API module crates.

2.  **`backlog-core` (crate)**:
    *   Defines core data structures (e.g., `IssueKey`, `ProjectIdOrKey`, `IssueIdOrKey`, `User`, `DocumentId`) and enums (`Role`, `Language`) shared across the Backlog API modules.
    *   Ensures consistency in data representation.

3.  **`backlog-api-core` (crate)**:
    *   Provides common API-related utilities. Its `Error` type (often aliased as `ApiError`) is central to the library's error handling, now capable of wrapping `backlog_core::Error` (validation errors) in addition to HTTP, JSON, and URL errors. Defines the `Result` type used throughout the API client library.

4.  **`backlog-issue` (API module crate)**:
    *   Responsible for the Issue domain of the Backlog API, including issue details, listing, milestones, and comments.
    *   Defines domain-specific models like `Issue`, `Milestone`, `Comment`, `ChangeLogEntry`, `Star`, `Notification`.
    *   Defines request parameter structs like `GetIssueListParams`, `UpdateIssueParams`, `GetCommentListParams`.
    *   Implements API endpoint wrappers in `IssueApi` (e.g., `get_issue`, `get_issue_list`, `update_issue`, `get_comment_list`, `get_version_milestone_list`).
    *   Depends on `backlog-core` for shared types (e.g., `User`, `IssueIdOrKey`) and `client` for HTTP communication.

5.  **`backlog-project`, `backlog-space`, `backlog-user`, `backlog-document` (other API module crates)**:
    *   Each crate is responsible for a specific domain of the Backlog API (Projects, Space, Users, Documents respectively).
    *   Define domain-specific request and response structs/models.
    *   Implement API endpoint wrappers/methods using the generic `client` crate.
    *   Depend on `backlog-core` for shared types and `client` for HTTP communication.

6.  **`backlog-git` (API module crate)**:
    *   Responsible for the Git and Pull Request domains of the Backlog API.
    *   Defines domain-specific models (`Repository`, `PullRequest`) and API interaction logic (`GitHandler`).
    *   Depends on `backlog-core` and `client`.

7.  **`backlog-api-client` (crate - library part)**:
    *   Acts as the main entry point or facade for the Backlog API library. It re-exports key data models, ID types, request builders, error types (`ApiError`, `BacklogCoreError`), and API handlers from the underlying crates.
    *   The `BacklogApiClient` struct aggregates or provides accessors for the individual API module clients (e.g., `issue()`, `project()`, `document()`, `git()`).
    *   Simplifies usage for library consumers by providing a unified API surface.

8.  **`backlog-api-client` (crate - binary `blg`)**:
    *   The command-line interface tool, now using `clap` for argument parsing.
    *   Provides subcommands for various Backlog operations, including Git, PRs, and Issues (e.g., `issue list`, `issue show`).
    *   Uses the `backlog-api-client` library.

9.  **`mcp-backlog-server` (crate - binary)**:
    *   Implements an MCP server using the `rmcp` Rust SDK.
    *   Exposes tools (e.g., `get_issue_details`, `get_document_details`, `get_repository_list`, `get_issue_comments`) that wrap functionalities of the `backlog-api-client`.
    *   Primarily depends on `backlog-api-client` for Backlog functionalities and types, minimizing direct dependencies on other `backlog-*` sub-crates.
    *   Configured via environment variables (`BACKLOG_BASE_URL`, `BACKLOG_API_KEY`) passed by the MCP system.
    *   Communicates with MCP clients over stdio.
    *   Tool logic (helper implementations) is organized into modules within a `src/tools/` directory (e.g., `tools/issue.rs`, `tools/document.rs`, `tools/git.rs`), called from methods in `server.rs`.

## Design Patterns
-   **Workspace Structure**: Manages multiple interdependent crates.
-   **Facade Pattern**: `backlog-api-client` library simplifies access to API modules.
-   **Modular Design**: Functionality broken down by Backlog entity into separate crates.
-   **Builder Pattern for Request Parameters**: `derive_builder` is used for request parameter structs. Conventionally, these are configured with `#[builder(..., build_fn(error = "ApiError"))]` so that their `build()` method returns `Result<Self, backlog_api_core::Error>`.
-   **Centralized Core Types**: `backlog-core` for common data models.
-   **MCP Tool Implementation**: In `mcp-backlog-server`, tools are methods on a `Server` struct, using `#[tool(tool_box)]` and `#[tool(aggr)]` attributes from the `rmcp` SDK. Input/output schemas are defined via struct derives (`schemars::JsonSchema`) or `serde_json::json!`.
-   **Layered Architecture**:
    -   User Interfaces: CLI (`blg`), MCP Server (`mcp-backlog-server`)
    -   Main API Client Library (`backlog-api-client`)
    -   Specific API Modules (`backlog-issue`, `backlog-document`, `backlog-git`, etc.)
    -   Generic HTTP Client (`client`)
    -   Foundational: `backlog-core` (Data Models), `backlog-api-core` (API Utilities)

## Component Relationships
-   `blg` (CLI) depends on `backlog-api-client` (library) for all Backlog-related types and functionalities.
-   `mcp-backlog-server` depends on `rmcp` SDK and `backlog-api-client` (library) for all Backlog-related types and functionalities. Direct dependencies on `backlog-core`, `backlog-issue`, `backlog-document`, and `backlog-git` have been removed from `mcp-backlog-server`.
-   `backlog-api-client` (library) depends on API module crates (`backlog-issue`, `backlog-project`, `backlog-document`, `backlog-git`, etc., as optional features) and directly on `backlog-core` and `backlog-api-core`.
-   API module crates (e.g., `backlog-issue`, `backlog-git`) depend on `client` (for HTTP), `backlog-core` (for types), and `backlog-api-core` (for `Result` and `ApiError`).
-   `client` (generic HTTP client) depends on `backlog-api-core` (for `Result` and `ApiError`).
-   `backlog-api-core` now depends on `backlog-core` to allow `ApiError` to wrap `backlog_core::Error`.

## Critical Implementation Paths
-   **Authentication**: Handling API keys/tokens in `client` and `mcp-backlog-server` (via env vars).
-   **Request/Response Handling**: Serialization/deserialization (`serde`), URL construction, HTTP method mapping in `client` and API module crates.
-   **Error Handling**: Unified error propagation. Errors from HTTP (`reqwest`), JSON parsing (`serde_json`), URL parsing (`url`), and core type validation (`backlog_core::Error`) are all converted to or wrapped by `backlog_api_core::Error` (`ApiError`). Consumers of `backlog-api-client` primarily handle `ApiError`.
-   **MCP Tool Definition**: Correctly defining tool schemas and implementing the `call` logic in `mcp-backlog-server`.
-   **API Endpoint Coverage**: Systematically implementing methods for Backlog API endpoints.
