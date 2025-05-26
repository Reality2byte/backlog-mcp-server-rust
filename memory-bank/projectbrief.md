# Project Brief

## Core Goal
This project aims to develop a comprehensive Rust client library for interacting with the Backlog API. It also appears to include a command-line interface (CLI) tool, likely named `blg`, for using this library.

## Key Features
-   **API Client Library**: A set of Rust crates (`backlog-api-client`, `backlog-issue`, `backlog-project`, etc.) to provide programmatic access to Backlog API functionalities.
-   **Core Data Models**: A `backlog-core` crate defining common data structures (e.g., `IssueKey`, `ProjectKey`, `User`) used across the API.
-   **Modular Design**: The API functionalities are broken down into modules/crates (e.g., `issue`, `project`, `space`, `user`).
-   **Authentication**: Support for API key and potentially other authentication methods for the Backlog API.
-   **Generic HTTP Client**: A foundational `client` crate providing generic HTTP request capabilities.
-   **Command-Line Interface (CLI)**: A tool (`blg.rs`) for interacting with the Backlog API from the command line, built upon the client library.
-   **MCP Server**: A dedicated server (`mcp-backlog-server` crate) exposing Backlog functionalities as tools via the Model Context Protocol, enabling integration with MCP-compatible clients.

## Scope
-   Provide comprehensive coverage of the Backlog API, segmented by resource types (issues, projects, users, space).
-   Offer robust and well-typed Rust interfaces for API interactions.
-   The project is structured as a Rust workspace with multiple interdependent crates.
