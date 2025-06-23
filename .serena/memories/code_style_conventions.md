# Code Style and Conventions

## Naming Conventions
- **Rust Code**: snake_case for functions, variables, modules
- **API Parameters**: camelCase for form-encoded parameters (matching Backlog API)
- **Types**: PascalCase for structs, enums, traits
- **Constants**: SCREAMING_SNAKE_CASE

## Architectural Patterns
### Domain Crate Structure
Each domain API crate follows consistent patterns:
```
crates/backlog-{domain}/
├── src/
│   ├── lib.rs                # Public API exports with feature gates
│   ├── models.rs             # Domain-specific model re-exports
│   └── api/
│       ├── mod.rs            # Module declarations and exports
│       ├── {domain}_api.rs   # Main API implementation
│       ├── get_*.rs          # Read-only operations
│       ├── add_*.rs          # Create operations (feature-gated)
│       ├── update_*.rs       # Update operations (feature-gated)
│       └── delete_*.rs       # Delete operations (feature-gated)
```

### Parameter Handling Patterns
- **Read Operations**: Simple struct with builder methods
- **Write Operations**: Manual form encoding via `From<&Params> for Vec<(String, String)>`
- **Array Parameters**: Use `foo[]` syntax for form encoding

### Error Handling
- Unified error handling via `ApiError` in `backlog-api-core`
- Domain-specific errors wrap core error types
- Comprehensive error testing for all scenarios

## Type Safety Patterns
- **Strongly-typed identifiers**: `ProjectId`, `IssueKey`, `SharedFileId`
- **Domain enums**: `FileType`, `Priority`, `Status`, etc.
- **Builder pattern**: For complex request parameters
- **Custom deserialization**: Using temporary `Raw*` structs for complex JSON

## Feature Flag Organization
- **Read operations**: Always available
- **Write operations**: Behind `*_writable` feature flags
- **Domain modules**: Individual feature flags (`issue`, `project`, etc.)
- **Meta features**: `all_writable` for convenience

## Testing Standards
- **Comprehensive coverage**: Success, error, and edge cases
- **Mock-based testing**: Using `wiremock` for unit tests
- **Separate test files**: Read-only vs writable operations
- **Common test utilities**: Shared setup functions in `tests/common/`

## Documentation Standards
- **API endpoint mapping**: `/// Corresponds to \`GET /api/v2/endpoint\``
- **Parameter documentation**: Clear field descriptions and constraints
- **Error scenarios**: Document expected error conditions
- **Examples**: Include usage examples where appropriate