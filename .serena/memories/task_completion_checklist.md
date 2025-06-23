# Task Completion Checklist

When a task is completed, run these commands in sequence:

1. **Build Check**: `cargo check --all-targets --all-features`
2. **Tests**: `cargo test --all-features --all-targets`
3. **Linting**: `cargo clippy --all-features --all-targets -- -D warnings`
4. **Formatting**: `cargo fmt --all`

## Important Notes
- Always run all commands before considering a task complete
- Fix any warnings or errors before proceeding
- Use feature flags appropriately (writable features for write operations)