# Backlog API Client (Rust)

This project provides a comprehensive Rust client library, command-line interface (CLI), and Model Context Protocol (MCP) server for interacting with the Backlog API.

## Key Features

- **Type Safety**: Strongly-typed identifiers and domain-specific enums throughout
- **Unified File Downloads**: Intelligent format detection for attachments and shared files
- **MCP Integration**: AI-friendly tools via Model Context Protocol server
- **CLI Tool**: Command-line interface for common Backlog operations

## Project Structure

Below is an overview of the main directories within this workspace:

### Core Libraries
-   `backlog-api-client/`: Contains the primary library crate that aggregates all API modules and the source code for the `blg` CLI tool.
-   `backlog-api-core/`: Provides core utilities shared across API client modules, such as common error types and result aliases.
-   `backlog-core/`: Defines fundamental data structures, newtype identifiers (e.g., `ProjectId`, `IssueKey`, `SharedFileId`), and shared enums (`FileType`, etc.) used throughout the ecosystem.
-   `client/`: A foundational crate providing a generic HTTP client wrapper (around `reqwest`) and shared test utilities.

### API Domain Modules
-   `backlog-document/`: Client module for Backlog's Document API endpoints.
-   `backlog-file/`: Client module for Backlog's Shared File API endpoints (listing and downloading shared files).
-   `backlog-git/`: Client module for Backlog's Git repository and Pull Request API endpoints.
-   `backlog-issue/`: Client module for Backlog's Issue API endpoints (including comments and attachments).
-   `backlog-project/`: Client module for Backlog's Project API endpoints (including statuses, categories, etc.).
-   `backlog-space/`: Client module for Backlog's Space API endpoints.
-   `backlog-user/`: Client module for Backlog's User API endpoints.

### Applications
-   `mcp-backlog-server/`: MCP server that exposes Backlog API functionalities as AI-friendly tools with unified file download capabilities.
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
