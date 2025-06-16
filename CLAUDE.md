# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture Overview

This is a Rust workspace providing a comprehensive Backlog API client ecosystem with three main components:
- **Library** (`backlog-api-client`): Core client library with modular API wrappers
- **CLI** (`blg`): Command-line tool for Backlog API interactions
- **MCP Server** (`mcp-backlog-server`): Model Context Protocol server for AI integration

### Workspace Structure

```
backlog-core/                 # Core types and identifiers shared across all modules
backlog-api-core/            # Common API utilities and error types
backlog-{issue,project,space,user,document,git}/ # Domain-specific API modules
client/                      # Generic HTTP client wrapper
backlog-api-client/          # Main library facade + CLI binary
mcp-backlog-server/         # MCP server implementation
```

## Development Commands

### Build and Test
```bash
# Build all crates
cargo check --all-targets --all-features
cargo test --all-features --all-targets

# Lint and format
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --all

# Build CLI tool
cargo build --package backlog-api-client --features "cli git issue" --bin blg

# Build MCP server
cargo build --package mcp-backlog-server
```

### Environment Setup
```bash
export BACKLOG_BASE_URL="https://your-space.backlog.jp"
export BACKLOG_API_KEY="your_api_key"
```

## Key Design Patterns

### Modular API Structure
- Each API domain has its own crate (e.g., `backlog-issue`, `backlog-project`)
- Domain crates depend on `client` for HTTP operations and `backlog-core` for shared types
- Cross-domain dependencies exist where needed (e.g., `backlog-issue` uses `backlog-project::Status`)

### Error Handling
- Unified error handling via `ApiError` in `backlog-api-core`
- Domain-specific errors wrap the core error types
- MCP server has its own error type that converts to `rmcp::Error`

### Type Safety
- Strongly-typed identifiers (e.g., `ProjectId`, `IssueKey`, `AttachmentId`) in `backlog-core`
- Builder pattern for complex request parameters
- Custom deserialization for complex JSON structures using temporary `Raw*` structs

### MCP Server Organization
- Tools organized by domain modules (`issue/`, `git/`, `document/`, `project/`)
- Each module contains:
  - `request.rs`: Input structs with JSON schema derivation
  - `bridge.rs`: Core logic functions
- Main `server.rs` registers tools with `#[tool]` attributes

## Important Implementation Details

### Custom Deserialization Pattern
For complex JSON where field types depend on other field values:
1. Define `Raw*` struct with `serde_json::Value` for ambiguous fields
2. Manual `impl Deserialize` that first deserializes to `Raw*`
3. Use type-discriminating field to construct strongly-typed final struct
4. Example: `CustomFieldType` handles different settings based on `typeId`

### Cross-Domain Dependencies
- `backlog-issue` depends on `backlog-project` for `Status` type
- This establishes semantic relationships between domains while maintaining modularity

### Test Utilities
- `client` crate provides shared test utilities via `test-utils` feature
- Use `setup_client()` for consistent test client initialization

### Feature Flags
- CLI requires `"cli git issue"` features
- MCP server has `issue_writable` feature (enabled by default)
- Use `--no-default-features` to disable optional functionality