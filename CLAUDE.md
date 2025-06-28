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
├── backlog-api-macros/     # Procedural macros for API parameter serialization
├── backlog-core/           # Core types and identifiers shared across all modules
├── backlog-api-core/       # Common API utilities and error types
├── backlog-domain-models/  # Shared domain models (Priority, Status, Category, etc.)
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

### API Parameter Date Handling Guidelines

The Backlog API uses different date formats for request parameters versus response data. Follow these guidelines for consistent date handling:

#### Date Type Selection Rules

**For Request Parameters (API Input):**
- **Use `ApiDate`** for date-only parameters (start dates, due dates, date filters)
- **Format**: Automatically converts to "yyyy-MM-dd" via Display trait
- **Examples**: `created_since`, `created_until`, `start_date`, `due_date`
- **Benefits**: Type-safe, automatic formatting, prevents invalid date strings

**For Response Data (API Output):**
- **Use `DateTime<Utc>`** for timestamp fields in models
- **Format**: Parses ISO 8601 format "2013-08-05T06:15:06Z"
- **Examples**: `created`, `updated`, `pushed_at`, `merge_at`
- **Benefits**: Full timestamp precision, timezone awareness

**Legacy String Fields:**
- Some existing model fields use `String` for dates (e.g., Issue.start_date, Issue.due_date)
- These remain as String to maintain API compatibility
- Do not convert response String date fields to ApiDate

#### Implementation Patterns

**Parameter Struct Pattern:**
```rust
#[derive(Debug, Clone)]
pub struct UpdateIssueParams {
    // Use ApiDate for date parameters
    pub start_date: Option<ApiDate>,
    pub due_date: Option<ApiDate>,
}
```

**Model Pattern:**
```rust
pub struct Issue {
    // Legacy String fields remain unchanged
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    // Timestamp fields use DateTime<Utc>
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
```

**CLI Integration:**
```rust
// Convert CLI string input to ApiDate
params.start_date = start_date.as_ref().map(|d| {
    DateTime::parse_from_str(&format!("{}T00:00:00Z", d), "%Y-%m-%dT%H:%M:%SZ")
        .map(|dt| ApiDate::from(dt.with_timezone(&Utc)))
        .unwrap_or_else(|_| panic!("Invalid date format: {}", d))
});
```

#### Decision Matrix

| Use Case | Type | Format | Direction | Example |
|----------|------|--------|-----------|---------|
| Date filter parameters | `ApiDate` | yyyy-MM-dd | Request → API | `created_since`, `updated_until` |
| Date input parameters | `ApiDate` | yyyy-MM-dd | Request → API | `start_date`, `due_date` |
| Timestamp responses | `DateTime<Utc>` | ISO 8601 | API → Response | `created`, `updated` |
| Legacy model dates | `String` | varies | API → Response | `Issue.start_date` |

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

### Automated Form Parameter Serialization (ToFormParams Macro)

The `backlog-api-macros` crate provides the `ToFormParams` derive macro to automatically generate `From<&T> for Vec<(String, String)>` implementations for API parameter structs. This eliminates boilerplate and ensures consistency across all form parameter serialization.

#### Usage

```rust
use backlog_api_macros::ToFormParams;

#[derive(ToFormParams)]
struct AddCommentParams {
    content: String,                           // Required field → "content"
    #[form(array, name = "notifiedUserId")]
    notified_user_ids: Option<Vec<u32>>,      // Optional array → "notifiedUserId[]"
    #[form(skip)]
    issue_id_or_key: IssueIdOrKey,            // Skipped (used for URL path)
}
```

#### Supported Attributes

- `#[form(skip)]` - Skip this field during serialization
- `#[form(name = "customName")]` - Use custom field name in API
- `#[form(array)]` - Treat as array parameter (adds `[]` suffix)
- `#[form(flatten)]` - Flatten nested struct fields (reserved for future use)

#### Automatic Transformations

- **Field Names**: `snake_case` → `camelCase` (e.g., `user_id` → `userId`)
- **Optional Fields**: `Option<T>` fields are only included if `Some(value)`
- **Array Fields**: `Vec<T>` or `Option<Vec<T>>` with `#[form(array)]` generate multiple entries with `[]` suffix
- **Type Conversion**: All values converted via `.to_string()`
- **Enum Support**: Enums with `Display` trait (including `serde_repr` enums) are automatically supported

#### Migration Strategy

The macro is designed for **seamless adoption**:

1. **Backward Compatible**: Existing manual implementations continue to work
2. **Gradual Migration**: Replace manual implementations one at a time
3. **No Breaking Changes**: API consumers see no difference
4. **Feature-Gated**: Optional dependency that doesn't affect default builds

#### Implementation Benefits

- **Code Reduction**: 20+ manual implementations → declarative attributes
- **Consistency**: Eliminates human error in field name mapping
- **Maintainability**: Add new fields without touching serialization logic
- **Type Safety**: Compile-time validation of attribute usage

#### Special Processing Patterns for ToFormParams Macro

For the rare cases where the ToFormParams macro cannot handle certain field types or transformations automatically, use these established patterns:

**Date Field Processing**
For date fields requiring custom formatting (e.g., DateTime<Utc> to "yyyy-MM-dd"):

```rust
#[cfg_attr(feature = "macros", form(skip))] // Skip in macro
pub start_date: Option<DateTime<Utc>>,

// Extension method for macro users
#[cfg(all(feature = "writable", feature = "macros"))]
impl AddIssueParams {
    fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = self.into();
        
        if let Some(start_date) = &self.start_date {
            params.push((
                "startDate".to_string(),
                start_date.format("%Y-%m-%d").to_string(),
            ));
        }
        
        params
    }
}

// Use extension method in to_form()
fn to_form(&self) -> impl Serialize {
    #[cfg(feature = "macros")]
    { self.to_form_params() }
    #[cfg(not(feature = "macros"))]
    { /* manual implementation */ }
}
```

**Note on Enum Processing**: Most enums (especially those using `serde_repr` with `Display` trait) are handled automatically by the macro and do not require special processing. Only implement custom processing for enums with complex serialization requirements.

**Conditional Import Management**
For backward compatibility with manual implementations:

```rust
#[cfg(feature = "macros")]
use backlog_api_macros::ToFormParams;
#[cfg(not(feature = "macros"))]
use backlog_core::identifier::Identifier; // Needed for .value() method
```

**Extension Method Naming Convention**
Use `to_form_params()` (not `into_form_params()`) to avoid clippy::wrong_self_convention warnings when methods take `&self` by reference.

### Standard Domain Crate Structure

The `backlog-project` crate serves as the standard template for all domain API crates. Follow this structure when implementing new domain crates or refactoring existing ones:

#### Directory Structure
```
crates/backlog-{domain}/
├── src/
│   ├── lib.rs                    # Public API exports and feature gates
│   ├── models.rs                 # Domain-specific model re-exports
│   └── api/
│       ├── mod.rs                # Module declarations and exports
│       ├── {domain}_api.rs       # Main API implementation struct
│       ├── get_*.rs              # Read-only API parameter structs
│       ├── add_*.rs              # Create operation parameters (feature-gated)
│       ├── update_*.rs           # Update operation parameters (feature-gated)
│       └── delete_*.rs           # Delete operation parameters (feature-gated)
└── tests/
    ├── common/
    │   └── mod.rs                # Shared test utilities and imports
    ├── {domain}_api_test.rs      # Read-only API integration tests
    └── {domain}_writable_test.rs # Write operation tests (feature-gated)
```

#### Implementation Patterns

**1. Parameter File Structure (`api/get_*.rs`, `api/add_*.rs`, etc.)**

For read operations with automatic serialization:
```rust
// Type alias for API response
pub type GetEntityListResponse = Vec<Entity>;

// Parameter struct for the operation with automatic camelCase serialization
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEntityListParams {
    #[serde(skip)]
    pub project_id: ProjectId,  // Path parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>, // Query parameter
}

// Constructor and builder methods
impl GetEntityListParams {
    pub fn new(project_id: ProjectId) -> Self {
        Self {
            project_id,
            archived: None,
        }
    }
    
    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self
    }
}

impl IntoRequest for GetEntityListParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/entities", self.project_id)
    }
    
    fn to_query(&self) -> impl Serialize {
        self  // Automatic serialization with camelCase
    }
}
```

For write operations with manual serialization:
```rust
// Form serialization for write operations
#[cfg(feature = "writable")]
impl From<&AddEntityParams> for Vec<(String, String)> {
    fn from(params: &AddEntityParams) -> Self {
        // Implementation following form encoding pattern
    }
}
```

**2. Main API Struct (`api/{domain}_api.rs`)**
```rust
pub struct ProjectApi(Client);

// You should use `XxxxResponse` structs for the response types instead of models directly.
impl ProjectApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    // Read operations (always available)
    pub async fn get_entity_list(&self, params: GetEntityListParams) -> Result<GetEntityListResponse> {
        // Implementation
    }

    // Write operations (feature-gated)
    #[cfg(feature = "writable")]
    pub async fn add_entity(&self, params: AddEntityParams) -> Result<AddEntityResponse> {
        let params_vec: Vec<(String, String)> = (&params).into();
        self.0.post("/api/v2/path", &params_vec).await
    }
}
```

**3. Module Exports (`api/mod.rs`)**
```rust
mod {domain}_api;
mod get_entity_list;

#[cfg(feature = "writable")]
mod add_entity;

// Export response types (always available)
pub use get_entity_list::{GetEntityListParams, GetEntityListResponse};

// Export writable types with feature gates
#[cfg(feature = "writable")]
pub use add_entity::AddEntityParams;
pub use add_entity::AddEntityResponse;

pub use {domain}_api::{ProjectApi};
```

**4. Crate-level Exports (`lib.rs`)**
```rust
// Re-export API components
pub use api::*;

// Re-export domain models
pub use models::*;
```

**5. Test Organization**

**Common Test Utilities (`tests/common/mod.rs`)**
```rust
use {domain}_project::api::{ProjectApi};
use client::test_utils::setup_client;
use wiremock::MockServer;

/// Common test setup function
pub async fn setup_{domain}_api(mock_server: &MockServer) -> {ProjectApi} {
    let client = setup_client(mock_server).await;
    {ProjectApi}::new(client)
}

/// Common imports for tests
pub use backlog_core::identifier::{/* relevant IDs */};
pub use wiremock::matchers::{method, path};
pub use wiremock::{Mock, ResponseTemplate};
```

**Read-only Tests (`tests/{domain}_api_test.rs`)**
```rust
mod common;
use common::*;

#[tokio::test]
async fn test_get_entity_success() {
    let mock_server = MockServer::start().await;
    let api = setup_{domain}_api(&mock_server).await;
    
    // Mock setup and test implementation
}
```

**Write Operation Tests (`tests/{domain}_writable_test.rs`)**
```rust
#[cfg(feature = "writable")]
mod writable_tests {
    use {domain}_project::api::{/* writable params */};
    use client::test_utils::setup_client;
    // Other imports...

    #[tokio::test]
    async fn test_add_entity_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = {ProjectApi}::new(client);
        
        // Test implementation
    }
}
```

#### Key Benefits of This Structure

1. **Consistent Organization**: All domain crates follow the same pattern
2. **Feature Separation**: Read-only operations always available, write operations feature-gated
3. **Type Safety**: Response types clearly defined with `XxxResponse` pattern
4. **Test Isolation**: Separate test files for read/write operations with shared utilities
5. **Maintainability**: Clear separation of concerns and modular file organization
6. **Token Efficiency**: AI can easily navigate and understand the structure

#### Migration Guidelines

When refactoring existing domain crates to follow this standard:

1. **Create Parameter Files**: Extract parameter structs to individual files
2. **Implement Response Types**: Add `XxxResponse` type aliases
3. **Reorganize Tests**: Split tests into read/write files with common utilities
4. **Update Exports**: Ensure proper feature gating in module exports
5. **Verify Build**: Test with both default and `--features writable` builds

### MCP Server Organization
- Tools organized by domain modules (`issue/`, `git/`, `document/`, `project/`, `file/`, `user/`, `wiki/`)
- Each module contains:
  - `request.rs`: Input structs with JSON schema derivation
  - `bridge.rs`: Core logic functions
- Main `server.rs` registers tools with `#[tool]` attributes
- Unified file handling via `file_utils.rs` with intelligent format detection
- Access control via `access_control.rs` with optional project filtering

#### Project Access Control
The MCP server supports project-level access control through the `BACKLOG_PROJECTS` environment variable:

- **Environment Variable**: `BACKLOG_PROJECTS` - Comma-separated list of allowed project keys (e.g., `MFP,DEMO,TEST`)
- **Behavior**: When set, the server only allows access to the specified projects. If not set, all projects accessible with the API key are available
- **Implementation**: Uses type-safe access control with lazy API-based resolution for ProjectId ↔ ProjectKey mapping
- **Location**: Access control checks are performed in bridge layer functions before executing API calls
- **Example**: Setting `BACKLOG_PROJECTS="MFP"` allows access using both project key "MFP" and its corresponding project ID "14165"

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

### API Parameter Patterns to Recognize
When optimizing parameter structures, first analyze the parameter types:
- **Array parameters with special syntax**: If parameters use array syntax like `foo[]` in the serialized form, manual serialization is required. The `#[serde(skip)]` optimization pattern cannot be applied.
- **Simple scalar parameters**: Parameters that map directly to API fields (strings, numbers, booleans) can use automatic serialization. Since all Backlog API parameters use camelCase, add `#[serde(rename_all = "camelCase")]` to the struct level instead of individual field annotations.
- **Before optimizing**: Always check the `to_query()` implementation to understand the serialization requirements.

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
- **Issue Shared Files**: Added support for listing shared files linked to issues
  - Implemented `get_shared_file_list()` in `backlog-issue` API
  - Added `issue list-shared-files` command to CLI
  - Added `get_issue_shared_files` tool to MCP server
  - Re-exported `IssueSharedFile` type from `backlog-api-client` for proper type resolution
- **User Management Support**: Added user management capabilities to MCP server
  - Implemented `user/` module with request/bridge pattern
  - Added `get_user_list` tool for retrieving users in the space
  - Integration with `backlog-user` domain crate
- **Issue Comment Update API**: Complete implementation of comment update functionality (TDD)
  - Added `update_comment()` API method in `backlog-issue` crate with comprehensive test coverage
  - Implemented `UpdateCommentParams` following standard parameter patterns
  - Extended CLI with `issue update-comment` command (`--features issue_writable`)
  - Added `update_issue_comment` tool to MCP server
  - Full integration with real Backlog API testing on MFP-2 project
  - Follows `PATCH /api/v2/issues/:issueIdOrKey/comments/:commentId` endpoint specification
- **Wiki Page Creation API**: Complete implementation of wiki page creation functionality (TDD)
  - Added `add_wiki()` API method in `backlog-wiki` crate with comprehensive test coverage
  - Implemented `AddWikiParams` with manual form serialization following standard patterns
  - Extended CLI with `wiki create` command (`--features wiki_writable`)
  - Full integration with real Backlog API testing on MFP project (IDs: 4188464, 4188465)
  - Follows `POST /api/v2/wikis` endpoint specification with proper error handling
  - Supports required parameters (projectId, name, content) and optional mailNotify parameter
- **Wiki Page Deletion API**: Complete implementation of wiki page deletion functionality (TDD)
  - Added `delete_wiki()` API method in `backlog-wiki` crate with comprehensive test coverage
  - Implemented `DeleteWikiParams` with query parameter serialization for DELETE requests
  - Extended CLI with `wiki delete` command (`--features wiki_writable`)
  - Full integration with real Backlog API testing on MFP project (IDs: 4188466, 4188467)
  - Follows `DELETE /api/v2/wikis/:wikiId` endpoint specification with proper error handling
  - Supports required wikiId parameter and optional mailNotify query parameter
- **Wiki File Attachment API**: Complete implementation of wiki file attachment functionality (TDD)
  - Added `attach_files_to_wiki()` API method in `backlog-wiki` crate with comprehensive test coverage
  - Implemented `AttachFilesToWikiParams` with form-encoded array parameters (`attachmentId[]`)
  - Extended CLI with `wiki attach-file` command providing seamless 2-step workflow (`--features wiki_writable`)
  - Full integration with real Backlog API testing on MFP project (attachment ID: 90074, file: test-attachment.txt)
  - Follows `POST /api/v2/wikis/:wikiId/attachments` endpoint specification with proper error handling
  - Implements transparent file upload → attachment workflow (space upload + wiki attachment)
- **Wiki Attachment Deletion API**: Complete implementation of wiki attachment deletion functionality (TDD)
  - Added `delete_wiki_attachment()` API method in `backlog-wiki` crate with comprehensive test coverage
  - Implemented `DeleteWikiAttachmentParams` following standard parameter patterns
  - Extended CLI with `wiki delete-attachment` command with safety confirmation prompt (`--features wiki_writable`)
  - Follows `DELETE /api/v2/wikis/:wikiId/attachments/:attachmentId` endpoint specification
  - Safety features: interactive confirmation prompt, `--force` flag for automation, detailed error handling
  - Returns deleted attachment details including original creation information

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

### Refactoring Guidelines
Before applying any optimization pattern across multiple structures:
1. **Analyze all target structures first**: Review the implementation details of each structure to identify any special cases
2. **Group by compatibility**: Separate structures that can use the same optimization approach from those that need special handling
3. **Test one representative first**: If optimizing multiple similar structures, fully test one before proceeding to others
4. **Document limitations discovered**: When an optimization can't be applied, explain why for future reference

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

### Reporting Partial Completions
When completing tasks that involve multiple items where some succeed and others don't:
- Clearly separate successful items from unsuccessful ones
- Explain the specific reason for each unsuccessful item
- Provide a summary count (e.g., "Optimized 3 out of 5 structures")
- If patterns emerge (e.g., all array parameters failed), summarize the pattern
