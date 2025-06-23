# Suggested Commands

## Development Commands

### Build and Test
```bash
# Build all crates
cargo check --all-targets --all-features
cargo test --all-features --all-targets

# Lint and format
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --all
```

### System Utilities (Darwin)
- File operations: `fd` (find files), `rg` (search text), `ast-grep` (structural code search)
- JSON/YAML: `jq` (JSON), `yq` (YAML/XML)
- Git operations: `git`
- Directory operations: `ls`, `cd`

## API Testing
```bash
# Set environment variables
export BACKLOG_BASE_URL="https://your-space.backlog.jp"
export BACKLOG_API_KEY="your_api_key"
```