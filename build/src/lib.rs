mod geometry;

use std::collections::HashMap;
use std::path::{Path};
use geojson::GeoJson;
use world_data_types::data::country::Country;
use world_data_types::data::WorldData;
use crate::geometry::parse_geometry;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

pub fn build_data(data_path: &Path) -> WorldData {
    let countries_geojson_path = data_path.join("ne_10m_admin_0_countries.geojson");
    let countries_geojson_string = std::fs::read_to_string(countries_geojson_path).unwrap();
    let countries_geojson = countries_geojson_string.parse::<GeoJson>().unwrap();

    let mut world_data = WorldData {
        countries: HashMap::new()
    };

    let GeoJson::FeatureCollection(collection) = countries_geojson else {
        return world_data;
    };

    for feature in collection.features {
        let Some(properties) = feature.properties else {
            continue;
        };

        let Some(code) = properties.get("ISO_A2").and_then(|v| v.as_str()) else {
            continue;
        };

        let Some(geometry) = feature.geometry else {
            continue;
        };

        let polygons = parse_geometry(&geometry);
        let country = Country {
            polygons,
        };

        world_data.countries.insert(code.to_owned(), country);
    }

    world_data
}

