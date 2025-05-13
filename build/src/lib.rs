use std::collections::HashSet;
use std::path::Path;
use world_data_types::data::WorldData;

mod capitals;
mod countries;
mod geometry;
mod models;

pub fn build_data(data_path: &Path) -> WorldData {
    let countries = countries::build_countries(data_path);
    let capital_names: HashSet<String> = countries
        .values()
        .flat_map(|country| country.capitals.clone())
        .collect();
    let capitals = capitals::build_capitals(&capital_names, data_path);

    WorldData {
        countries,
        capitals,
    }
}
