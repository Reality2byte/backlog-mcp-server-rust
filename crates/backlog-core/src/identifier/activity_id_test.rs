#[cfg(test)]
mod tests {
    use crate::identifier::{ActivityId, Identifier};
    use std::str::FromStr;

    #[test]
    fn test_activity_id_creation() {
        let id = ActivityId::new(123456789);
        assert_eq!(id.value(), 123456789);
    }

    #[test]
    fn test_activity_id_from_u32() {
        let id: ActivityId = 987654321.into();
        assert_eq!(id.value(), 987654321);
    }

    #[test]
    fn test_activity_id_from_str() {
        let id = ActivityId::from_str("555555555").unwrap();
        assert_eq!(id.value(), 555555555);
    }

    #[test]
    fn test_activity_id_from_str_invalid_negative() {
        let result = ActivityId::from_str("-123456");
        assert!(result.is_err());
    }

    #[test]
    fn test_activity_id_from_str_invalid() {
        let result = ActivityId::from_str("not_a_number");
        assert!(result.is_err());
    }

    #[test]
    fn test_activity_id_display() {
        let id = ActivityId::new(789012345);
        assert_eq!(format!("{id}"), "789012345");
    }

    #[test]
    fn test_activity_id_serialization() {
        let id = ActivityId::new(111222333);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "111222333");
    }

    #[test]
    fn test_activity_id_deserialization() {
        let id: ActivityId = serde_json::from_str("444555666").unwrap();
        assert_eq!(id.value(), 444555666);
    }
}
