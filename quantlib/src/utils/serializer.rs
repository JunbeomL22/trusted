use time::{OffsetDateTime, format_description};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mock {
    pub name: String,
    pub dt: OffsetDateTime,
}

impl Mock {
    pub fn new(name: String, dt: OffsetDateTime) -> Mock {
        Mock {
            name,
            dt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_string, from_str};
    #[test]
    fn test_mock_serialization() {
        let mock = Mock::new("test".to_string(), OffsetDateTime::parse("2021-01-01T00:00:00+00:00", &format_description!(""))).unwrap();
        let serialized = to_string(&mock).unwrap();
        let deserialized: Mock = from_str(&serialized).unwrap();
        assert_eq!(mock, deserialized);
    }
}
