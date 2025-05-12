use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct CountriesExtra {
    pub enclaves: HashSet<String>,
}
