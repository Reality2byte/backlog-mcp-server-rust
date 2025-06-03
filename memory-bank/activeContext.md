## Current Work Focus
-   Improved error handling in the Backlog API client (`client` crate) and MCP server (`mcp-backlog-server`) for clearer error reporting, particularly for HTTP errors and JSON parsing issues.

## Recent Changes
-   **Improved Error Handling for API Client and MCP Server**:
    -   Defined `BacklogApiErrorEntry` and `BacklogApiErrorResponse` structs in `backlog-api-core/src/error.rs` to represent structured errors from the Backlog API. Made these structs public and re-exported them from `backlog-api-core/src/lib.rs`.
    -   Added a new `HttpStatus { status: u16, errors: Vec<BacklogApiErrorEntry>, errors_summary: String }` variant to `backlog_api_core::Error` (`ApiError`) to hold structured Backlog API error details.
    -   Modified `client/src/client.rs` (`execute_request` function):
        -   For non-2xx HTTP responses, it now attempts to parse the response body as `BacklogApiErrorResponse`.
        -   If successful, it returns `ApiError::HttpStatus` with the parsed errors.
        -   If parsing the error body fails, it currently returns `ApiError::InvalidBuildParameter` with the status and raw error body (this is a temporary fallback, ideally a more specific error variant would be used).
    -   Updated `mcp-backlog-server/src/error.rs`:
        -   The `From<Error> for McpError` implementation now specifically handles `ApiError::HttpStatus` to provide a clear message to the MCP user.
        -   For `ApiError::Json` (which occurs on 2xx responses with unexpected JSON structure), it now generates a more helpful `McpError::internal_error` message, suggesting potential misconfigurations or API changes.
    -   Verified all changes with `cargo build --all-features`, `cargo clippy --all-features -- -D warnings`, and `cargo test --all-features --all-targets`.
-   **Implemented `get_issue_comments` MCP Tool**:
    -   Added `GetIssueCommentsRequest` struct to `mcp-backlog-server/src/issue/request.rs`.
    -   Implemented `get_issue_comments_impl` helper function in `mcp-backlog-server/src/issue/bridge.rs`.
    -   Registered `get_issue_comments` as a new tool in `mcp-backlog-server/src/server.rs`.
    -   Updated `mcp-backlog-server/README.md`.
    -   Verified with `cargo build`, `clippy`, and `test`.
-   **Established New Builder Pattern Convention**:
    -   Adopted `#[builder(..., build_fn(error = "ApiError"))]` for request parameter structs.
-   **Previous: Implemented `get_comment_list` API for Issues.**
-   **Previous: Workspace-wide Build Fixes & Refinements.**
-   **Previous: Verification of `get_comment_list` library changes.**

## Next Steps
-   Finalize Memory Bank updates for the error handling improvements.
-   Await further instructions from the user.

## Active Decisions & Considerations
-   **Facade Pattern Strength**: The `backlog-api-client` crate is increasingly acting as the primary facade.
-   **Unified Error Handling**: `backlog-api-core::Error` (`ApiError`) is the central error type. The recent changes enhance this by providing more structured information for HTTP errors from the Backlog API (via `ApiError::HttpStatus`) and improving how these are reported to end-users (especially MCP users).
-   **Minimized Direct Dependencies for Consumers**: `mcp-backlog-server` relies on `backlog-api-client`.
-   The project remains a Rust workspace with a modular design.
-   CLI tool (`blg`) uses `clap`. MCP server tools in `mcp-backlog-server` are organized.

## Important Patterns & Preferences
-   **Centralized Facade (`backlog-api-client`)**.
-   **Consistent Error Propagation**: Errors are propagated as `ApiError`. Non-2xx HTTP responses from Backlog are now parsed for detailed error messages and wrapped in `ApiError::HttpStatus`. JSON parsing errors on successful (2xx) responses are wrapped in `ApiError::Json`.
-   **Builder Pattern for Request Params**: Use `#[builder(..., build_fn(error = "ApiError"))]`.
-   Standard Rust project structure, workspace, feature flags, `thiserror`, `schemars`.

## Learnings & Project Insights
-   Clearer error messages are crucial, especially when distinguishing between client-side misconfiguration (e.g., wrong API key/URL) and server-side API issues or unexpected successful responses.
-   Parsing specific error structures from API responses (like Backlog's `errors` array) significantly improves debuggability over generic HTTP status errors.
-   The `Display` implementation of error enums, and how they are converted for final user presentation (e.g., to `McpError`), directly impacts user experience when errors occur.
-   Refactoring to a stronger facade pattern significantly cleans up consumer dependencies.
-   Unifying error types requires careful consideration.
-   The `#[from]` attribute in `thiserror` is very powerful.
-   Iterative refinement of dependencies and `use` statements is effective.
