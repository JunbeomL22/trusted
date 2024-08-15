use crate::InstId;
use crate::Real;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

pub struct Account {
    pub id: InstId,
    pub cash: Real,
    pub position: Real,
}