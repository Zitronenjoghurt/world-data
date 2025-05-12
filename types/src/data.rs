use crate::data::country::Country;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod country;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldData {
    /// Countries mapped by their ISO_A2 country code
    pub countries: HashMap<String, Country>,
}
