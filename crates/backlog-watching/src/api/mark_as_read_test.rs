#[cfg(test)]
mod mark_as_read_tests {
    use backlog_api_core::IntoRequest;
    use backlog_core::identifier::{Identifier, WatchingId};

    use crate::api::mark_as_read::MarkAsReadParams;

    #[test]
    fn test_params_creation() {
        let params = MarkAsReadParams::new(WatchingId::from(123));
        assert_eq!(params.watching_id.value(), 123);
    }

    #[test]
    fn test_path_generation() {
        let params = MarkAsReadParams::new(WatchingId::from(456));
        assert_eq!(params.path(), "/api/v2/watchings/456/markAsRead");
    }

    #[test]
    fn test_no_query_params() {
        let params = MarkAsReadParams::new(WatchingId::from(789));
        // Mark as read has no query parameters
        assert_eq!(params.watching_id, WatchingId::from(789));
    }
}
