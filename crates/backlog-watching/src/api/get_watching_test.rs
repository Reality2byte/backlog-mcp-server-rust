#[cfg(test)]
mod get_watching_tests {
    use backlog_api_core::IntoRequest;
    use backlog_core::identifier::{Identifier, WatchingId};

    use crate::api::get_watching::GetWatchingParams;

    #[test]
    fn test_params_creation() {
        let params = GetWatchingParams::new(WatchingId::from(123));
        assert_eq!(params.watching_id.value(), 123);
    }

    #[test]
    fn test_path_generation() {
        let params = GetWatchingParams::new(WatchingId::from(456));
        assert_eq!(params.path(), "/api/v2/watchings/456");
    }

    #[test]
    fn test_no_query_params() {
        let params = GetWatchingParams::new(WatchingId::from(789));
        let query = serde_json::to_value(params.to_query()).unwrap();
        assert!(query.as_object().unwrap().is_empty());
    }
}
