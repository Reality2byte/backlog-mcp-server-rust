## Current Work Focus
-   Updating Memory Bank to reflect new builder pattern convention and ensure all files are up-to-date.

## Recent Changes
-   **Established New Builder Pattern Convention**:
    -   Adopted the convention to use `#[builder(..., build_fn(error = "ApiError"))]` for request parameter structs using `derive_builder`. This ensures that the `build()` method returns `Result<Self, backlog_api_core::Error>`.
    -   Verified that `backlog-api-core/src/error.rs` correctly implements `From<derive_builder::UninitializedFieldError>` for `ApiError`. (User confirmed `SetFieldError` conversion is not needed for this project's `derive_builder` version/usage).
    -   Verified that relevant request parameter structs in `backlog-issue` (e.g., `GetCommentListParams`, `AddIssueParams`, `GetIssueListParams`, `UpdateIssueParams`) adhere to this new convention.
-   **Previous: Implemented `get_comment_list` API for Issues:**
    -   Defined new data models (`Comment`, `ChangeLogEntry`, `Star`, `Notification`) in `backlog-issue/src/models/comment.rs`. These models derive `Deserialize`, `Serialize`, `Debug`, `Clone`, `PartialEq`, and conditionally `JsonSchema`.
    -   Defined request parameters (`GetCommentListParams`, `CommentOrder`) with a builder pattern in `backlog-issue/src/requests/get_comment_list.rs`.
    -   Added the `get_comment_list` async method to `IssueApi` in `backlog-issue/src/api/mod.rs`.
    -   Added comprehensive unit tests with mocking for `get_comment_list` in `backlog-issue/src/api/mod.rs`.
    -   Added Rustdoc comments for all new public types and methods.
    -   Updated `backlog-api-client/src/lib.rs` to re-export the new comment-related types under the `issue` feature.
-   **Previous: Workspace-wide Build Fixes & Refinements (necessitated by `get_comment_list` and previous stubbing):**
    -   Added `schemars` as an optional dependency and feature to `backlog-issue/Cargo.toml` and `backlog-core/Cargo.toml`.
    -   Updated `User` (in `backlog-core/src/user.rs`), `Role` (in `backlog-core/src/role.rs`), `Language` (in `backlog-core/src/language.rs`), and all ID types (in `backlog-core/src/identifier.rs` via macro) to conditionally derive `JsonSchema` when the `schemars` feature of `backlog-core` is active.
    -   Corrected the `create_mock_user` test helper in `backlog-issue/src/api/mod.rs` regarding `UserId` type (u32) and the correct variants for `Role` (`Developer`) and `Language` (`Japanese`).
    -   Corrected the usage of `IssueIdOrKey::Key` and `IssueIdOrKey::Id` variants in `backlog-issue` tests.
    -   Populated previously stubbed request parameter structs (`GetIssueListParams`, `UpdateIssueParams`) in `backlog-issue/src/requests/mod.rs` with necessary fields to resolve build errors in `blg` (CLI) and `mcp-backlog-server`.
    -   Implemented `From<GetIssueListParams> for Vec<(String, String)>` in `backlog-issue/src/requests/mod.rs` to correctly serialize query parameters, fixing a failing test.
    -   Enhanced error handling in `mcp-backlog-server/src/error.rs` to include `From` implementations for `GetIssueListParamsBuilderError` and `UpdateIssueParamsBuilderError` from `backlog-api-client`.
-   **Previous: Verification**: All changes related to `get_comment_list` were successfully verified with `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-targets --all-features`.

## Next Steps
-   Finalize Memory Bank updates for the new builder pattern convention.
-   Await further instructions from the user.


## Active Decisions & Considerations
-   **Facade Pattern Strength**: The `backlog-api-client` crate is increasingly acting as the primary facade for all Backlog API interactions, providing a unified interface for types and errors. This simplifies its usage for consumers like `mcp-backlog-server` and `blg`.
-   **Unified Error Handling**: The decision to make `backlog-api-core::Error` (`ApiError`) the central error type, capable of wrapping lower-level errors like `backlog_core::Error`, promotes consistency in error handling for library consumers.
-   **Minimized Direct Dependencies for Consumers**: `mcp-backlog-server` now has significantly fewer direct dependencies on individual `backlog-*` sub-crates, relying on `backlog-api-client` to expose the necessary API surface. This improves modularity and reduces coupling for consumers.
-   The project remains a Rust workspace with a modular design, but the interaction patterns between the facade (`backlog-api-client`) and its consumers are now cleaner.
-   CLI tool (`blg`) uses `clap`. MCP server tools in `mcp-backlog-server` are organized under `src/tools/`.

## Important Patterns & Preferences
-   **Centralized Facade (`backlog-api-client`)**: Consumers should primarily interact with `backlog-api-client` rather than directly with sub-crates like `backlog-core`, `backlog-issue`, etc.
-   **Consistent Error Propagation**: Errors from underlying operations (HTTP, JSON parsing, core type validation) are expected to be propagated or converted into `backlog_api_core::Error` by the `backlog-api-client` library layer.
-   **Builder Pattern for Request Params**: Request parameter structs using `derive_builder` should specify `#[builder(..., build_fn(error = "ApiError"))]` to ensure builder errors are `backlog_api_core::Error`. This requires `ApiError` to implement `From<derive_builder::UninitializedFieldError>`.
-   Standard Rust project structure, workspace for multiple crates, feature flags for optionality, `thiserror` for error definitions, and `schemars` for MCP schemas remain key patterns.

## Learnings & Project Insights
-   Refactoring to a stronger facade pattern significantly cleans up consumer dependencies and simplifies their `use` statements and error handling logic.
-   Unifying error types requires careful consideration of how different error sources (validation, HTTP, etc.) are represented and converted, but leads to a more ergonomic API for the library user.
-   The `#[from]` attribute in `thiserror` is very powerful for creating ergonomic error conversions.
-   Iterative refinement of `Cargo.toml` dependencies and `use` statements, guided by compiler and Clippy feedback, is effective for achieving cleaner crate linkage.
