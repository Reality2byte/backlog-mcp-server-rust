# Backlog API Client (Rust)

This project provides a comprehensive Rust client library, command-line interface (CLI), and Model Context Protocol (MCP) server for interacting with the Backlog API.

## Key Features

- **Type Safety**: Strongly-typed identifiers and domain-specific enums throughout
- **Comprehensive API Coverage**: 78+ API endpoints across 8 domain modules
- **Unified File Downloads**: Intelligent format detection for attachments and shared files
- **Write Operations Support**: Create, update, and delete operations with feature flags
- **MCP Integration**: AI-friendly tools via Model Context Protocol server
- **CLI Tool**: Full-featured command-line interface for Backlog operations
- **Test-Driven Development**: Comprehensive test coverage with 250+ tests

## Project Structure

The workspace follows a clear separation between deliverables and internal libraries:

```
cli/                        # CLI application (`blg` binary)
backlog-mcp-server/         # Model Context Protocol server for AI integration
crates/                     # Internal library crates
├── backlog-api-client/     # Main library facade (aggregates all API modules)
├── backlog-core/           # Core types and identifiers shared across all modules
├── backlog-api-core/       # Common API utilities and error types
├── backlog-api-macros/     # Procedural macros for API parameter serialization
├── backlog-domain-models/  # Shared domain models (Priority, Status, Category, etc.)
├── backlog-issue/          # Issue management API
├── backlog-project/        # Project management API
├── backlog-space/          # Space management API
├── backlog-user/           # User management API
├── backlog-document/       # Document API
├── backlog-wiki/           # Wiki API (full CRUD operations and file attachment)
├── backlog-git/            # Git repository and Pull Request API
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
- **`backlog-api-macros/`**: Procedural macros for API parameter serialization

#### API Domain Modules
- **`backlog-document/`**: Document API endpoints (4 endpoints) - document tree navigation and attachment downloads.
- **`backlog-file/`**: Shared File API endpoints (2 endpoints) - project file management with type-safe directory/file distinction.
- **`backlog-git/`**: Git repository and Pull Request API endpoints (16 endpoints) - complete Git workflow including PR management.
- **`backlog-issue/`**: Issue management API endpoints (14 endpoints) - comprehensive issue lifecycle and shared file linking.
- **`backlog-project/`**: Project management API endpoints (22 endpoints) - most extensive API covering categories, statuses, versions, issue types.
- **`backlog-space/`**: Space API endpoints (2 endpoints) - basic space information.
- **`backlog-user/`**: User management API endpoints (4 endpoints) - user information and icons.
- **`backlog-wiki/`**: Wiki API endpoints (8 endpoints) - wiki pages with full CRUD operations, attachment management, and file attachment capabilities.

## Feature Flags

The library uses Cargo feature flags to enable specific API modules and functionalities:

### API Module Features
- **`issue`**: Enable Issue API support (comments, attachments)
- **`project`**: Enable Project API support (categories, statuses, milestones)
- **`space`**: Enable Space API support
- **`user`**: Enable User API support
- **`document`**: Enable Document API support
- **`wiki`**: Enable Wiki API support
- **`git`**: Enable Git repository and Pull Request API support
- **`file`**: Enable Shared File API support

### Writable Features
By default, only read operations are enabled. To enable write operations (create, update, delete), use the corresponding `*_writable` features:
- **`issue_writable`**: Enable write operations for issues (add, update, delete issues and comments, link shared files)
- **`project_writable`**: Enable write operations for projects (add, update, delete categories, statuses, versions, issue types)
- **`git_writable`**: Enable write operations for Git/PR (add, update pull requests and comments, delete attachments)
- **`wiki_writable`**: Enable write operations for wikis (update wiki pages with name, content, and email notifications)
- **`space_writable`**: Enable write operations for space (planned feature)
- **`user_writable`**: Enable write operations for users (planned feature)
- **`all_writable`**: Enable all write operations

### Additional Features
- **`schemars`**: Enable JSON Schema generation support (useful for MCP server)

## API Implementation Status

### Comprehensive API Coverage
The project implements **78+ API endpoints** across 8 domain modules with varying levels of completeness:

| Domain | Endpoints | Read Ops | Write Ops | Coverage |
|--------|-----------|----------|-----------|----------|
| **Project** | 22 | ✅ Complete | ✅ Full CRUD | 🟢 Extensive |
| **Git/PR** | 16 | ✅ Complete | ✅ Full CRUD | 🟢 Complete |
| **Issue** | 14 | ✅ Complete | ✅ Full CRUD | 🟢 Complete |
| **Wiki** | 8 | ✅ Complete | ✅ Create/Update/Delete | 🟢 Extensive |
| **Document** | 4 | ✅ Complete | (Read-only API) | 🟢 Read-only |
| **User** | 4 | ✅ Complete | ❌ Planned | 🟡 Read-only |
| **File** | 2 | ✅ Complete | (Read-only API) | 🟢 Complete |
| **Space** | 2 | ✅ Complete | ❌ Planned | 🟡 Read-only |

### Advanced Features
- **Shared File Integration**: Issues can link to project shared files with type-safe APIs
- **Intelligent Downloads**: Automatic format detection (Image/Text/Raw) for all file operations
- **Form-Encoded Writes**: Proper `application/x-www-form-urlencoded` handling for all write operations
- **Unified Error Handling**: Consistent error types across all domains

### Example Usage

```bash
# Build CLI with default features (includes all writable operations)
cargo build --package blg

# Build CLI with specific features only
cargo build --package blg --features "git issue project space wiki"

# Build CLI with specific writable operations
cargo build --package blg --features "git git_writable issue issue_writable project project_writable wiki wiki_writable"

# Build MCP server (includes issue_writable, git_writable, wiki_writable by default)
cargo build --package mcp-backlog-server

# Use the library in your own project
# Add to Cargo.toml:
# backlog-api-client = { path = "path/to/crates/backlog-api-client" }
```

### Custom Fields

The Backlog API client provides comprehensive support for custom fields in issues:

#### Library Usage

```rust
use backlog_api_client::{BacklogApiClient, AddIssueParamsBuilder};
use backlog_issue::models::CustomFieldInput;
use backlog_core::identifier::{CustomFieldId, ProjectId};
use std::collections::HashMap;
use chrono::NaiveDate;

// Create custom fields map
let mut custom_fields = HashMap::new();

// Text field
custom_fields.insert(
    CustomFieldId::new(1),
    CustomFieldInput::Text("Sample text".to_string())
);

// Date field
custom_fields.insert(
    CustomFieldId::new(2),
    CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap())
);

// Single selection list
custom_fields.insert(
    CustomFieldId::new(3),
    CustomFieldInput::SingleList {
        id: 100,
        other_value: Some("Additional info".to_string())
    }
);

// Create issue with custom fields
let params = AddIssueParamsBuilder::default()
    .project_id(ProjectId::new(1))
    .summary("Issue with custom fields")
    .custom_fields(custom_fields)
    .build()?;

let issue = client.issue().add_issue(params).await?;
```

#### CLI Usage

```bash
# Create issue with custom fields using individual arguments
blg issue create --project-id 1 --summary "Test Issue" \
  --custom-field "1:text:Sample text" \
  --custom-field "2:date:2024-06-24" \
  --custom-field "3:single_list:100:Other description"

# Create issue with custom fields using JSON file
blg issue create --project-id 1 --summary "Test Issue" \
  --custom-fields-json custom_fields.json

# Update issue with custom fields
blg issue update TEST-123 \
  --custom-field "1:text:Updated text" \
  --custom-field "4:numeric:123.45"
```

#### JSON Format for Custom Fields

Create a `custom_fields.json` file:

```json
{
  "1": {"type": "text", "value": "Sample text"},
  "2": {"type": "numeric", "value": 123.45},
  "3": {"type": "date", "value": "2024-06-24"},
  "4": {"type": "single_list", "id": 100, "other_value": "Other"},
  "5": {"type": "multiple_list", "ids": [100, 200], "other_value": "Other"},
  "6": {"type": "checkbox", "ids": [10, 20, 30]},
  "7": {"type": "radio", "id": 400, "other_value": "Other"}
}
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

## Architecture Highlights

### Type Safety & Domain Design
- **Strongly-typed identifiers**: `ProjectId`, `IssueKey`, `SharedFileId`, `WikiId`, etc.
- **Domain separation**: Each API domain is its own crate with clear boundaries
- **Shared models**: Common domain models centralized to avoid duplication

### Test-Driven Development
- **250+ comprehensive tests** covering success, error, and edge cases
- **Mock-based testing** using `wiremock` for reliable unit tests
- **Integration testing** with real Backlog API instances

### File Management Innovation
- **Unified download system**: Single API for all file types with automatic format detection
- **Content-type analysis**: Intelligent Image/Text/Raw classification
- **Base64 handling**: Proper encoding for JSON responses containing binary data

### AI Integration (MCP Server)
- **30+ AI-friendly tools** for comprehensive Backlog automation
- **JSON Schema**: Full parameter validation and documentation
- **Writable by default**: Enables AI agents to perform actions, not just queries

This project represents a mature, production-ready Backlog API ecosystem suitable for both direct integration and AI-powered automation workflows.
