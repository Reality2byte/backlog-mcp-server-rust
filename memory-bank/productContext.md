# Product Context

## Problem Solved
Interacting with the Backlog API directly can be cumbersome, requiring manual HTTP request construction, response parsing, and error handling. For Rust developers, there's a need for a type-safe, idiomatic, and easy-to-use interface to streamline these interactions. Similarly, users who prefer command-line tools need a convenient way to perform Backlog operations without writing code.

## How It Should Work
-   **As a Library**: Developers should be able to easily integrate the Rust crates into their applications. They should instantiate a `BacklogApiClient`, configure authentication, and then call methods corresponding to Backlog API endpoints. For Git/PR functionality, they would use `client.git().list_repositories(...)`, `client.git().get_pull_request(...)`, etc. These methods accept well-defined Rust types (e.g., `ProjectIdOrKey`, `&str` for repo name/ID, `u64` for PR number) as input and return `Result` types containing parsed Rust structs (`Repository`, `PullRequest`) or appropriate errors.
-   **As a CLI Tool (`blg`)**: Users should be able to invoke `blg` from their terminal with subcommands and arguments that map to Backlog API actions. For Git/PRs, this includes commands like `blg repo list --project-id <id_or_key>`, `blg pr show --project-id <id_or_key> --repo-id <repo> --pr-number <num>`. The CLI handles authentication via environment variables (`BACKLOG_BASE_URL`, `BACKLOG_API_KEY`) and presents API responses in a user-friendly (currently debug, to be improved) format.
-   **As an MCP Server (`mcp-backlog-server`)**: Provides tools (e.g., `get_issue_details`, `get_document_details`, and now `get_repository_list`, `get_pull_request_details`) that can be invoked by MCP clients. This allows for automated or assisted interaction with Backlog's Git and PR features. The server handles API authentication via environment variables configured in the MCP settings. Input for Git/PR tools includes project identifiers, repository identifiers, and PR numbers. Output is structured JSON representing the requested resources.

## User Experience Goals
-   **Developer Experience (Library)**:
    -   **Ease of Use**: Simple and intuitive API.
    -   **Type Safety**: Leverage Rust's type system to prevent common errors at compile time.
    -   **Ergonomics**: Methods and data structures should feel natural to Rust developers.
    -   **Comprehensive Coverage**: Aim to support all relevant Backlog API endpoints.
    -   **Good Documentation**: Clear examples and API documentation (rustdoc).
    -   **Robust Error Handling**: Clear and actionable error messages.
-   **User Experience (CLI)**:
    -   **Simplicity**: Easy-to-understand commands and options.
    -   **Efficiency**: Allow users to perform Backlog tasks quickly from the terminal.
    -   **Discoverability**: Helpful `--help` messages for commands and subcommands.
    -   **Clear Output**: Formatted output that is easy to read and parse if needed.
-   **MCP Tool User Experience (e.g., for an AI Assistant)**:
    -   **Well-defined Tools**: Clear names, descriptions, and input/output schemas for each tool.
    -   **Reliability**: Tools should robustly interact with the Backlog API and handle errors gracefully.
    -   **Actionability**: Tool outputs should provide clear, structured data that the MCP client can easily use.
