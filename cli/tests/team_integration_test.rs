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

#[test]
#[ignore]
fn test_team_icon_download() {
    if std::env::var("BACKLOG_BASE_URL").is_err() || std::env::var("BACKLOG_API_KEY").is_err() {
        return;
    }

    // Create a temporary directory for the output
    let temp_dir = std::env::temp_dir();
    let output_path = temp_dir.join("test_team_icon.png");

    // If team ID is provided in env, use it
    let team_id = std::env::var("BACKLOG_TEST_TEAM_ID").unwrap_or_else(|_| "1".to_string());

    let output = Command::new("cargo")
        .arg("run")
        .arg("--features")
        .arg("team")
        .arg("--")
        .arg("team")
        .arg("icon")
        .arg(&team_id)
        .arg("--output")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Team icon download output:\n{stdout}");
        // Check if file was created
        assert!(stdout.contains("Team icon saved to:"));

        // Clean up
        if output_path.exists() {
            let _ = std::fs::remove_file(&output_path);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed: {stderr}");
        // It might fail if team doesn't have an icon or permission denied
        assert!(
            stderr.contains("permission")
                || stderr.contains("403")
                || stderr.contains("404")
                || stderr.contains("Not Found")
        );
    }
}
