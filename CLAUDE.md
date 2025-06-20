# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture Overview

This is a Rust workspace providing a comprehensive Backlog API client ecosystem with three main components:
- **Library** (`crates/backlog-api-client/`): Core client library with modular API wrappers
- **CLI** (`cli/`): Command-line interface built on top of the library
- **MCP Server** (`backlog-mcp-server/`): Model Context Protocol server for AI integration

### Workspace Structure

```
cli/                        # CLI binary application
backlog-mcp-server/         # MCP server implementation
crates/                     # Internal library crates
├── backlog-api-client/     # Main library facade (aggregates all API modules)
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

## Development Commands

### Build and Test
```bash
# Build all crates
cargo check --all-targets --all-features
cargo test --all-features --all-targets

# Lint and format
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --all

# Build CLI tool (read-only operations)
cargo build --package blg

# Build CLI tool with write operations
cargo build --package blg --features "all_writable"

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
- Domain crates depend on `client` for HTTP operations, `backlog-core` for shared types, and `backlog-domain-models` for shared domain models
- Shared domain models (Priority, Status, Category, etc.) are centralized in `backlog-domain-models` to avoid duplication and circular dependencies

### Error Handling
- Unified error handling via `ApiError` in `backlog-api-core`
- Domain-specific errors wrap the core error types
- MCP server has its own error type that converts to `rmcp::Error`

### Type Safety
- Strongly-typed identifiers (e.g., `ProjectId`, `IssueKey`, `AttachmentId`, `SharedFileId`) in `backlog-core`
- Strongly-typed enums for domain concepts (e.g., `FileType` for shared file types)
- Builder pattern for complex request parameters
- Custom deserialization for complex JSON structures using temporary `Raw*` structs

### Form Encoding for Array Parameters
When implementing writable APIs that require form-encoded data with array parameters (e.g., `foo[]`), follow this pattern:

1. **Parameter Structure**: Define request parameters without `serde::Serialize`
2. **Manual Serialization**: Implement `From<&ParamsStruct> for Vec<(String, String)>`
3. **Array Handling**: Use array syntax (e.g., `"notifiedUserId[]"`) for array parameters
4. **API Integration**: Convert parameters using `.into()` before passing to `client.post()`

**Example Pattern:**
```rust
// Parameter struct without Serialize
#[derive(Builder, Debug, Clone)]
pub struct AddCommentParams {
    pub content: String,
    #[builder(default)]
    pub notified_user_ids: Option<Vec<u32>>,
}

// Manual serialization for form encoding
impl From<&AddCommentParams> for Vec<(String, String)> {
    fn from(params: &AddCommentParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("content".to_string(), params.content.clone()));
        
        // Handle array parameters with [] syntax
        if let Some(user_ids) = &params.notified_user_ids {
            user_ids.iter().for_each(|id| {
                seq.push(("notifiedUserId[]".to_string(), id.to_string()));
            });
        }
        seq
    }
}

// API method using the pattern
pub async fn add_comment(&self, params: &AddCommentParams) -> Result<Comment> {
    let params_vec: Vec<(String, String)> = params.into();
    self.client.post("/api/v2/comments", &params_vec).await
}
```

**Why This Pattern:**
- Backlog API expects `application/x-www-form-urlencoded` format for writable operations
- Array parameters require special `foo[]` syntax that serde cannot handle automatically
- Manual serialization provides precise control over parameter formatting
- Consistent with existing patterns in `backlog-issue` and other domain crates

### MCP Server Organization
- Tools organized by domain modules (`issue/`, `git/`, `document/`, `project/`, `file/`)
- Each module contains:
  - `request.rs`: Input structs with JSON schema derivation
  - `bridge.rs`: Core logic functions
- Main `server.rs` registers tools with `#[tool]` attributes
- Unified file handling via `file_utils.rs` with intelligent format detection

## Important Implementation Details

### Custom Deserialization Pattern
For complex JSON where field types depend on other field values:
1. Define `Raw*` struct with `serde_json::Value` for ambiguous fields
2. Manual `impl Deserialize` that first deserializes to `Raw*`
3. Use type-discriminating field to construct strongly-typed final struct
4. Example: `CustomFieldType` handles different settings based on `typeId`

### Domain Model Architecture
- Shared domain models (Priority, Resolution, Status, Category, IssueType, Milestone) are centralized in `backlog-domain-models`
- Domain API crates (`backlog-issue`, `backlog-project`) depend on `backlog-domain-models` for these shared models
- This eliminates circular dependencies and model duplication while maintaining clear domain boundaries
- All domain crates use `backlog-core::FileType` for consistent file type handling

### Unified File Download System (MCP Server)
The MCP server implements a unified file download system that consolidates multiple format-specific tools into single, intelligent tools:

#### Architecture
- **`SerializableFile`**: Core struct handling file content with automatic format detection
- **`FileFormat`**: Enum defining supported formats (`Image`, `Text`, `Raw`)
- **`FormatDetector`**: Intelligent format detection using content-type analysis and UTF-8 validation
- **Integration**: Single download tools replace 12 separate format-specific tools (3 domains × 4 formats)

#### Format Detection Logic
1. **Explicit format override**: Honors user-specified format with validation
2. **Image detection**: Content-type starts with `image/`
3. **Text detection**: Known text content-types or UTF-8 validity + character composition analysis
4. **Raw fallback**: Everything else treated as binary data

#### Benefits
- **Simplified API**: 4 unified tools instead of 12 format-specific tools
- **Automatic handling**: Users don't need to specify format in most cases
- **Type safety**: Proper error handling for format mismatches
- **Extensibility**: Easy to add new formats or detection strategies

### Shared File Support
The system now includes comprehensive shared file support:

#### Core Components
- **`backlog-file`**: New domain crate for shared file operations
- **`SharedFileId`**: Strongly-typed identifier in `backlog-core`
- **`FileType`**: Enum for shared file types (`File`, `Directory`)
- **MCP Integration**: Browse and download shared files via MCP tools

#### API Operations
- **List files**: Browse shared files in project directories with pagination
- **Download files**: Download shared files with the same unified format detection
- **Type safety**: Distinguish between files and directories at the type level

### Test Utilities
- `client` crate provides shared test utilities via `test-utils` feature
- Use `setup_client()` for consistent test client initialization

### Feature Flags
- **CLI Base Features**: `"cli git issue project space"` required for basic functionality
- **Writable Features**: Add `project_writable`, `issue_writable` for write operations (create/update/delete)
- **MCP Server**: Has `issue_writable` feature (enabled by default)
- **Shared File Support**: Available by default in all builds
- Use `--no-default-features` to disable optional functionality

#### Common Feature Combinations
```bash
# Read-only CLI
--features "cli git issue project space"

# Full CLI with write capabilities
--features "cli git issue project project_writable space"

# Development/testing with all features
--all-features
```

### Recent Major Updates
- **Import Organization Fix**: Fixed missing imports in `backlog-issue` and `backlog-project` crates
  - Added `CategoryId` import to `backlog-project/src/api/mod.rs`
  - Added `IssueKey`, `AddIssueParams`, and `UpdateIssueParams` imports to `backlog-issue/src/api/mod.rs`
- **Category Management**: Complete CRUD operations for project categories with TDD implementation
  - Added `update_category()` API method with comprehensive test coverage
  - Extended CLI with `category-add`, `category-update`, `category-delete` commands
  - Proper feature flag separation (`project_writable`) for write operations
- **Domain Model Refactoring**: Extracted shared domain models into `backlog-domain-models` crate to eliminate circular dependencies and reduce code duplication
- **Unified File Downloads**: Consolidated 12 format-specific download tools into 4 intelligent tools
- **Shared File API**: Added complete support for browsing and downloading shared files
- **Type Safety Improvements**: Added `FileType` enum and `SharedFileId` for better type safety
- **Format Detection**: Intelligent content-type and UTF-8 analysis for automatic format handling

### TDD Development Process
When implementing new API features, follow Test-Driven Development:

1. **API Documentation Research**: Read official Backlog API docs carefully
2. **Write Tests First**: Create comprehensive unit tests covering success and error cases
3. **Implement API Method**: Write the minimal implementation to pass tests
4. **Add CLI Commands**: Extend CLI with new subcommands when applicable
5. **Integration Testing**: Test with real Backlog instance
6. **Documentation Update**: Update API.md, README.md, and CLAUDE.md

**Important**: Never modify tests during implementation phase - tests define the contract.

**Testing Requirements**:
- Comprehensive test coverage for all API methods (success, error, edge cases)
- Mock different HTTP status codes and API responses
- Test both minimal and maximal parameter sets
- Include integration tests with real API when possible

### Todo Management Guidelines
When working on complex tasks (3+ steps or multi-file changes):

1. **Create Todo List**: Use TodoWrite to plan implementation steps
2. **Mark In-Progress**: Update ONE task to in_progress before beginning work
3. **Complete Immediately**: Mark tasks as completed as soon as finished (don't batch completions)
4. **Sequential Work**: Only have one task in_progress at any time
5. **Remove Obsolete**: Remove tasks that become irrelevant during implementation
6. **Never Mark Incomplete**: Only mark completed when fully done - if blocked, keep as in_progress and create new tasks for blockers

**Todo Requirements**:
- Mark completed ONLY when tests pass, code compiles, and functionality works
- Create new tasks for discovered work during implementation
- Use specific, actionable task descriptions
- Don't use todo system for trivial single-step tasks

**When to Use Todo System**:
- Complex multi-step tasks requiring 3+ distinct steps
- Non-trivial implementations requiring careful planning
- When user explicitly requests todo list
- When user provides multiple tasks to complete

### Error Handling and Debugging Process
When encountering failures (compilation, tests, linting):

1. **Identify Root Cause**: Use git bisect for CI failures, read error messages carefully
2. **Fix Systematically**: Address one error type at a time
3. **Verify Fixes**: Run affected commands after each fix
4. **Follow Patterns**: Use existing error handling patterns in the codebase
5. **Test Edge Cases**: Ensure fixes handle both success and error scenarios

**Common Issues**:
- Clippy warnings: Follow lint suggestions exactly (bool_assert_comparison, clone_on_copy, redundant_field_names)
- Missing imports: Check existing modules for import patterns
- Feature flag issues: Ensure conditional compilation is correctly applied

**Debugging Commands**:
```bash
# For compilation issues
cargo check --all-targets --all-features

# For test failures
cargo test --all-features --all-targets

# For linting issues
cargo clippy --all-features --all-targets -- -D warnings

# For formatting
cargo fmt --all
```

### Code Quality Standards
Maintain consistent code quality across the codebase:

**Testing Requirements**:
- Comprehensive test coverage for all API methods (success, error, edge cases)
- Mock different HTTP status codes and API responses
- Test both minimal and maximal parameter sets
- Include integration tests with real API when possible

**Documentation Standards**:
- API methods must have docstring describing endpoint mapping
- Example: `/// Corresponds to \`GET /api/v2/projects/:projectIdOrKey\`.`
- Parameter structs should document field purposes and constraints
- Update API.md implementation counts after adding new endpoints

**Code Organization**:
- Follow existing import patterns and module structure
- Use consistent naming conventions (snake_case for Rust, camelCase for API parameters)
- Implement proper error handling with specific error types
- Apply feature flags correctly for conditional compilation

### Git Workflow Guidelines
Follow these practices for repository management:

**Branch Management**:
- Work on feature branches for new API implementations
- Use descriptive branch names (e.g., `feature/version-milestone-api`)
- Keep commits focused and atomic

**Commit Standards**:
- Write clear commit messages describing the change purpose
- Include API endpoint information in commit messages
- Reference issue numbers or documentation when applicable
- Example: `feat: Add Version/Milestone update API - PATCH /api/v2/projects/:projectIdOrKey/versions/:id`

**CI/CD Compliance**:
- Ensure all CI checks pass before merging (check, test, clippy, fmt)
- Use git bisect to identify CI-breaking commits
- Fix linting and formatting issues immediately
- Never ignore clippy warnings in production code

**Pre-commit Verification**:
```bash
# Always run before committing
cargo check --all-targets --all-features
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --all
```

### API Implementation Patterns
Follow these established patterns when implementing new API endpoints:

**Parameter Struct Pattern**:
```rust
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateEntityParams {
    pub required_field: String,
    pub optional_field: Option<String>,
    pub optional_bool: Option<bool>,
}

#[cfg(feature = "writable")]
impl From<&UpdateEntityParams> for Vec<(String, String)> {
    fn from(params: &UpdateEntityParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("requiredField".to_string(), params.required_field.clone()));
        
        if let Some(value) = &params.optional_field {
            seq.push(("optionalField".to_string(), value.clone()));
        }
        
        if let Some(flag) = params.optional_bool {
            seq.push(("optionalBool".to_string(), flag.to_string()));
        }
        
        seq
    }
}
```

**API Method Pattern**:
```rust
#[cfg(feature = "writable")]
pub async fn update_entity(
    &self,
    project_id_or_key: impl Into<ProjectIdOrKey>,
    entity_id: impl Into<EntityId>,
    params: &UpdateEntityParams,
) -> Result<Entity> {
    let params_vec: Vec<(String, String)> = params.into();
    let path = format!(
        "/api/v2/projects/{}/entities/{}", 
        project_id_or_key.into(), 
        entity_id.into()
    );
    self.client.patch(&path, &params_vec).await
}
```

**Test Pattern**:
```rust
#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;
    use client::test_utils::setup_client;

    #[tokio::test]
    async fn test_update_entity_success() {
        let client = setup_client(200, r#"{"id": 1, "name": "Updated"}"#);
        let api = EntityApi::new(client);
        
        let params = UpdateEntityParams {
            required_field: "test".to_string(),
            optional_field: Some("optional".to_string()),
            optional_bool: Some(true),
        };
        
        let result = api.update_entity("PROJECT", 1, &params).await;
        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.name, "Updated");
    }

    #[tokio::test]
    async fn test_update_entity_not_found() {
        let client = setup_client(404, r#"{"errors": [{"message": "Entity not found"}]}"#);
        let api = EntityApi::new(client);
        
        let params = UpdateEntityParams {
            required_field: "test".to_string(),
            optional_field: None,
            optional_bool: None,
        };
        
        let result = api.update_entity("PROJECT", 999, &params).await;
        assert!(result.is_err());
    }
}
```

**CLI Integration Pattern**:
```rust
#[cfg(feature = "writable")]
EntityUpdate {
    #[clap(short, long)]
    project_id: String,
    #[clap(short, long)]
    entity_id: u32,
    #[clap(short, long)]
    name: String,
    #[clap(long)]
    description: Option<String>,
    #[clap(long)]
    archived: Option<bool>,
}

// In command handler
#[cfg(feature = "writable")]
ProjectCommands::EntityUpdate(args) => {
    let params = UpdateEntityParams {
        required_field: args.name,
        optional_field: args.description,
        optional_bool: args.archived,
    };
    
    match client.project().update_entity(&args.project_id, args.entity_id, &params).await {
        Ok(entity) => {
            println!("✅ Entity updated successfully");
            println!("ID: {}", entity.id);
            println!("Name: {}", entity.name);
        }
        Err(e) => {
            eprintln!("❌ Failed to update entity: {}", e);
            std::process::exit(1);
        }
    }
}
```

**Key Points**:
- Always use feature flags for write operations
- Form-encode parameters manually for precise control
- Follow consistent naming (camelCase for API, snake_case for Rust)
- Include comprehensive error handling in tests
- Provide clear success/failure messages in CLI
- Use strongly-typed identifiers from `backlog-core`

### Real API Testing Guidelines
When testing with actual Backlog API:

**Safety Practices**:
- Use test projects/repositories when possible
- Verify endpoints with read operations before write operations
- Be cautious with delete operations - verify data before deletion
- Use API key in URL parameters: `?apiKey=${BACKLOG_API_KEY}`

**Testing Workflow**:
1. **Verify Environment**: Check `BACKLOG_BASE_URL` and `BACKLOG_API_KEY`
2. **Start with Read Operations**: List existing data before creating/modifying
3. **Test Error Cases**: Try invalid IDs to verify error handling
4. **Clean Up**: Remove test data when appropriate
5. **Document Results**: Note successful operations and any issues

**Example Testing Sequence**:
```bash
# 1. List existing data
cargo run --package blg -- pr list -p PROJECT -r REPO

# 2. Test creation
cargo run --package blg -- pr create -p PROJECT -r REPO --summary "Test" --description "Test" --base master --branch feature

# 3. Test modification/deletion
cargo run --package blg -- pr delete-attachment -p PROJECT -r REPO -n PR_NUMBER -a ATTACHMENT_ID

# 4. Verify changes
curl -s "https://api.url/endpoint?apiKey=${BACKLOG_API_KEY}" | jq .
```
