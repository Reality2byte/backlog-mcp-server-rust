## Current Work Focus
-   Adding issue-related commands (list, show) to the `blg` CLI tool.
-   Updating Memory Bank to reflect these CLI enhancements.

## Recent Changes
-   **CLI Enhancements for Issues (`blg` tool):**
    -   Added `Issue` subcommand to `blg.rs` with `List` and `Show` actions.
    -   Defined `IssueListCliParams` for `issue list` command, supporting filters like project ID, assignee ID, status ID, keyword, and count.
    -   Implemented logic to parse CLI arguments, build `GetIssueListParams` (from `backlog-issue` crate), and call `client.issue().get_issue_list()`.
    -   Implemented logic for `issue show` to parse `IssueIdOrKey` and call `client.issue().get_issue()`.
    -   Updated `backlog-api-client/Cargo.toml` to include `issue` in `required-features` for the `blg` binary.
-   **Previous Task (Git/PR Feature Implementation):**
    -   Successfully implemented and documented features for viewing Git Repositories and Pull Requests across the library, CLI (`blg repo/pr` subcommands), and MCP server. This involved creating the `backlog-git` crate, updating `backlog-core` for `JsonSchema`, and integrating these into `backlog-api-client` and `mcp-backlog-server`. Memory Bank files were updated accordingly.

## Next Steps
-   Update `progress.md` to reflect the new CLI issue commands.
-   Consider adding more filter options to `blg issue list` if requested.
-   Consider adding other issue commands (`create`, `update`, `delete`) to `blg` if requested.
-   Add tests for the new CLI commands.
-   Confirm completion of the "add issue commands to blg.rs" task with the user.


## Active Decisions & Considerations
-   The project is a Rust workspace for a Backlog API client, CLI, and MCP server.
-   The modular structure (crates for core, issue, project, git, etc.) is a key architectural pattern. Each API module crate (`backlog-issue`, `backlog-git`, etc.) now typically exposes a `Handler` struct (e.g., `IssueApi`, `GitHandler`) that takes a `client::Client` instance.
-   The `client` crate is a generic HTTP client, while `backlog-api-client` (library) is the specific Backlog API orchestrator.
-   CLI tool (`blg`) uses `clap` for argument parsing.
-   MCP server tools are implemented as methods on `Server`, calling helper functions that manage `BacklogApiClient` interaction.

## Important Patterns & Preferences
-   The project follows standard Rust project structure and conventions.
-   Use of workspace for managing multiple related crates.
-   Feature flags for optional modules in `backlog-api-client`.
-   `schemars` for `JsonSchema` derivation for MCP tool inputs/outputs.
-   Consistent error handling patterns using `thiserror` and `rmcp::Error`.

## Learnings & Project Insights
-   The project is well-structured, facilitating the addition of new modules like `backlog-git`.
-   The CLI (`blg`) has evolved from a testbed to a more structured `clap`-based application.
-   Core data types in `backlog-core` are increasingly shared and require updates (e.g., `JsonSchema` derives) as new features are added.
-   Careful attention to client handling (e.g., `Arc<Mutex<...>>` vs. direct reference) is needed between different layers (MCP server methods vs. helper impl functions).
