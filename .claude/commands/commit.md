Execute the following command. And then, commit it. I just want to get it done with just one approval so you SHOULD concatinate `git add` and `git commit`.

cargo check -q --all-targets --all-features
cargo test -q --all-features --all-targets
cargo clippy -q --all-features --all-targets -- -D warnings
cargo fmt --all
 