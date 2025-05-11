use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::data::country::Country;

pub mod country;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldData {
    pub countries: HashMap<String, Country>
}