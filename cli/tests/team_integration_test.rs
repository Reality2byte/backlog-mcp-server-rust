use std::process::Command;

#[test]
#[ignore] // Requires actual API credentials
fn test_team_list_command() {
    // Check if environment variables are set
    if std::env::var("BACKLOG_BASE_URL").is_err() || std::env::var("BACKLOG_API_KEY").is_err() {
        eprintln!("Skipping integration test: BACKLOG_BASE_URL and BACKLOG_API_KEY not set");
        return;
    }

    // Test basic list command
    let output = Command::new("cargo")
        .arg("run")
        .arg("--features")
        .arg("team")
        .arg("--")
        .arg("team")
        .arg("list")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Team list output:\n{stdout}");
        // Should contain table headers or "No teams found"
        assert!(stdout.contains("ID") || stdout.contains("No teams found"));
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed: {stderr}");
        // If it's a permission error, that's expected
        assert!(stderr.contains("permission") || stderr.contains("403"));
    }
}

#[test]
#[ignore]
fn test_team_list_with_format() {
    if std::env::var("BACKLOG_BASE_URL").is_err() || std::env::var("BACKLOG_API_KEY").is_err() {
        return;
    }

    // Test JSON format
    let output = Command::new("cargo")
        .arg("run")
        .arg("--features")
        .arg("team")
        .arg("--")
        .arg("team")
        .arg("list")
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should be valid JSON array
        assert!(stdout.trim().starts_with('[') || stdout.trim() == "[]");
    }

    // Test CSV format
    let output = Command::new("cargo")
        .arg("run")
        .arg("--features")
        .arg("team")
        .arg("--")
        .arg("team")
        .arg("list")
        .arg("--format")
        .arg("csv")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should have CSV header
        assert!(stdout.contains("id,name,member_count,created,updated"));
    }
}

#[test]
#[ignore]
fn test_team_list_with_pagination() {
    if std::env::var("BACKLOG_BASE_URL").is_err() || std::env::var("BACKLOG_API_KEY").is_err() {
        return;
    }

    // Test with pagination parameters
    let output = Command::new("cargo")
        .arg("run")
        .arg("--features")
        .arg("team")
        .arg("--")
        .arg("team")
        .arg("list")
        .arg("--count")
        .arg("5")
        .arg("--offset")
        .arg("0")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success() || output.status.code() == Some(1));
}
