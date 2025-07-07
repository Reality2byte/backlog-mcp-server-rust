#[cfg(test)]
mod add_watching_tests {
    use backlog_api_core::IntoRequest;
    use backlog_core::{IssueIdOrKey, IssueKey};
    use std::str::FromStr;

    use crate::api::add_watching::AddWatchingParams;

    #[test]
    fn test_params_with_issue_id() {
        let params = AddWatchingParams::new(IssueIdOrKey::Id(123.into()));
        assert_eq!(params.issue_id_or_key.to_string(), "123");
        assert_eq!(params.note, None);
    }

    #[test]
    fn test_params_with_issue_key() {
        let params =
            AddWatchingParams::new(IssueIdOrKey::Key(IssueKey::from_str("PROJ-123").unwrap()));
        assert_eq!(params.issue_id_or_key.to_string(), "PROJ-123");
        assert_eq!(params.note, None);
    }

    #[test]
    fn test_params_with_note() {
        let params =
            AddWatchingParams::new(IssueIdOrKey::Id(456.into())).with_note("Important to track");
        assert_eq!(params.issue_id_or_key.to_string(), "456");
        assert_eq!(params.note, Some("Important to track".to_string()));
    }

    #[test]
    fn test_path_generation() {
        let params = AddWatchingParams::new(IssueIdOrKey::Id(123.into()));
        assert_eq!(params.path(), "/api/v2/watchings");
    }

    #[test]
    fn test_form_serialization_minimal() {
        let params = AddWatchingParams::new(IssueIdOrKey::Id(789.into()));
        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 1);
        assert!(form_data.contains(&("issueIdOrKey".to_string(), "789".to_string())));
    }

    #[test]
    fn test_form_serialization_with_note() {
        let params =
            AddWatchingParams::new(IssueIdOrKey::Key(IssueKey::from_str("TEST-99").unwrap()))
                .with_note("Watch this carefully");
        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 2);
        assert!(form_data.contains(&("issueIdOrKey".to_string(), "TEST-99".to_string())));
        assert!(form_data.contains(&("note".to_string(), "Watch this carefully".to_string())));
    }
}
