# Backlog API Client (Rust)

This project provides a comprehensive Rust client library, command-line interface (CLI), and Model Context Protocol (MCP) server for interacting with the Backlog API.

## Key Features

- **Type Safety**: Strongly-typed identifiers and domain-specific enums throughout
- **Unified File Downloads**: Intelligent format detection for attachments and shared files
- **MCP Integration**: AI-friendly tools via Model Context Protocol server
- **CLI Tool**: Command-line interface for common Backlog operations

## Project Structure

The workspace follows a clear separation between deliverables and internal libraries:

```
cli/                        # Main CLI application and library facade
backlog-mcp-server/         # MCP server implementation  
crates/                     # Internal library crates
├── backlog-core/           # Core types and identifiers shared across all modules
├── backlog-api-core/       # Common API utilities and error types
├── backlog-domain-models/  # Shared domain models (Priority, Status, Category, etc.)
├── backlog-issue/          # Issue management API
├── backlog-project/        # Project management API
├── backlog-space/          # Space management API
├── backlog-user/           # User management API
├── backlog-document/       # Document/Wiki API
├── backlog-git/            # Git repository API
├── backlog-file/           # Shared file API
└── client/                 # Generic HTTP client wrapper
```

### Applications

#### CLI (`cli/`)
Command-line interface for Backlog API operations. The `blg` binary provides a user-friendly way to interact with Backlog from the terminal.

#### MCP Server (`backlog-mcp-server/`)
Model Context Protocol server that exposes Backlog API functionalities as AI-friendly tools with unified file download capabilities.

### Internal Libraries (`crates/`)

#### Main Library
- **`backlog-api-client/`**: The primary library crate that aggregates all API modules and provides a unified client interface.

#### Core Libraries
- **`backlog-core/`**: Defines fundamental data structures, newtype identifiers (e.g., `ProjectId`, `IssueKey`, `SharedFileId`), and shared enums (`FileType`, etc.).
- **`backlog-api-core/`**: Provides core utilities shared across API client modules, such as common error types and result aliases.
- **`backlog-domain-models/`**: Contains shared domain models (e.g., `Priority`, `Resolution`, `Status`, `Category`, `IssueType`, `Milestone`).
- **`client/`**: A foundational crate providing a generic HTTP client wrapper (around `reqwest`) and shared test utilities.

#### API Domain Modules
- **`backlog-document/`**: Client module for Backlog's Document API endpoints.
- **`backlog-file/`**: Client module for Backlog's Shared File API endpoints (listing and downloading shared files).
- **`backlog-git/`**: Client module for Backlog's Git repository and Pull Request API endpoints.
- **`backlog-issue/`**: Client module for Backlog's Issue API endpoints (including comments and attachments).
- **`backlog-project/`**: Client module for Backlog's Project API endpoints (including statuses, categories, etc.).
- **`backlog-space/`**: Client module for Backlog's Space API endpoints.
- **`backlog-user/`**: Client module for Backlog's User API endpoints.

## Feature Flags

The library uses Cargo feature flags to enable specific API modules and functionalities:

### API Module Features
- **`issue`**: Enable Issue API support (comments, attachments)
- **`project`**: Enable Project API support (categories, statuses, milestones)
- **`space`**: Enable Space API support
- **`user`**: Enable User API support
- **`document`**: Enable Document/Wiki API support
- **`git`**: Enable Git repository and Pull Request API support
- **`file`**: Enable Shared File API support

### Writable Features
By default, only read operations are enabled. To enable write operations (create, update, delete), use the corresponding `*_writable` features:
- **`issue_writable`**: Enable write operations for issues (add, update, delete issues and comments)
- **`project_writable`**: Enable write operations for projects (add, update, delete categories, statuses, versions, issue types)
- **`git_writable`**: Enable write operations for Git/PR (add comments, update pull requests)
- **`all_writable`**: Enable all write operations

### Additional Features
- **`schemars`**: Enable JSON Schema generation support (useful for MCP server)

### Example Usage

```bash
# Build CLI with default features (read-only operations)
cargo build --package blg

# Build CLI with write operations
cargo build --package blg --features "all_writable"

# Build MCP server (has issue_writable by default)
cargo build --package mcp-backlog-server

# Use the library in your own project
# Add to Cargo.toml:
# backlog-api-client = { path = "path/to/crates/backlog-api-client" }
```

## Building and Testing

To build all crates and run tests, you can use the standard Cargo commands from the workspace root:

```bash
cargo check --all-targets --all-features
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings 
cargo fmt --all
```

For specific instructions on building and running the `blg` CLI or the MCP server, please refer to the README files within their respective directories (`cli/README.md` and `backlog-mcp-server/README.md`).
