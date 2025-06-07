# Backlog API Client CLI (`blg`)

`blg` is a command-line interface (CLI) tool for interacting with the Backlog API. It is built using the `backlog-api-client` Rust library.

## Building `blg`

### Prerequisites

-   Rust and Cargo: Ensure you have a recent version of Rust and Cargo installed. You can get them from [rustup.rs](https://rustup.rs/).

### Build Command

To build the `blg` executable, navigate to the workspace root (`/Users/mac/src/_mydev/backlog-api-client`) and run:

```bash
cargo build --package backlog-api-client --features "cli git issue" --bin blg
```

Alternatively, if you are in the `backlog-api-client` directory:

```bash
cargo build --features "cli git issue" 
```

The `cli`, `git`, and `issue` features are required to build the `blg` binary as specified in its `Cargo.toml`. The executable will be located at `target/debug/blg` (or `target/release/blg` if you add `--release`).

## Configuration

`blg` requires two environment variables to be set for authentication:

-   `BACKLOG_BASE_URL`: The base URL of your Backlog space (e.g., `https://your-space.backlog.jp`).
-   `BACKLOG_API_KEY`: Your Backlog API key. You can generate one from your personal settings page in Backlog.

Example:

```bash
export BACKLOG_BASE_URL="https://yourspace.backlog.jp"
export BACKLOG_API_KEY="yourgeneratedapikey"
```

## Basic Usage

The general syntax for `blg` is:

```bash
blg <COMMAND> <SUBCOMMAND> [OPTIONS]
```

You can get help for any command or subcommand by appending `--help`.

### Examples:

**List issues for a project:**
(Assuming `MYPROJ` is your project key)
```bash
blg issue list --project-id MYPROJ 
```

**Show details of a specific pull request:**
(Assuming PR #42 in repository `my-repo` under project `MYPROJ`)
```bash
blg pr show --project-id MYPROJ --repo-id my-repo --pr-number 42
```

**Download an issue attachment:**
(Assuming issue `MYPROJ-101`, attachment ID `12345`, save to `downloaded_file.dat`)
```bash
blg issue download-attachment MYPROJ-101 12345 --output downloaded_file.dat
```

**Download a pull request attachment:**
(Assuming project `MYPROJ`, repo `my-repo`, PR #42, attachment ID `56789`, save to `pr_attachment.zip`)
```bash
blg pr download-attachment -p MYPROJ -r my-repo -n 42 -a 56789 -o pr_attachment.zip
```

### Getting Help

-   For a list of all top-level commands:
    ```bash
    blg --help
    ```
-   For help with a specific command (e.g., `issue`):
    ```bash
    blg issue --help
    ```
-   For help with a specific subcommand (e.g., `issue list`):
    ```bash
    blg issue list --help
