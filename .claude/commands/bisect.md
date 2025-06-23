The build is failing in our CI from some commit somewhere. Find the commit that introduced a bug by using `git bisect`

## commands executed in CI
cargo check --all-targets --all-features
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
