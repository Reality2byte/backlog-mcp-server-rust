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
    -   In `mcp-backlog-server`, errors from `backlog-api-client` (i.e., `ApiError`) or `backlog-core` (`CoreError`) are converted into `mcp_backlog_server::error::Error`. This custom error type then has an `impl From<Error> for rmcp::Error` to integrate with the MCP framework's error handling.
-   **URL Parsing/Manipulation**: `url` (version 2.5) - For handling URLs.
-   **Regular Expressions**: `regex` (version 1.11) - Potentially used for parsing or validating string patterns.
-   **Date and Time**: `chrono` (version 0.4.41, with `serde` feature) - For handling date and time values.
-   **Builder Pattern**: `derive_builder` (version 0.20) - Used for creating builder patterns for request parameter structs.
    -   Convention: Use `#[builder(..., build_fn(error = "ApiError"))]` to make the `build()` method return `Result<Self, backlog_api_core::Error>`.
-   **Binary Data Handling**: `bytes` (version 1) - For efficient handling of byte streams, used for file downloads. Re-exported by `backlog-api-core`.
-   **Base64 Encoding**: `base64` (version 0.21) - Used for encoding binary data for JSON transport in MCP tools.
-   **CLI Argument Parsing**: `clap` (version 4.5, with `derive` feature) - Used by the `blg` binary.
-   **MCP SDK**: `rmcp` (git, branch = "main", features = ["transport-io"]) - For building MCP servers.
    -   The `download_issue_attachment_image` tool now uses `rmcp::model::Content::image` to return image data, which expects base64 encoded string and a MIME type. It includes a check to ensure the attachment is an image.
    -   The `download_issue_attachment_text` tool uses `rmcp::model::Content::text` to return text content. It attempts to convert the attachment bytes to a UTF-8 string and errors if the conversion fails.
-   **Schema Generation (for MCP tools)**: `schemars` (version 0.8).
    -   For library crates (`backlog-core`, `backlog-issue`, `backlog-git`, etc.), `schemars` is an optional dependency enabled via a `schemars` feature. Models derive `JsonSchema` conditionally.
    -   For `mcp-backlog-server` request structs (e.g., in `src/issue/request.rs`), the convention is to use `use rmcp::schemars;` and then `#[derive(schemars::JsonSchema)]`. This leverages the `schemars` re-export from the `rmcp` crate.
        -   **Naming Convention**: JSON keys for MCP tool request parameters MUST use `snake_case`. `#[serde(rename_all = "camelCase")]` or other renaming attributes for request structs in `mcp-backlog-server` should NOT be used. Rust struct field names should also be `snake_case`.
    -   Used for models like `User`, `Comment`, `Repository`, `PullRequest`, `PullRequestAttachment` (in `backlog-git`), `ProjectStatus` (in `backlog-project`), and `Attachment` (in `backlog-issue`), as well as MCP request structs.
-   **Inter-Crate Dependencies**:
    -   `backlog-issue` now depends on `backlog-project` for the `ProjectStatus` model.
    -   The `schemars` feature in `backlog-issue` now also enables `backlog-core/schemars` to ensure `AttachmentId` (from `backlog-core`) can derive `JsonSchema`.
    -   `backlog-git` now has a `schemars` feature that enables `backlog-core/schemars` for `AttachmentId` and `dep:schemars` for its own models like `PullRequestAttachment`.
    -   `mcp-backlog-server` specific dependencies and features:
        -   Its `backlog-api-client` dependency now enables the `project` feature, in addition to `issue`, `document`, and `git`.
        -   It now has direct dependencies on `backlog-project` (for `ProjectStatus` type) and `backlog-core` (for `ProjectIdOrKey` parsing).
        -   It now has a direct dependency on `schemars` (with `derive` feature) for `JsonSchema` derivation on its request structs.

## Development Setup
-   **Rust Toolchain**: Edition 2024.
-   **Cargo**: For building, testing, etc.
    -   Standard build: `cargo build --all-features`
    -   Standard test: `cargo test --all-features --all-targets`
    -   Run CLI: `cargo run --bin blg ...`
    -   Run MCP Server: `cargo run --bin mcp-backlog-server ...`
    -   **Final Verification Commands (to be run after each implementation task)**:
        -   `cargo check --all-targets --all-features` (Comprehensive check)
        -   `cargo test --all-features --all-targets` (Run all tests)
        -   `cargo clippy` (Linting for the entire workspace, default settings)
        -   `cargo fmt --all` (Formatting the entire workspace, default settings)

## Technical Constraints
-   Network connectivity for Backlog API.
-   API rate limits.
-   **MCP Server Configuration**: Env vars `BACKLOG_BASE_URL`, `BACKLOG_API_KEY`.

## Tool Usage Patterns
-   **`list_code_definition_names`**: For high-level overview.
-   **`list_files` (recursive)**: For module structure.
-   **`read_file`**: For `Cargo.toml`, `Readme.md`, etc.
