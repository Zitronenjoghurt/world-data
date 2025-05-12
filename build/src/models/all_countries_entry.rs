use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AllCountriesEntryName {
    pub common: String,
    pub official: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AllCountriesEntryMaps {
    #[serde(alias = "googleMaps")]
    pub google_maps: String,
    #[serde(alias = "openStreetMaps")]
    pub open_street_maps: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AllCountriesEntry {
    pub name: AllCountriesEntryName,
    #[serde(default)]
    pub tld: Vec<String>,
    pub cca2: String,
    pub ccn3: String,
    pub cca3: String,
    pub independent: bool,
    #[serde(alias = "unMember")]
    pub un_member: bool,
    #[serde(default)]
    pub capital: Vec<String>,
    #[serde(alias = "altSpellings")]
    pub alt_spellings: Vec<String>,
    pub region: String,
    pub subregion: Option<String>,
    pub landlocked: bool,
    #[serde(default)]
    pub borders: Vec<String>,
    pub area: f64,
    pub maps: AllCountriesEntryMaps,
    pub population: u32,
    pub continents: Vec<String>,
}
