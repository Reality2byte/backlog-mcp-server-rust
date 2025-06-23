# Task Completion Requirements

## Essential Commands to Run After Code Changes

### 1. Build Verification
```bash
# Verify all crates compile
cargo check --all-targets --all-features
```

### 2. Test Execution
```bash
# Run all tests with all features
cargo test --all-features --all-targets
```

### 3. Code Quality Checks
```bash
# Lint with clippy (treat warnings as errors)
cargo clippy --all-features --all-targets -- -D warnings

# Format all code
cargo fmt --all
```

### 4. Feature-Specific Testing
```bash
# Test specific domain changes (example for backlog-project)
cargo test --package backlog-project --all-features

# Test with specific feature combinations
cargo test --package backlog-project --features "writable"
```

## Critical Requirements

### Before Committing
1. **All CI checks must pass**: check, test, clippy, fmt
2. **No clippy warnings**: Use `-D warnings` flag to treat warnings as errors
3. **All tests passing**: Both unit and integration tests
4. **Code formatted**: Use `cargo fmt --all`

### For API Changes
1. **Test coverage**: Both success and error scenarios
2. **Integration testing**: Test with real Backlog API when possible
3. **Documentation updates**: Update API.md endpoint counts
4. **Feature flag compliance**: Ensure proper conditional compilation

### For CLI Changes
1. **Feature flag verification**: Test with different feature combinations
2. **Help text validation**: Ensure commands show proper usage
3. **Error handling**: Proper error messages and exit codes

### For MCP Server Changes
1. **JSON schema validation**: Ensure proper schemars derives
2. **Tool registration**: Verify tools are properly registered in server.rs
3. **Error conversion**: Proper mapping to rmcp::Error

## Quality Standards
- **No compilation errors**: All code must compile cleanly
- **No test failures**: All tests must pass
- **No clippy warnings**: Follow all lint suggestions
- **Consistent formatting**: Use cargo fmt
- **Feature gate compliance**: Conditional compilation must work correctly

## Verification Commands
Run these commands in sequence to verify task completion:
```bash
cargo check --all-targets --all-features && \
cargo test --all-features --all-targets && \
cargo clippy --all-features --all-targets -- -D warnings && \
cargo fmt --all
```

If any command fails, the task is not complete and must be fixed before proceeding.