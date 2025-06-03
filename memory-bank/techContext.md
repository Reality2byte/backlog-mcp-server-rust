# Tech Context

## Core Technologies
-   **Programming Language**: Rust (Edition 2024, as per workspace `Cargo.toml`)
-   **Build System/Package Manager**: Cargo

## Key Dependencies (versions from workspace `Cargo.toml` where specified)
-   **HTTP Client**: `reqwest` (version 0.12.18, with `json` feature) - For making HTTP requests to the Backlog API.
-   **Asynchronous Runtime**: `tokio` (version 1.45, with `full` features) - For managing asynchronous operations.
-   **Serialization/Deserialization**:
    -   `serde` (version 1.0, with `derive` feature) - Core library for serializing and deserializing Rust data structures.
    -   `serde_json` (version 1.0) - For JSON-specific serialization/deserialization.
    -   `serde_repr` (version 0.1) - For serializing/deserializing enums as their integer representations (used in `backlog-core`).
-   **Error Handling**: `thiserror` (version 2.0) is used for defining custom error types. 
    -   The primary error type for the `backlog-api-client` library is `backlog_api_core::Error` (often aliased as `ApiError`).
    -   It unifies various error sources: HTTP (`reqwest`), JSON (`serde_json`), URL (`url`) parsing, and validation errors from `backlog_core::Error`.
    -   It now includes an `HttpStatus` variant to hold structured error details (`BacklogApiErrorEntry`, `BacklogApiErrorResponse`) parsed from non-2xx Backlog API responses by the `client::Client`.
-   **URL Parsing/Manipulation**: `url` (version 2.5) - For handling URLs.
-   **Regular Expressions**: `regex` (version 1.11) - Potentially used for parsing or validating string patterns.
-   **Date and Time**: `chrono` (version 0.4.41, with `serde` feature) - For handling date and time values.
-   **Builder Pattern**: `derive_builder` (version 0.20) - Used for creating builder patterns for request parameter structs.
    -   Convention: Use `#[builder(..., build_fn(error = "ApiError"))]` to make the `build()` method return `Result<Self, backlog_api_core::Error>`.
    -   This requires `backlog_api_core::Error` to implement `From<derive_builder::UninitializedFieldError>`. (Note: `SetFieldError` conversion was deemed unnecessary for the current `derive_builder` version/usage in this project).
-   **CLI Argument Parsing**: `clap` (version 4.5, with `derive` feature) - Used by the `blg` binary in `backlog-api-client` (enabled via `cli` feature).
-   **MCP SDK**: `rmcp` (git = "https://github.com/modelcontextprotocol/rust-sdk", branch = "main", features = ["transport-io"]) - For building MCP servers (used by `mcp-backlog-server`).
-   **Schema Generation (for MCP tools)**: `schemars` (version 0.8, typically with `chrono` feature enabled via workspace dependency) - Used for generating JSON schemas for tool inputs/outputs, and for models that might be serialized by MCP tools.
    -   It's an optional dependency in `backlog-core`, `backlog-issue`, and `backlog-git`, enabled via a `schemars` feature in each of those crates.
    -   When this feature is active, relevant models (e.g., `User`, `IssueId` in `backlog-core`; `Comment` in `backlog-issue`; `Repository`, `PullRequest` in `backlog-git`) derive `JsonSchema`.

## Development Setup
-   **Rust Toolchain**: Requires a Rust installation compatible with Edition 2024.
-   **Cargo**: Used for building, testing, and managing dependencies.
    -   `cargo build --all-targets --all-features` to compile the entire workspace with all features.
    -   `cargo test --all-targets --all-features` to run tests for the entire workspace.
    -   `cargo run --bin blg --features git,cli -- <subcommand> <args>` to run the CLI tool (e.g., `cargo run --bin blg --features git,cli -- repo list --project-id MYPROJ`).
    -   `cargo run --bin mcp-backlog-server` to run the MCP server locally for testing (requires `BACKLOG_BASE_URL` and `BACKLOG_API_KEY` env vars to be set).

## Technical Constraints
-   The library and CLI must interact with the Backlog API, so network connectivity to the Backlog instance is required at runtime.
-   API rate limits imposed by Backlog need to be considered (the `ApiRateLimit` struct in `backlog-api-core` suggests this is being handled).
-   **MCP Server Configuration**:
    -   The `mcp-backlog-server` requires `BACKLOG_BASE_URL` and `BACKLOG_API_KEY` environment variables to be set for its `BacklogApiClient` instance. These are typically provided by the MCP client system via the server's registration in `cline_mcp_settings.json` (or similar).
    -   The server itself is registered in `cline_mcp_settings.json` with its command path, arguments, and environment variables.

## Tool Usage Patterns
-   **`list_code_definition_names`**: Useful for getting a high-level overview of structs, enums, and functions within each module, especially for understanding the public API of each crate.
    -   Many `src` directories in the API module crates (`backlog-issue`, `backlog-project`, etc.) did not show top-level definitions. This implies that their primary `lib.rs` might be focused on re-exporting modules from subdirectories like `api/`, `models/`, `requests/`, and `responses/`. Further investigation into these subdirectories would be needed to map out all specific API calls and data structures.
-   **`list_files` (recursive)**: Essential for understanding the module structure within each crate (e.g., `api/`, `models/`, `requests/`, `responses/` subdirectories).
-   **`read_file`**: Critical for understanding `Cargo.toml` files (both workspace and individual crate level) to determine dependencies and crate features. Also used for reading `Readme.md` or other documentation if available.
