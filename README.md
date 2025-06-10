# Backlog API Client (Rust)

This project provides a comprehensive Rust client library, command-line interface (CLI), and Model Context Protocol (MCP) server for interacting with the Backlog API.

## Project Structure

Below is an overview of the main directories within this workspace:

-   `backlog-api-client/`: Contains the primary library crate that aggregates all API modules and the source code for the `blg` CLI tool.
-   `backlog-api-core/`: Provides core utilities shared across API client modules, such as common error types and result aliases.
-   `backlog-core/`: Defines fundamental data structures, newtype identifiers (e.g., `ProjectId`, `IssueKey`, `PullRequestNumber`), and shared enums used throughout the Backlog API client ecosystem.
-   `backlog-document/`: Implements the client module for interacting with Backlog's Document API endpoints.
-   `backlog-git/`: Implements the client module for interacting with Backlog's Git repository and Pull Request API endpoints.
-   `backlog-issue/`: Implements the client module for interacting with Backlog's Issue API endpoints (including comments and attachments).
-   `backlog-project/`: Implements the client module for interacting with Backlog's Project API endpoints (including statuses, categories, etc.).
-   `backlog-space/`: Implements the client module for interacting with Backlog's Space API endpoints.
-   `backlog-user/`: Implements the client module for interacting with Backlog's User API endpoints.
-   `client/`: A foundational crate providing a generic HTTP client wrapper (around `reqwest`) and shared test utilities.
-   `mcp-backlog-server/`: Implements an MCP server that exposes various Backlog API functionalities as tools for MCP-compatible clients.
-   `memory-bank/`: Contains Markdown documents used by Cline (the AI assistant) to maintain context and knowledge about this project.

## Building and Testing

To build all crates and run tests, you can use the standard Cargo commands from the workspace root:

```bash
cargo check --all-targets --all-features
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings 
cargo fmt --all
```

For specific instructions on building and running the `blg` CLI or the `mcp-backlog-server`, please refer to the README files within their respective directories (`backlog-api-client/README.md` and `mcp-backlog-server/README.md`).
