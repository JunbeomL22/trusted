use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use crate::definitions::Real;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CloseData {
    value: HashMap<OffsetDateTime, Real>,
    name: String,
    code: String,
}

impl Default for CloseData {
    fn default() -> Self {
        CloseData {
            value: HashMap::new(),
            name: String::new(),
            code: String::new(),
        }
    }
}

impl CloseData {
    pub fn new(value: HashMap<OffsetDateTime, Real>, name: String, code: String) -> CloseData {
        CloseData {
            value,
            name,
            code,
        }
    }

    pub fn get_value(&self) -> &HashMap<OffsetDateTime, Real> {
        &self.value
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }
}