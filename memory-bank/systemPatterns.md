# System Patterns

## Overall Architecture
The project is a Rust workspace composed of multiple crates, designed to provide a client library and a CLI tool for the Backlog API.

```mermaid
graph TD
    CLI[backlog-api-client (binary: blg)] --> LibClient[backlog-api-client (library)]
    LibClient --> API_Issue[backlog-issue]
    LibClient --> API_Project[backlog-project]
    LibClient --> API_Space[backlog-space]
    LibClient --> API_User[backlog-user]

    API_Issue --> HTTP_Client[client]
    API_Project --> HTTP_Client
    API_Space --> HTTP_Client
    API_User --> HTTP_Client

    HTTP_Client --> API_Core[backlog-api-core]
    
    API_Issue --> Core_Types[backlog-core]
    API_Project --> Core_Types
    API_Space --> Core_Types
    API_User --> Core_Types
    API_Core --> Core_Types
    LibClient --> Core_Types

    subgraph "Workspace Crates"
        CLI
        LibClient
        API_Issue
        API_Project
        API_Space
        API_User
        HTTP_Client
        API_Core
        Core_Types
    end
```

## Key Crates and Their Roles

1.  **`client` (crate)**:
    *   Provides a generic, low-level HTTP client (`struct Client`).
    *   Handles making HTTP requests (GET, POST, DELETE) and deserializing responses.
    *   Manages base URL and authentication details (API key, token).
    *   Used by the more specific API crates (`backlog-issue`, `backlog-project`, etc.).

2.  **`backlog-core` (crate)**:
    *   Defines core data structures and enums shared across the Backlog API modules.
    *   Examples: `IssueKey`, `ProjectKey`, `SpaceKey`, `User`, `Role`, `Language`.
    *   Ensures consistency in data representation.

3.  **`backlog-api-core` (crate)**:
    *   Provides common API-related functionalities or types that are not specific to a Backlog entity but are core to API interaction.
    *   Example: `ApiRateLimit` struct.
    *   Likely handles common error types or API response wrappers.

4.  **`backlog-issue`, `backlog-project`, `backlog-space`, `backlog-user` (API module crates)**:
    *   Each crate is responsible for a specific domain of the Backlog API (e.g., issues, projects).
    *   They likely define:
        *   Specific request and response structs for their domain.
        *   API endpoint wrappers/methods that use the generic `client` crate to make calls.
        *   These crates depend on `backlog-core` for shared types and `client` for HTTP communication.

5.  **`backlog-api-client` (crate - library part)**:
    *   Acts as the main entry point or facade for the Backlog API library.
    *   The `BacklogApiClient` struct likely aggregates instances or provides accessors for the individual API module clients (e.g., `issue()`, `project()`).
    *   Simplifies usage for library consumers by providing a single client object.

6.  **`backlog-api-client` (crate - binary `blg`)**:
    *   The command-line interface tool.
    *   Uses the `backlog-api-client` library to interact with the Backlog API.
    *   Parses command-line arguments and translates them into API calls.

## Design Patterns
-   **Workspace Structure**: Leverages Rust's workspaces to manage multiple interdependent crates, promoting modularity and code reuse.
-   **Facade Pattern**: The `backlog-api-client` library acts as a facade, simplifying the interface to the various underlying API module crates.
-   **Modular Design**: Functionality is broken down by Backlog entity (issue, project, etc.) into separate crates.
-   **Centralized Core Types**: `backlog-core` centralizes common data models, reducing redundancy and ensuring consistency.
-   **Layered Architecture**:
    -   CLI (highest level)
    -   Main API Client Library (`backlog-api-client`)
    -   Specific API Modules (`backlog-issue`, etc.)
    -   Generic HTTP Client (`client`)
    -   Core Data Types (`backlog-core`) & API Core Utilities (`backlog-api-core`) (lowest level, foundational)

## Component Relationships
-   The `blg` CLI depends directly on the `backlog-api-client` library.
-   The `backlog-api-client` library depends on all the specific API module crates (`backlog-issue`, `backlog-project`, etc.) and `backlog-core`.
-   Each specific API module crate (`backlog-issue`, etc.) depends on the `client` crate for HTTP operations and `backlog-core` for data types.
-   The `client` crate depends on `backlog-api-core` (e.g., for error handling, rate limiting info).
-   Most crates will depend on `backlog-core` for shared data structures.

## Critical Implementation Paths
-   **Authentication**: Securely handling API keys/tokens in both the library and CLI.
-   **Request/Response Serialization/Deserialization**: Correctly mapping Rust structs to JSON and vice-versa for API communication, handled by `serde`.
-   **Error Handling**: Propagating and translating HTTP errors and API-specific errors into meaningful Rust `Result` types. The `thiserror` crate is likely used for this.
-   **API Endpoint Coverage**: Systematically implementing methods for various Backlog API endpoints within their respective modules.
