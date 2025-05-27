## Current Work Focus
-   Implemented suggestion feature for `MilestoneNotFoundByName` error in `mcp-backlog-server`.
-   Updating Memory Bank to reflect this enhancement.

## Recent Changes
-   **Suggestion Feature for `MilestoneNotFoundByName` in `mcp-backlog-server`:**
    -   Added `strsim` crate to `mcp-backlog-server/Cargo.toml`.
    -   Modified `mcp-backlog-server/src/error.rs`:
        -   Added `suggestions: Option<Vec<String>>` field to the `MilestoneNotFoundByName` error variant.
        -   Updated `From<Error> for McpError` to include suggestions in the error message if present.
    -   Modified `mcp-backlog-server/src/issue.rs` (`get_issues_by_milestone_name_impl` function):
        -   Implemented a two-step approach for finding milestone ID by name:
            1.  Preprocess (lowercase, remove spaces and '#') input and milestone names, then check for exact match. If found, proceed to fetch issues (success path).
            2.  If no exact match, use Levenshtein distance (from `strsim` crate, threshold <= 2) on preprocessed names to find similar candidates.
            3.  Sort candidates by distance (ascending) and then by common prefix length with original input (descending).
            4.  Return `MilestoneNotFoundByName` error with the sorted suggestions (or None if no candidates).
    -   Resolved associated compilation errors and confirmed with `cargo check`.
-   **Previous Error Message Improvement in `mcp-backlog-server`:**
    -   Modified `mcp-backlog-server/src/error.rs` to suggest using `get_version_milestone_list` tool in `MilestoneNotFoundByName` error message.
-   **`mcp-backlog-server` crate changes for `get_issues_by_milestone_name` tool (further prior):**
    -   Defined `GetIssuesByMilestoneNameRequest` struct.
    -   Implemented initial `get_issues_by_milestone_name_impl` helper.
    -   Added `get_issues_by_milestone_name` tool method.
    -   Added `MilestoneNotFoundByName` error variant.
    -   Added `From<ProjectId/ProjectKey> for ProjectIdOrKey` in `backlog-core`.
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
