use geo::Polygon;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub common_name: String,
    pub official_name: String,
    pub region: String,
    pub subregion: Option<String>,
    pub continents: Vec<String>,
    /// ISO-A2 code of bordering countries
    pub bordering_countries: Vec<String>,
    /// The area of the country in square kilometers
    pub area: u32,
    pub population: u32,
    pub iso_a2: String,
    pub iso_a3: String,
    pub is_enclave: bool,
    pub is_landlocked: bool,
    pub is_independent: bool,
    pub is_un_member: bool,
    pub capitals: Vec<String>,
    pub top_level_domains: Vec<String>,
    pub polygons: Vec<Polygon<f32>>,
}
