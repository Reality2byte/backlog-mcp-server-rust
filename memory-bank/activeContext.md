## Current Work Focus
-   Implemented "Get Status List of Project" API in `backlog-project`.
-   Refactored `Status` model: defined a complete `Status` in `backlog-project`, and updated `backlog-issue` to use it.
-   Commonized test utility `setup_client` into `client::test_utils`.

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
    -   Updated mock JSON in `backlog-issue` tests for `Issue.status` to provide the full `Status` structure, fixing previous test failures.
    -   Re-exported `Status` from `backlog-api-client/src/lib.rs` under the `project` feature.
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
-   Finalize Memory Bank updates for the "Get Status List of Project" feature.
-   Await further instructions from the user.

## Active Decisions & Considerations
-   **Model Ownership and Dependencies**:
    -   The canonical, complete `Status` model (`Status`) is now defined in `backlog-project` as statuses are project-level configurations.
    -   `backlog-issue` now depends on `backlog-project` to use this `Status` for the `Issue.status` field. This reflects that an issue's status is one of the project-defined statuses.
-   **Test Utilities**: Common test helpers like `setup_client` are being centralized in the `client` crate's `test_utils` module, exposed via a feature flag, to reduce duplication across test suites in different crates.
-   **Facade Pattern Strength**: `backlog-api-client` continues to be the primary facade.
-   **Unified Error Handling**: `backlog_api_core::Error` (`ApiError`) remains central.

## Important Patterns & Preferences
-   **Centralized Facade (`backlog-api-client`)**.
-   **Consistent Error Propagation**.
-   **Builder Pattern for Request Params**.
-   Standard Rust project structure, workspace, feature flags, `thiserror`, `schemars`.
-   **Model Placement**: Shared core types in `backlog-core`. Domain-specific models in their respective crates (e.g., `Status` in `backlog-project`). If a model defined in one domain crate (e.g., `backlog-project::Status`) is needed by another (e.g., `backlog-issue`), a direct dependency is added.

## Learnings & Project Insights
-   Careful consideration of model ownership and inter-crate dependencies is crucial for maintaining a clean architecture, especially when types are shared or referenced across different API domains.
-   API documentation for embedded objects within larger responses (e.g., the `status` object within an `Issue`) must be checked to ensure local model definitions are complete and accurate.
-   Centralizing test utilities improves maintainability and consistency of tests across the workspace.
