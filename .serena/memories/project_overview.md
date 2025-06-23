# Project Overview

This is a Rust workspace providing a comprehensive Backlog API client ecosystem with three main components:
- **Library** (`crates/backlog-api-client/`): Core client library with modular API wrappers
- **CLI** (`cli/`): Command-line interface built on top of the library  
- **MCP Server** (`backlog-mcp-server/`): Model Context Protocol server for AI integration

## Main Crates
- backlog-api-client: Main library facade
- backlog-core: Core types and identifiers
- backlog-api-core: Common API utilities
- backlog-domain-models: Shared domain models
- backlog-issue, backlog-project, backlog-space, backlog-user, backlog-wiki, backlog-git, backlog-document, backlog-file: API domain crates
- client: Generic HTTP client wrapper

## Technology Stack
- Rust with Cargo workspace
- HTTP client for API communication
- MCP (Model Context Protocol) integration