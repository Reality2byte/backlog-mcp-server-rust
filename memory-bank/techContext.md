# Tech Context

## Core Technologies
-   **Programming Language**: Rust (Edition 2024, as per workspace `Cargo.toml`)
-   **Build System/Package Manager**: Cargo

## Key Dependencies (versions from workspace `Cargo.toml` where specified)
-   **HTTP Client**: `reqwest` (version 0.12.18, with `json` feature) - For making HTTP requests to the Backlog API.
    -   The `client` crate provides a wrapper around `reqwest` and now also includes a `test_utils` module (enabled by the `test-utils` feature) for sharing test helpers like `setup_client`.
-   **Asynchronous Runtime**: `tokio` (version 1.45, with `full` features) - For managing asynchronous operations.
-   **Serialization/Deserialization**:
    -   `serde` (version 1.0, with `derive` feature) - Core library for serializing and deserializing Rust data structures.
    -   `serde_json` (version 1.0) - For JSON-specific serialization/deserialization.
    -   `serde_repr` (version 0.1) - For serializing/deserializing enums as their integer representations (used in `backlog-core`).
-   **Error Handling**: `thiserror` (version 2.0) is used for defining custom error types.
    -   The primary error type for the `backlog-api-client` library is `backlog_api_core::Error` (often aliased as `ApiError`).
    -   It unifies various error sources: HTTP (`reqwest`), JSON (`serde_json`), URL (`url`) parsing, and validation errors from `backlog_core::Error`.
    -   It includes an `HttpStatus` variant to hold structured error details (`BacklogApiErrorEntry`, `BacklogApiErrorResponse`) parsed from non-2xx Backlog API responses by the `client::Client`.
-   **URL Parsing/Manipulation**: `url` (version 2.5) - For handling URLs.
-   **Regular Expressions**: `regex` (version 1.11) - Potentially used for parsing or validating string patterns.
-   **Date and Time**: `chrono` (version 0.4.41, with `serde` feature) - For handling date and time values.
-   **Builder Pattern**: `derive_builder` (version 0.20) - Used for creating builder patterns for request parameter structs.
    -   Convention: Use `#[builder(..., build_fn(error = "ApiError"))]` to make the `build()` method return `Result<Self, backlog_api_core::Error>`.
-   **CLI Argument Parsing**: `clap` (version 4.5, with `derive` feature) - Used by the `blg` binary.
-   **MCP SDK**: `rmcp` (git, branch = "main", features = ["transport-io"]) - For building MCP servers.
-   **Schema Generation (for MCP tools)**: `schemars` (version 0.8).
    -   Optional dependency in `backlog-core`, `backlog-issue`, `backlog-git`, and now `backlog-project`, enabled via a `schemars` feature in each.
    -   Used for models like `User`, `Comment`, `Repository`, `PullRequest`, and the new `Status` in `backlog-project`.
-   **Inter-Crate Dependencies**:
    -   `backlog-issue` now depends on `backlog-project` for the `Status` model. This is a new dependency reflecting that project statuses are defined at the project level but used by issues.

## Development Setup
-   **Rust Toolchain**: Edition 2024.
-   **Cargo**: For building, testing, etc.
    -   `cargo build --all-features`
    -   `cargo test --all-features --all-targets`
    -   `cargo run --bin blg ...`
    -   `cargo run --bin mcp-backlog-server ...`

## Technical Constraints
-   Network connectivity for Backlog API.
-   API rate limits.
-   **MCP Server Configuration**: Env vars `BACKLOG_BASE_URL`, `BACKLOG_API_KEY`.

## Tool Usage Patterns
-   **`list_code_definition_names`**: For high-level overview.
-   **`list_files` (recursive)**: For module structure.
-   **`read_file`**: For `Cargo.toml`, `Readme.md`, etc.
