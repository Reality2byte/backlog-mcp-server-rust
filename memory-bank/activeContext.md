## Current Work Focus
-   Implemented functionality to view Git Repositories and Pull Requests from Backlog.
-   Updating Memory Bank to reflect these new features.

## Recent Changes
-   **Git and Pull Request Feature Implementation:**
    -   Created new crate `backlog-git` for Git/PR API interactions.
        -   Defined data models: `Repository`, `PullRequest`, `PullRequestStatus`, `IssueLink`.
        -   Implemented `GitHandler` with methods: `list_repositories`, `get_repository`, `list_pull_requests`, `get_pull_request`.
        -   Added `schemars` for `JsonSchema` derivation (with `chrono` feature) to models.
    -   Updated `backlog-core` to support `JsonSchema` for `User`, `UserId`, `Role`, `Language`.
    -   Integrated `backlog-git` into `backlog-api-client` library:
        -   Added `git` feature and dependency.
        -   Added `git()` method to `BacklogApiClient`.
    -   Enhanced `blg` CLI tool (`backlog-api-client/src/bin/blg.rs`):
        -   Added `clap` dependency and `cli` feature.
        -   Implemented subcommands: `repo list`, `repo show`, `pr list`, `pr show`.
        -   Corrected `ProjectIdOrKey` parsing using `.parse()`.
    -   Enhanced `mcp-backlog-server`:
        -   Enabled `git` feature for `backlog-api-client` dependency.
        -   Added `backlog-git` as a direct dependency for its models.
        -   Created `git_tools.rs` module for new tool request/response structs and helper implementations.
        -   Added MCP tools: `get_repository_list`, `get_repository_details`, `list_pull_requests`, `get_pull_request_details`.
        -   Ensured consistent client handling (`Arc<Mutex<...>>` and internal locking) in helper functions.
        -   Corrected `McpError` construction in `git_tools.rs`.
-   **Previous Work (Code Quality & MCP Tooling):**
    -   Addressed Clippy warnings and formatted the codebase.
    -   Implemented/improved various MCP tools in `mcp-backlog-server` for issues and milestones.


## Next Steps
-   Update `progress.md` to reflect the new Git/PR features.
-   Update `API.md` to mark implemented Git/PR API endpoints.
-   Add Rustdoc comments to new public items in `backlog-git`.
-   Consider adding tests for the new functionality.
-   Confirm completion of the Git/PR feature implementation task with the user.


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
