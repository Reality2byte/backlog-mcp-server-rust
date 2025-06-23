The build is failing in the current repository. Investigate the root cause of this bug.

## commands I executed
cargo check --all-targets --all-features
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
