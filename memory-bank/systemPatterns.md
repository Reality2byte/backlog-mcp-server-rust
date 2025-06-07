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
        API_Issue["backlog-issue (uses Status)"]
        API_Project["backlog-project (defines Status)"]
        API_Space["backlog-space"]
        API_User["backlog-user"]
        API_Document["backlog-document"]
        API_Git["backlog-git"]
        HTTP_Client["client (generic HTTP, provides test_utils)"]
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
    API_Issue --> API_Project # For Status model
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

    subgraph MCP_Srv_Internals
        direction LR
        MCP_Srv_Main["server.rs (Tool Registration)"]
        MCP_Srv_Issue["issue module (bridge, request)"]
        MCP_Srv_Git["git module (bridge, request)"]
        MCP_Srv_Doc["document module (bridge, request)"]
        MCP_Srv_Proj["project module (bridge, request)"]
        MCP_Srv_Error["error.rs"]
        MCP_Srv_Lib["lib.rs (Module Exports)"]

        MCP_Srv --> MCP_Srv_Main
        MCP_Srv_Main --> MCP_Srv_Issue
        MCP_Srv_Main --> MCP_Srv_Git
        MCP_Srv_Main --> MCP_Srv_Doc
        MCP_Srv_Main --> MCP_Srv_Proj
        MCP_Srv_Issue --> MCP_Srv_Error
        MCP_Srv_Git --> MCP_Srv_Error
        MCP_Srv_Doc --> MCP_Srv_Error
        MCP_Srv_Proj --> MCP_Srv_Error
        MCP_Srv_Lib -.-> MCP_Srv_Issue
        MCP_Srv_Lib -.-> MCP_Srv_Git
        MCP_Srv_Lib -.-> MCP_Srv_Doc
        MCP_Srv_Lib -.-> MCP_Srv_Proj
    end
```

## Key Crates and Their Roles

1.  **`client` (crate)**:
    *   Provides a generic, low-level HTTP client (`struct Client`).
    *   Handles making HTTP requests and deserializing JSON responses.
    *   Includes `download_file_raw` method for fetching raw byte streams (e.g., file downloads).
    *   Manages base URL and authentication.
    *   Provides common test utilities (e.g., `setup_client`) via a `test-utils` feature.

2.  **`backlog-core` (crate)**:
    *   Defines core data structures (ID types, `User`, `IssueKey`, etc.) and enums shared across modules.

3.  **`backlog-api-core` (crate)**:
    *   Provides common API utilities, including the central `Error` type (`ApiError`) and `Result`.

4.  **`backlog-project` (API module crate)**:
    *   Responsible for the Project domain of the Backlog API.
    *   Defines project-specific models like `Project` and `Status`.
    *   Implements API endpoint wrappers like `get_project`, `get_project_list`, and `get_status_list`.
    *   Depends on `backlog-core` and `client`.

5.  **`backlog-issue` (API module crate)**:
    *   Responsible for the Issue domain.
    *   Defines models like `Issue`, `Milestone`, `Comment`, and `Attachment`.
    *   Uses `backlog_project::Status` for the `Issue.status` field.
    *   Implements API endpoint wrappers like `get_issue`, `get_issue_list`, `get_comment_list`, `get_attachment_list`, and `get_attachment_file` (for downloading attachment content as `bytes::Bytes`).
    *   Depends on `backlog-core`, `client`, and now `backlog-project` (for `Status`).

6.  **`backlog-space`, `backlog-user`, `backlog-document`, `backlog-git` (other API module crates)**:
    *   Handle their respective API domains. Depend on `backlog-core` and `client`.

7.  **`backlog-api-client` (crate - library part)**:
    *   Main facade, re-exporting types and API handlers. Also re-exports `AttachmentId` from `backlog_core::identifier`.

8.  **`backlog-api-client` (crate - binary `blg`)**:
    *   CLI tool using the library.
    *   Provides subcommands for various operations, e.g., `issue list`, `issue show`, `issue download-attachment`.

9.  **`mcp-backlog-server` (crate - binary)**:
    *   MCP server using the `backlog-api-client` library.
    *   Organizes tools into domain-specific modules (e.g., `issue` for issue details, comments, attachment listing, image-specific attachment download using `Content::image`, and text-specific attachment download using `Content::text`; `git` for repositories, pull requests; `document` for document details; `project` for project-level info like statuses). Each module typically contains:
        *   `request.rs`: Defines request structs deriving `serde::Deserialize` and `rmcp::schemars::JsonSchema` (using the `use rmcp::schemars;` convention).
        *   `bridge.rs`: Contains functions that take these request structs and the `BacklogApiClient` (wrapped in `Arc<Mutex<>>`), perform the API call, and return a `crate::error::Result`.
    *   `server.rs` defines the main `Server` struct and registers tool methods (annotated with `#[tool(...)]`) that call these bridge functions.
    *   `error.rs` defines a custom error type (`crate::error::Error`) that wraps `ApiError` and `CoreError`. It also provides an `impl From<crate::error::Error> for rmcp::Error` for integration with the MCP framework.

## Design Patterns
-   **Workspace Structure**.
-   **Facade Pattern** (`backlog-api-client`).
-   **Modular Design** (both for API client crates and MCP server modules).
-   **Builder Pattern for Request Parameters**.
-   **Centralized Core Types** (`backlog-core`).
-   **MCP Tool Implementation**:
    -   Tools are methods on the `Server` struct in `mcp-backlog-server/src/server.rs`.
    -   Logic for each tool is delegated to a "bridge" function in a domain-specific module (e.g., `mcp-backlog-server/src/project/bridge.rs`).
    -   Request parameters are defined in structs within these modules (e.g., `mcp-backlog-server/src/project/request.rs`).
    -   Consistent error handling is achieved by bridge functions returning `crate::error::Result`, which is then converted to `rmcp::Error` in `server.rs` using `?`.
-   **Layered Architecture**.
-   **Shared Test Utilities**: Common test helpers (like `setup_client`) are provided by the `client` crate via a feature flag (`test-utils`) to promote consistency and reduce duplication in tests of other crates.

## Component Relationships
-   `blg` (CLI) depends on `backlog-api-client` (library).
-   `mcp-backlog-server` depends on `rmcp` SDK and `backlog-api-client` (library).
-   `backlog-api-client` (library) depends on API module crates and core crates.
-   API module crates:
    -   Generally depend on `client`, `backlog-core`, and `backlog-api-core`.
    -   **New**: `backlog-issue` now depends on `backlog-project` for the `Status` type.
-   `client` depends on `backlog-api-core`.
-   `backlog-api-core` depends on `backlog-core`.

## Critical Implementation Paths
-   **Authentication**.
-   **Request/Response Handling**.
-   **Error Handling**: Unified via `ApiError`, with `HttpStatus` variant for structured API errors.
-   **Model Definition and Ownership**:
    -   Core IDs (like `UserId`, `IssueId`, `AttachmentId`) and truly shared simple types (like `User`) in `backlog-core`.
    -   More complex, domain-specific models in their respective crates (e.g., `ProjectStatus` in `backlog-project`, `Attachment` in `backlog-issue`).
    -   If a model from one domain crate is semantically part of another (e.g., an `Issue` has a `Status`), a dependency is established (e.g., `backlog-issue` uses `backlog_project::ProjectStatus`).
-   **MCP Tool Definition**:
    -   Tools are organized into modules by domain (e.g., `issue`, `git`, `document`, `project`) within `mcp-backlog-server`.
    -   Each module typically has `request.rs` for input structs (using `rmcp::schemars`) and `bridge.rs` for the core logic.
    -   The main `server.rs` registers these tools (often via macros like `#[tool(tool_box)]`).
-   **API Endpoint Coverage**.
