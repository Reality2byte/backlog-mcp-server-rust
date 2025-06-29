# CLAUDE.md

This file provides guidance to Claude Code when working with code in this repository.

## Architecture Overview

Rust workspace for Backlog API client ecosystem:
- **Library** (`crates/backlog-api-client/`): Core client library with modular API wrappers
- **CLI** (`cli/`): Command-line interface built on the library
- **MCP Server** (`backlog-mcp-server/`): Model Context Protocol server for AI integration

### Workspace Structure
```
cli/                        # CLI binary application
backlog-mcp-server/         # MCP server implementation
crates/
├── backlog-api-client/     # Main library facade
├── backlog-api-macros/     # Procedural macros for API parameter serialization
├── backlog-core/           # Core types and identifiers
├── backlog-api-core/       # Common API utilities and error types
├── backlog-domain-models/  # Shared domain models
├── backlog-issue/          # Issue management API
├── backlog-project/        # Project management API
├── backlog-space/          # Space management API
├── backlog-user/           # User management API
├── backlog-document/       # Document API
├── backlog-wiki/           # Wiki API
├── backlog-git/            # Git repository API
├── backlog-file/           # Shared file API
└── client/                 # Generic HTTP client wrapper
```

## Quick Start

### Environment Setup
```bash
export BACKLOG_BASE_URL="https://your-space.backlog.jp"
export BACKLOG_API_KEY="your_api_key"
```

### Development Commands
```bash
# Build and test
cargo check --all-targets --all-features
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --all

# Build variants
cargo build --package blg                           # Read-only CLI
cargo build --package blg --features "all_writable" # Full CLI
cargo build --package mcp-backlog-server           # MCP server
```

## Key Design Patterns

### Date Handling
| Context | Type | Format | Example |
|---------|------|--------|---------|
| Request parameters | `ApiDate` | yyyy-MM-dd | `start_date`, `due_date` |
| Response timestamps | `DateTime<Utc>` | ISO 8601 | `created`, `updated` |
| Legacy fields | `String` | varies | `Issue.start_date` |

### Form Encoding
Array parameters require special `foo[]` syntax and manual serialization:
```rust
// Manual form encoding
impl From<&AddCommentParams> for Vec<(String, String)> {
    fn from(params: &AddCommentParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("content".to_string(), params.content.clone()));
        
        if let Some(user_ids) = &params.notified_user_ids {
            user_ids.iter().for_each(|id| {
                seq.push(("notifiedUserId[]".to_string(), id.to_string()));
            });
        }
        seq
    }
}
```

### ToFormParams Macro
Automates form parameter serialization:
```rust
use backlog_api_macros::ToFormParams;

#[derive(ToFormParams)]
struct AddCommentParams {
    content: String,                           // → "content"
    #[form(array, name = "notifiedUserId")]
    notified_user_ids: Option<Vec<u32>>,      // → "notifiedUserId[]"
    #[form(skip)]
    issue_id_or_key: IssueIdOrKey,            // Skipped
}
```

### Domain Crate Structure
Standard template for all domain crates:
```
crates/backlog-{domain}/
├── src/
│   ├── lib.rs                    # Public exports
│   ├── models.rs                 # Domain models
│   └── api/
│       ├── mod.rs                # Module exports
│       ├── {domain}_api.rs       # Main API struct
│       ├── get_*.rs              # Read operations
│       ├── add_*.rs              # Create operations (feature-gated)
│       ├── update_*.rs           # Update operations (feature-gated)
│       └── delete_*.rs           # Delete operations (feature-gated)
└── tests/
    ├── common/mod.rs             # Shared test utilities
    ├── {domain}_api_test.rs      # Read-only tests
    └── {domain}_writable_test.rs # Write operation tests
```

### API Implementation Pattern
```rust
// Parameter struct
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateEntityParams {
    pub required_field: String,
    pub optional_field: Option<String>,
}

// Form serialization
#[cfg(feature = "writable")]
impl From<&UpdateEntityParams> for Vec<(String, String)> {
    fn from(params: &UpdateEntityParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("requiredField".to_string(), params.required_field.clone()));
        
        if let Some(value) = &params.optional_field {
            seq.push(("optionalField".to_string(), value.clone()));
        }
        seq
    }
}

// API method
#[cfg(feature = "writable")]
pub async fn update_entity(
    &self,
    project_id_or_key: impl Into<ProjectIdOrKey>,
    entity_id: impl Into<EntityId>,
    params: &UpdateEntityParams,
) -> Result<Entity> {
    let params_vec: Vec<(String, String)> = params.into();
    let path = format!("/api/v2/projects/{}/entities/{}", 
        project_id_or_key.into(), entity_id.into());
    self.client.patch(&path, &params_vec).await
}
```

### Custom Fields
Type-safe handling with dynamic form parameter names:
```rust
// Response type (reading)
pub enum CustomFieldValue {
    Text(String),
    Numeric(f64),
    SingleList { item: CustomFieldListItem, other_value: Option<String> },
    MultipleList { items: Vec<CustomFieldListItem>, other_value: Option<String> },
    // ...
}

// Request type (writing)
pub enum CustomFieldInput {
    Text(String),
    Numeric(f64),
    SingleList { id: u32, other_value: Option<String> },
    MultipleList { ids: Vec<u32>, other_value: Option<String> },
    // ...
}

// Form serialization
params.push((format!("customField_{}", id.value()), value));
```

## Development Process

### TDD Workflow
1. **Research**: Read official Backlog API docs
2. **Test First**: Write comprehensive unit tests
3. **Implement**: Minimal implementation to pass tests
4. **CLI**: Add commands when applicable
5. **Integration**: Test with real Backlog instance
6. **Document**: Update API.md, README.md, CLAUDE.md

### Testing Requirements
- Test success, error, and edge cases
- Mock different HTTP status codes
- Test minimal and maximal parameter sets
- Include integration tests when possible

### Pre-commit Checklist
```bash
cargo check --all-targets --all-features
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --all
```

## Important Guidelines

### Feature Flags
- Read operations: Always available
- Write operations: Behind `writable` feature flags
- Common combinations:
  - `--features "cli git issue project space"` (read-only)
  - `--features "cli git issue project project_writable space"` (with writes)
  - `--all-features` (development)

### Error Handling
- Use `ApiError` from `backlog-api-core`
- Domain-specific errors wrap core types
- MCP server converts to `rmcp::Error`

### Type Safety
- Use strongly-typed identifiers (`ProjectId`, `IssueKey`, etc.)
- Use `XxxResponse` type aliases for API responses
- Custom deserialization with `Raw*` structs for complex JSON

### Documentation
- API methods must include endpoint mapping
- Example: `/// Corresponds to \`GET /api/v2/projects/:projectIdOrKey\`.`
- Update API.md counts after adding endpoints

## Recent Updates
- Custom field implementation with type-safe handling
- ToFormParams macro for automated form serialization
- Unified file download system in MCP server
- Comprehensive wiki API support (create, delete, attach files)
- Issue comment update functionality
- User management in MCP server

## MCP Server
- Domain modules: `issue/`, `git/`, `document/`, `project/`, `file/`, `user/`, `wiki/`
- Each module has `request.rs` and `bridge.rs`
- Project access control via `BACKLOG_PROJECTS` environment variable
- Unified file handling with intelligent format detection