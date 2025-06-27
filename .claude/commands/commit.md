1. Execute the following command.
    cargo check -q --all-targets --all-features
    cargo test -q --all-features --all-targets
    cargo clippy -q --all-features --all-targets -- -D warnings
    cargo fmt --all
2. Commit your changes. I just want to get it done with just one approval so you SHOULD concatinate `git add` and `git commit`.
