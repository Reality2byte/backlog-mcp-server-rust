# Task Completion Workflow

## Always Run After Code Changes
```bash
# 1. Check compilation
cargo check --all-targets --all-features

# 2. Run tests
cargo test --all-features --all-targets

# 3. Lint (must pass with zero warnings)
cargo clippy --all-features --all-targets -- -D warnings

# 4. Format code
cargo fmt --all
```

## Error Resolution Process
1. **Identify Root Cause**: Read error messages carefully
2. **Fix Systematically**: Address one error type at a time
3. **Verify Fixes**: Run affected commands after each fix
4. **Follow Patterns**: Use existing error handling patterns

## Common Issues
- **Clippy warnings**: Follow suggestions exactly (bool_assert_comparison, clone_on_copy, redundant_field_names)
- **Missing imports**: Check existing modules for import patterns
- **Feature flag issues**: Ensure conditional compilation is correct

## Before Committing
- All CI checks must pass (check, test, clippy, fmt)
- No clippy warnings allowed in production code
- Use `git bisect` to identify CI-breaking commits if needed

## Testing Requirements
- Comprehensive test coverage for all API methods
- Test both success and error cases
- Mock different HTTP status codes
- Test minimal and maximal parameter sets
- Include integration tests when possible

## Documentation Updates
- Update API.md implementation counts after adding endpoints
- Update README.md for new features
- Update CLAUDE.md for new patterns or conventions