# Tech Context

## Core Technologies
-   **Programming Language**: Rust (Edition 2021, as per `Cargo.toml`)
-   **Build System/Package Manager**: Cargo

## Key Dependencies (from workspace `Cargo.toml`)
-   **HTTP Client**: `reqwest` (version 0.12, with `json` feature) - For making HTTP requests to the Backlog API.
-   **Asynchronous Runtime**: `tokio` (version 1.44, with `full` features) - For managing asynchronous operations, likely used by `reqwest` and the overall client.
-   **Serialization/Deserialization**:
    -   `serde` (version 1.0, with `derive` feature) - Core library for serializing and deserializing Rust data structures.
    -   `serde_json` (version 1.0) - For JSON-specific serialization/deserialization.
    -   `serde_repr` (version 0.1) - For serializing/deserializing enums as their integer representations.
-   **Error Handling**: `thiserror` (version 2.0) - For easily creating custom error types.
-   **URL Parsing/Manipulation**: `url` (version 2.5) - For handling URLs, likely for constructing API endpoint paths.
-   **Regular Expressions**: `regex` (version 1.11) - Potentially used for parsing or validating string patterns (e.g., `IssueKey`, `ProjectKey`).
-   **Date and Time**: `chrono` (version 0.4.40, with `serde` feature) - For handling date and time values from the API and enabling their serialization/deserialization.
-   **Builder Pattern**: `derive_builder` (version 0.20) - For easily creating builder patterns for complex structs, often used for request objects.

## Development Setup
-   **Rust Toolchain**: Requires a Rust installation compatible with Edition 2021.
-   **Cargo**: Used for building, testing, and managing dependencies.
    -   `cargo build` to compile the workspace.
    -   `cargo test` to run tests.
    -   `cargo run --bin blg` (or similar) to run the CLI tool.

## Technical Constraints
-   The library and CLI must interact with the Backlog API, so network connectivity to the Backlog instance is required at runtime.
-   API rate limits imposed by Backlog need to be considered (the `ApiRateLimit` struct in `backlog-api-core` suggests this is being handled).

## Tool Usage Patterns
-   **`list_code_definition_names`**: Useful for getting a high-level overview of structs, enums, and functions within each module, especially for understanding the public API of each crate.
    -   Many `src` directories in the API module crates (`backlog-issue`, `backlog-project`, etc.) did not show top-level definitions. This implies that their primary `lib.rs` might be focused on re-exporting modules from subdirectories like `api/`, `models/`, `requests/`, and `responses/`. Further investigation into these subdirectories would be needed to map out all specific API calls and data structures.
-   **`list_files` (recursive)**: Essential for understanding the module structure within each crate (e.g., `api/`, `models/`, `requests/`, `responses/` subdirectories).
-   **`read_file`**: Critical for understanding `Cargo.toml` files (both workspace and individual crate level) to determine dependencies and crate features. Also used for reading `Readme.md` or other documentation if available.
