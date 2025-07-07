#[cfg(test)]
mod update_watching_tests {
    use backlog_api_core::IntoRequest;
    use backlog_core::identifier::{Identifier, WatchingId};

    use crate::api::update_watching::UpdateWatchingParams;

    #[test]
    fn test_params_creation() {
        let params = UpdateWatchingParams::new(WatchingId::from(123));
        assert_eq!(params.watching_id.value(), 123);
        assert_eq!(params.note, None);
    }

    #[test]
    fn test_params_with_note() {
        let params = UpdateWatchingParams::new(WatchingId::from(456)).with_note("Updated note");
        assert_eq!(params.watching_id.value(), 456);
        assert_eq!(params.note, Some("Updated note".to_string()));
    }

    #[test]
    fn test_path_generation() {
        let params = UpdateWatchingParams::new(WatchingId::from(789));
        assert_eq!(params.path(), "/api/v2/watchings/789");
    }

    #[test]
    fn test_form_serialization_with_note() {
        let params = UpdateWatchingParams::new(WatchingId::from(123)).with_note("New note content");
        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 1);
        assert!(form_data.contains(&("note".to_string(), "New note content".to_string())));
    }

    #[test]
    fn test_form_serialization_without_note() {
        let params = UpdateWatchingParams::new(WatchingId::from(456));
        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 0);
    }
}
