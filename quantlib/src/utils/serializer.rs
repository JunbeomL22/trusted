use time::OffsetDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
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
    use time::macros::datetime;
    use serde_json::{to_string, from_str};

    #[test]
    fn test_mock_serialization() {
        let mock = Mock::new(
            "test".to_string(), 
            datetime!(2021-01-01 00:00:00 +0000)
        );
        let serialized = to_string(&mock).unwrap();    
        let deserialized: Mock = from_str(&serialized).unwrap();
        assert_eq!(mock, deserialized);
    
    }
}
