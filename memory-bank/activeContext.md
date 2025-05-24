# Active Context

## Current Work Focus
-   Completed writing mock server tests for the `get_issue_list` functionality using `wiremock`.

## Recent Changes
-   Read all core Memory Bank files.
-   Updated `activeContext.md` and `progress.md` for the `get_issue_list` mock tests task.
-   Added `wiremock`, `tokio` (with features), and `serde_json` to `[dev-dependencies]` in `backlog-issue/Cargo.toml`.
-   Added a `tests` module to `backlog-issue/src/api/mod.rs`.
-   Implemented initial mock tests for `get_issue_list`, including:
    -   Successful retrieval with empty parameters.
    -   Successful retrieval with `projectId` query parameter.
    -   Handling of a server error (500).

## Next Steps
-   Task complete. Awaiting review or next task.
-   (Future considerations: Expand test coverage for more query parameters and edge cases).

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
