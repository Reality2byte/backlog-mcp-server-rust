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
```

## Key Crates and Their Roles

1.  **`client` (crate)**:
    *   Provides a generic, low-level HTTP client (`struct Client`).
    *   Handles making HTTP requests and deserializing responses.
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
    *   Defines models like `Issue`, `Milestone`, `Comment`.
    *   Uses `backlog_project::Status` for the `Issue.status` field.
    *   Implements API endpoint wrappers like `get_issue`, `get_issue_list`, `get_comment_list`.
    *   Depends on `backlog-core`, `client`, and now `backlog-project` (for `Status`).

6.  **`backlog-space`, `backlog-user`, `backlog-document`, `backlog-git` (other API module crates)**:
    *   Handle their respective API domains. Depend on `backlog-core` and `client`.

7.  **`backlog-api-client` (crate - library part)**:
    *   Main facade, re-exporting types and API handlers.

8.  **`backlog-api-client` (crate - binary `blg`)**:
    *   CLI tool using the library.

9.  **`mcp-backlog-server` (crate - binary)**:
    *   MCP server using the library.

## Design Patterns
-   **Workspace Structure**.
-   **Facade Pattern** (`backlog-api-client`).
-   **Modular Design**.
-   **Builder Pattern for Request Parameters**.
-   **Centralized Core Types** (`backlog-core`).
-   **MCP Tool Implementation**.
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
    -   Core IDs and truly shared simple types in `backlog-core`.
    -   More complex, domain-specific models in their respective crates (e.g., `Status` in `backlog-project`).
    -   If a model from one domain crate is semantically part of another (e.g., an `Issue` has a `Status`), a dependency is established (e.g., `backlog-issue` uses `backlog_project::Status`).
-   **MCP Tool Definition**.
-   **API Endpoint Coverage**.
