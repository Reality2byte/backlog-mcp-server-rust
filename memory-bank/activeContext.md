## Current Work Focus
-   Implemented "Get Status List of Project" API in `backlog-project`.
-   Refactored `Status` model: defined a complete `Status` (`ProjectStatus`) in `backlog-project`, and updated `backlog-issue` to use it.
-   Commonized test utility `setup_client` into `client::test_utils`.
-   Implemented `get_project_status_list` MCP tool in `mcp-backlog-server`.

## Recent Changes
-   **Implemented "Get Status List of Project" API & Refactored `Status` Model**:
    -   Added `schemars` as an optional dependency and feature to `backlog-project/Cargo.toml`.
    -   Defined a new, complete `Status` struct in `backlog-project/src/models/status.rs` (fields: `id`, `project_id`, `name`, `color`, `display_order`), deriving `Deserialize`, `Serialize`, and conditionally `JsonSchema`.
    -   Exported `Status` from `backlog-project`'s `models/mod.rs` and `lib.rs`.
    -   Added `backlog-project` as a dependency to `backlog-issue/Cargo.toml` and updated its `schemars` feature to also enable `backlog-project/schemars`.
    -   In `backlog-issue/src/models/issue.rs`:
        -   Removed the local, incomplete `Status` struct definition.
        -   Changed the `Issue.status` field to use `Box<backlog_project::Status>`.
        -   Updated imports to use `backlog_project::Status`.
        -   Removed unused `StatusId` import.
    -   Removed the re-export of the old `Status` from `backlog-issue/src/lib.rs`.
    -   Implemented `get_status_list` async method in `backlog-project/src/api/mod.rs` within `ProjectApi`.
    -   Added unit tests for `get_status_list` in `backlog-project/src/api/mod.rs`, including success, empty list, and error cases.
    -   Updated mock JSON in `backlog-issue` tests for `Issue.status` to provide the full `ProjectStatus` structure, fixing previous test failures.
    -   Re-exported `ProjectStatus` from `backlog-api-client/src/lib.rs` under the `project` feature.
-   **Implemented `get_project_status_list` MCP Tool**:
    -   Created a new `project` module in `mcp-backlog-server` (`src/project/mod.rs`).
    -   Defined `GetProjectStatusListRequest` in `mcp-backlog-server/src/project/request.rs`.
    -   Implemented `get_project_status_list_tool` function in `mcp-backlog-server/src/project/bridge.rs`.
    -   Registered the `project` module in `mcp-backlog-server/src/lib.rs`.
    -   Added the `get_project_status_list` tool method to `Server` in `mcp-backlog-server/src/server.rs`.
    -   Updated `mcp-backlog-server/Cargo.toml`:
        -   Enabled `project` feature for `backlog-api-client` dependency.
        -   Added `backlog-project` and `backlog-core` as direct dependencies.
        -   Added `schemars` as a direct dependency.
-   **Commonized Test Utility `setup_client`**:
    -   Added `test-utils` feature to `client/Cargo.toml`, enabling `wiremock` as an optional dependency.
    -   Created `client/src/test_utils.rs` and defined `pub async fn setup_client` there.
    -   Conditionally exported `test_utils` module from `client/src/lib.rs` via the `test-utils` feature.
    -   Updated `backlog-project/Cargo.toml` and `backlog-issue/Cargo.toml` to enable the `test-utils` feature for their `client` dependency.
    -   Updated test modules in `backlog-project` and `backlog-issue` to import and use `client::test_utils::setup_client`.
-   **Improved Error Handling for API Client and MCP Server** (Previous task):
    -   Enhanced `backlog_api_core::Error` with `HttpStatus` variant.
    -   Improved `client::Client` to parse structured Backlog API errors.
    -   Improved `mcp-backlog-server` error conversion.
-   **Implemented `get_issue_comments` MCP Tool** (Previous task).
-   **Established New Builder Pattern Convention** (Previous task).

## Next Steps
-   Await further instructions from the user.

## Active Decisions & Considerations
-   **Model Ownership and Dependencies**:
    -   The canonical, complete status model (`ProjectStatus`) is now defined in `backlog-project` as statuses are project-level configurations.
    -   `backlog-issue` now depends on `backlog-project` to use this `ProjectStatus` for the `Issue.status` field. This reflects that an issue's status is one of the project-defined statuses.
-   **MCP Server Structure**: New MCP tools related to a specific domain (e.g., "project") are organized into their own module within `mcp-backlog-server` (e.g., `src/project/`). This module typically contains `request.rs` for input structs and `bridge.rs` for the core logic interfacing with `backlog-api-client`.
-   **Test Utilities**: Common test helpers like `setup_client` are being centralized in the `client` crate's `test_utils` module, exposed via a feature flag, to reduce duplication across test suites in different crates.
-   **Facade Pattern Strength**: `backlog-api-client` continues to be the primary facade.
-   **Unified Error Handling**: `backlog_api_core::Error` (`ApiError`) remains central.

## Important Patterns & Preferences
-   **Centralized Facade (`backlog-api-client`)**.
-   **Consistent Error Propagation**.
-   **Builder Pattern for Request Params**.
-   Standard Rust project structure, workspace, feature flags, `thiserror`, `schemars`.
-   **Model Placement**: Shared core types in `backlog-core`. Domain-specific models in their respective crates (e.g., `ProjectStatus` in `backlog-project`). If a model defined in one domain crate (e.g., `backlog-project::ProjectStatus`) is needed by another (e.g., `backlog-issue`), a direct dependency is added.
-   **MCP Tool Structure**: Tools in `mcp-backlog-server` are organized by domain into modules (e.g., `issue`, `git`, `project`). Each module typically contains:
    -   `request.rs`: Defines request structs deriving `Deserialize` and `JsonSchema`.
    -   `bridge.rs`: Contains functions that take these request structs and the `BacklogApiClient`, perform the API call, and return a `crate::error::Result`.
    -   The main `server.rs` file then defines tool methods that call these bridge functions.

## Learnings & Project Insights
-   Careful consideration of model ownership and inter-crate dependencies is crucial for maintaining a clean architecture, especially when types are shared or referenced across different API domains.
-   API documentation for embedded objects within larger responses (e.g., the `status` object within an `Issue`) must be checked to ensure local model definitions are complete and accurate. The `ProjectStatus` model is a good example of this.
-   Centralizing test utilities improves maintainability and consistency of tests across the workspace.
