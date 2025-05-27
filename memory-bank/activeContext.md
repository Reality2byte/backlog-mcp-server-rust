## Current Work Focus
-   Improved error messaging for `MilestoneNotFoundByName` in `mcp-backlog-server`.
-   Updating Memory Bank to reflect this change.

## Recent Changes
-   **Error Message Improvement in `mcp-backlog-server`:**
    -   Modified `mcp-backlog-server/src/error.rs`: Updated the `From<Error> for McpError` implementation for the `MilestoneNotFoundByName` variant to include a suggestion to use the `get_version_milestone_list` tool.
    -   Confirmed change with `cargo check`.
-   **`mcp-backlog-server` crate changes for `get_issues_by_milestone_name` tool (immediately prior):**
    -   Defined `GetIssuesByMilestoneNameRequest` struct in `mcp-backlog-server/src/server.rs`.
    -   Implemented `get_issues_by_milestone_name_impl` helper function in `mcp-backlog-server/src/issue.rs`.
    -   Added `get_issues_by_milestone_name` tool method to `Server` in `mcp-backlog-server/src/server.rs`.
    -   Added `MilestoneNotFoundByName` error variant to `mcp-backlog-server/src/error.rs`.
    -   Added `From<ProjectId/ProjectKey> for ProjectIdOrKey` in `backlog-core` to resolve compilation issues.
    -   Confirmed `mcp-backlog-server` crate compilation with `cargo check`.
-   **`mcp-backlog-server` crate changes for `get_version_milestone_list` tool (further prior):**
    -   Defined `GetVersionMilestoneListRequest` struct.
    -   Implemented `get_version_milestone_list_impl` helper function.
    -   Added `get_version_milestone_list` tool method.
-   **Previous (`backlog-issue` crate) changes for `get_version_milestone_list` API:**
    -   Modified `Milestone` struct (added `display_order`).
    -   Added `get_version_milestone_list` async method and tests.
-   Previous work also involved understanding and documenting the existing `mcp-backlog-server`.


## Next Steps
-   Update `progress.md` to reflect the error message improvement.
-   Confirm completion of the task with the user.


## Active Decisions & Considerations
-   The project is a Rust workspace for a Backlog API client and CLI.
-   The modular structure (crates for core, issue, project, etc.) is a key architectural pattern.
-   The `client` crate seems to be a generic HTTP client, while `backlog-api-client` is the specific Backlog API orchestrator.

## Important Patterns & Preferences
-   The project follows standard Rust project structure and conventions (e.g., `Cargo.toml`, `src/lib.rs`, `src/main.rs` or `src/bin/`).
-   Use of workspace for managing multiple related crates.

## Learnings & Project Insights
-   The project is well-structured into logical components (crates).
-   The presence of `blg.rs` in `backlog-api-client/src/bin/` strongly indicates a CLI component.
-   Core data types are centralized in `backlog-core`.
