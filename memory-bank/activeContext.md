## Current Work Focus
-   Refactoring `mcp-backlog-server` to organize tool helper modules into a `tools/` subdirectory.
-   Updating Memory Bank to reflect this refactoring.

## Recent Changes
-   **`mcp-backlog-server` Refactoring:**
    -   Created `mcp-backlog-server/src/tools/` directory.
    -   Moved `document.rs`, `issue.rs` into `src/tools/`.
    -   Moved and renamed `git_tools.rs` to `src/tools/git.rs`.
    -   Created `mcp-backlog-server/src/tools/mod.rs` to declare these modules.
    -   Updated `mcp-backlog-server/src/lib.rs` to use `pub mod tools;`.
    -   Updated `mcp-backlog-server/src/server.rs` to use new module paths (e.g., `use crate::tools::git;`, `git::get_repository_list_impl`).
    -   Emptied original module files at `mcp-backlog-server/src/`.
-   **Previous Task (CLI Issue Commands):**
    -   Added `issue list` and `issue show` commands to the `blg` CLI tool.
    -   Updated `backlog-issue` API for `get_issue` to accept `IssueIdOrKey`.
    -   Updated relevant Memory Bank files.

## Next Steps
-   Update `progress.md` to reflect the `mcp-backlog-server` refactoring.
-   Confirm completion of the refactoring task with the user.


## Active Decisions & Considerations
-   The project is a Rust workspace for a Backlog API client, CLI, and MCP server.
-   The modular structure (crates for core, issue, project, git, etc.) is a key architectural pattern. Each API module crate (`backlog-issue`, `backlog-git`, etc.) now typically exposes a `Handler` struct (e.g., `IssueApi`, `GitHandler`) that takes a `client::Client` instance.
-   The `client` crate is a generic HTTP client, while `backlog-api-client` (library) is the specific Backlog API orchestrator.
-   CLI tool (`blg`) uses `clap` for argument parsing.
-   MCP server tools are implemented as methods on `Server`. Helper functions for these tools are now organized under `mcp-backlog-server/src/tools/`.

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
