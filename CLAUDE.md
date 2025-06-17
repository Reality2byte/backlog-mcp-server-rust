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
backlog-domain-models/       # Shared domain models (Priority, Status, Category, etc.)
backlog-{issue,project,space,user,document,git,file}/ # Domain-specific API modules
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
- CLI requires `"cli git issue"` features  
- MCP server has `issue_writable` feature (enabled by default)
- Shared file support is available by default in all builds
- Use `--no-default-features` to disable optional functionality

### Recent Major Updates
- **Domain Model Refactoring**: Extracted shared domain models into `backlog-domain-models` crate to eliminate circular dependencies and reduce code duplication
- **Unified File Downloads**: Consolidated 12 format-specific download tools into 4 intelligent tools
- **Shared File API**: Added complete support for browsing and downloading shared files
- **Type Safety Improvements**: Added `FileType` enum and `SharedFileId` for better type safety
- **Format Detection**: Intelligent content-type and UTF-8 analysis for automatic format handling