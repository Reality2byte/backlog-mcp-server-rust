# Code Style and Conventions

## Naming Conventions
- **Rust style**: snake_case for functions, variables, modules
- **API parameters**: camelCase for JSON/form fields
- **Identifiers**: Strongly-typed newtypes (ProjectId, IssueKey, etc.)

## Type Safety Patterns
- Use strongly-typed identifiers from `backlog-core`
- Define `XxxResponse` type aliases for API responses
- Manual form serialization for write operations using `Vec<(String, String)>`
- Feature-gate write operations with `#[cfg(feature = "writable")]`

## Error Handling
- Unified error handling via `ApiError` in `backlog-api-core`
- Domain-specific errors wrap core error types
- Proper error propagation with `?` operator

## Testing Standards
- Mock-based testing using `wiremock`
- Test both success and error cases
- Separate test files for read/write operations
- Common test utilities in `tests/common/mod.rs`

## File Organization
- Parameter structs in separate files (`get_*.rs`, `add_*.rs`)
- Main API struct in `{domain}_api.rs`
- Module exports in `api/mod.rs` with proper feature gating
- Re-exports in `lib.rs`

## Form Encoding Pattern
```rust
// Parameter struct without Serialize
#[derive(Builder, Debug, Clone)]
pub struct AddCommentParams {
    pub content: String,
    pub notified_user_ids: Option<Vec<u32>>,
}

// Manual serialization for form encoding
impl From<&AddCommentParams> for Vec<(String, String)> {
    fn from(params: &AddCommentParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("content".to_string(), params.content.clone()));
        
        // Array parameters use [] syntax
        if let Some(user_ids) = &params.notified_user_ids {
            user_ids.iter().for_each(|id| {
                seq.push(("notifiedUserId[]".to_string(), id.to_string()));
            });
        }
        seq
    }
}
```