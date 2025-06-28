use mcp_backlog_server::access_control::AccessControl;
use std::env;

#[test]
fn test_access_control_disabled_by_default() {
    unsafe {
        env::remove_var("BACKLOG_PROJECTS");
    }
    let access_control = AccessControl::new().unwrap();

    // When environment variable is not set, all projects are accessible
    assert!(access_control.check_project_access("ANY_PROJECT").is_ok());
    assert!(access_control.check_project_access("123456").is_ok());
    assert!(access_control.check_project_access("PROJECT_XYZ").is_ok());
}

#[test]
fn test_access_control_with_project_keys() {
    unsafe {
        env::set_var("BACKLOG_PROJECTS", "PROJECT_A,PROJECT_B,PROJECT_C");
    }
    let access_control = AccessControl::new().unwrap();

    // Allowed projects are accessible
    assert!(access_control.check_project_access("PROJECT_A").is_ok());
    assert!(access_control.check_project_access("PROJECT_B").is_ok());
    assert!(access_control.check_project_access("PROJECT_C").is_ok());

    // Disallowed projects are denied access
    assert!(access_control.check_project_access("PROJECT_D").is_err());
    assert!(access_control.check_project_access("UNKNOWN").is_err());
}

#[test]
fn test_access_control_with_project_ids() {
    unsafe {
        env::set_var("BACKLOG_PROJECTS", "123456,789012");
    }
    let access_control = AccessControl::new().unwrap();

    // Allowed project IDs are accessible
    assert!(access_control.check_project_access("123456").is_ok());
    assert!(access_control.check_project_access("789012").is_ok());

    // Disallowed project IDs are denied access
    assert!(access_control.check_project_access("999999").is_err());
    assert!(access_control.check_project_access("111111").is_err());
}

#[test]
fn test_access_control_mixed_keys_and_ids() {
    unsafe {
        env::set_var("BACKLOG_PROJECTS", "PROJECT_A, 123456, PROJECT_C");
    }
    let access_control = AccessControl::new().unwrap();

    // Mixed project keys and IDs
    assert!(access_control.check_project_access("PROJECT_A").is_ok());
    assert!(access_control.check_project_access("123456").is_ok());
    assert!(access_control.check_project_access("PROJECT_C").is_ok());

    // Disallowed ones are denied access
    assert!(access_control.check_project_access("PROJECT_B").is_err());
    assert!(access_control.check_project_access("999999").is_err());
}

#[test]
fn test_access_control_empty_string() {
    unsafe {
        env::set_var("BACKLOG_PROJECTS", "");
    }
    let access_control = AccessControl::new().unwrap();

    // With empty string, all projects are accessible
    assert!(access_control.check_project_access("ANY_PROJECT").is_ok());
}

#[test]
fn test_access_control_whitespace_handling() {
    unsafe {
        env::set_var("BACKLOG_PROJECTS", " PROJECT_A , PROJECT_B , ");
    }
    let access_control = AccessControl::new().unwrap();

    // Leading and trailing spaces are ignored
    assert!(access_control.check_project_access("PROJECT_A").is_ok());
    assert!(access_control.check_project_access("PROJECT_B").is_ok());
    assert!(access_control.check_project_access("PROJECT_C").is_err());
}

#[test]
fn test_error_message_format() {
    unsafe {
        env::set_var("BACKLOG_PROJECTS", "PROJECT_A,PROJECT_B");
    }
    let access_control = AccessControl::new().unwrap();

    let result = access_control.check_project_access("PROJECT_X");
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_message = error.to_string();
    assert!(error_message.contains("Access denied to project 'PROJECT_X'"));
    assert!(error_message.contains("Allowed projects"));
}

#[test]
fn test_invalid_project_key() {
    unsafe {
        env::set_var("BACKLOG_PROJECTS", "valid-project-key");
    }
    // Project keys containing hyphens are invalid
    let result = AccessControl::new();
    assert!(result.is_err());
}
