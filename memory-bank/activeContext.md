## Current Work Focus
-   Completed a significant refactoring of error handling and type re-exports across the workspace.
-   Currently updating the Memory Bank to reflect these architectural improvements.

## Recent Changes
-   **Error Handling and Dependency Refactoring (The "100-Point Plan"):**
    -   **Unified Error Type**: `backlog-api-core::Error` (aliased as `ApiError` by `backlog-api-client`) was enhanced to wrap `backlog_core::Error` (validation errors for IDs, etc.) as a new `Validation` variant. This makes `ApiError` the primary error type returned by `backlog-api-client` and its API handlers.
    -   **API Handler Standardization**: Ensured all API handlers (e.g., `IssueApi`, `DocumentApi`, `GitHandler`) consistently return `Result<T, ApiError>`. Internal errors like `backlog_core::Error` are now mapped to `ApiError::Validation`.
    -   **Consumer Error Handling Simplification**: `mcp-backlog-server`'s local error type (`mcp_backlog_server::error::Error`) was refactored to remove direct handling of `backlog_core::Error`. It now primarily converts `ApiError` (which includes wrapped core errors) into its own error variants.
    -   **Comprehensive Type Re-exports**: `backlog-api-client/src/lib.rs` was updated to re-export a wider range of types from sub-crates, including:
        - `backlog_core::Error` as `BacklogCoreError`.
        - Request builder types like `GetIssueListParamsBuilder` and `UpdateIssueParamsBuilder` from `backlog_issue::requests`.
        - Specific ID types from `backlog_core::identifier` (e.g., `ProjectId`, `StatusId`, `UserId`).
    -   **Consumer `use` Statement Updates**: `mcp-backlog-server` and `blg` (CLI tool) were updated to import these types primarily through `backlog-api-client`.
    -   **Dependency Simplification**: `mcp-backlog-server/Cargo.toml` was modified to remove direct dependencies on `backlog-core`, `backlog-issue`, `backlog-document`, and `backlog-git`. It now relies on `backlog-api-client` for these functionalities, making the sub-crates transitive dependencies.
    -   **Verification**: All changes were verified with `cargo check`, `cargo test`, `cargo clippy -D warnings`, and `cargo fmt`.
-   **Previous: `mcp-backlog-server` Tool Module Refactoring:**
    -   Organized tool helper modules into a `tools/` subdirectory within `mcp-backlog-server`.

## Next Steps
-   Finalize Memory Bank updates for the recent refactoring.
-   Await further instructions from the user for the next development task.


## Active Decisions & Considerations
-   **Facade Pattern Strength**: The `backlog-api-client` crate is increasingly acting as the primary facade for all Backlog API interactions, providing a unified interface for types and errors. This simplifies its usage for consumers like `mcp-backlog-server` and `blg`.
-   **Unified Error Handling**: The decision to make `backlog-api-core::Error` (`ApiError`) the central error type, capable of wrapping lower-level errors like `backlog_core::Error`, promotes consistency in error handling for library consumers.
-   **Minimized Direct Dependencies for Consumers**: `mcp-backlog-server` now has significantly fewer direct dependencies on individual `backlog-*` sub-crates, relying on `backlog-api-client` to expose the necessary API surface. This improves modularity and reduces coupling for consumers.
-   The project remains a Rust workspace with a modular design, but the interaction patterns between the facade (`backlog-api-client`) and its consumers are now cleaner.
-   CLI tool (`blg`) uses `clap`. MCP server tools in `mcp-backlog-server` are organized under `src/tools/`.

## Important Patterns & Preferences
-   **Centralized Facade (`backlog-api-client`)**: Consumers should primarily interact with `backlog-api-client` rather than directly with sub-crates like `backlog-core`, `backlog-issue`, etc.
-   **Consistent Error Propagation**: Errors from underlying operations (HTTP, JSON parsing, core type validation) are expected to be propagated or converted into `backlog_api_core::Error` by the `backlog-api-client` library layer.
-   Standard Rust project structure, workspace for multiple crates, feature flags for optionality, `thiserror` for error definitions, and `schemars` for MCP schemas remain key patterns.

## Learnings & Project Insights
-   Refactoring to a stronger facade pattern significantly cleans up consumer dependencies and simplifies their `use` statements and error handling logic.
-   Unifying error types requires careful consideration of how different error sources (validation, HTTP, etc.) are represented and converted, but leads to a more ergonomic API for the library user.
-   The `#[from]` attribute in `thiserror` is very powerful for creating ergonomic error conversions.
-   Iterative refinement of `Cargo.toml` dependencies and `use` statements, guided by compiler and Clippy feedback, is effective for achieving cleaner crate linkage.
