use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn tools_list_applies_prefix() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mcp-backlog-server"))
        .env("BACKLOG_BASE_URL", "https://example.backlog.jp")
        .env("BACKLOG_API_KEY", "dummy")
        .env("BACKLOG_PREFIX", "test_")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn server");

    let mut stdin = child.stdin.take().unwrap();
    for line in [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    ] {
        writeln!(stdin, "{line}").unwrap();
    }

    let (tx, rx) = mpsc::channel();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    thread::spawn(move || {
        let mut line = String::new();
        while stdout.read_line(&mut line).unwrap_or(0) > 0 {
            let value: serde_json::Value = serde_json::from_str(&line).unwrap();
            if value["id"] == 2 {
                let _ = tx.send(value);
                break;
            }
            line.clear();
        }
    });
    let response = rx.recv_timeout(Duration::from_secs(30));
    drop(stdin);
    let _ = child.kill();
    let _ = child.wait();

    let response = response.expect("tools/list response");
    let names: Vec<&str> = response["result"]["tools"]
        .as_array()
        .expect("tools array")
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect();
    assert!(!names.is_empty());
    assert!(
        names.iter().all(|name| name.starts_with("test_")),
        "unprefixed tools: {names:?}"
    );
}
