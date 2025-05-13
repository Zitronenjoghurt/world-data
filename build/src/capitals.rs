use geo::Coord;
use geojson::GeoJson;
use geojson::Value::Point;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use world_data_types::data::capital::Capital;

pub fn build_capitals(
    capital_names: &HashSet<String>,
    data_path: &Path,
) -> HashMap<String, Capital> {
    let populated_places = get_populated_places_geojson(data_path);

    let mut capitals = HashMap::new();

    let GeoJson::FeatureCollection(collection) = populated_places else {
        return capitals;
    };

    for feature in collection.features {
        let Some(properties) = feature.properties else {
            continue;
        };

        let Some(name) = properties.get("NAME").and_then(|v| v.as_str()) else {
            continue;
        };

        if !capital_names.contains(name) {
            continue;
        }

        let Some(geometry) = feature.geometry else {
            continue;
        };

        let Point(position) = geometry.value else {
            continue;
        };

        let coordinates = Coord::from((position[0] as f32, position[1] as f32));
        let capital = Capital {
            name: name.to_owned(),
            coordinates,
        };

        capitals.insert(name.to_owned(), capital);
    }

    capitals
}

fn get_populated_places_geojson(data_path: &Path) -> GeoJson {
    let file_path = data_path.join("ne_10m_populated_places.geojson");
    let content_string = std::fs::read_to_string(file_path).unwrap();
    content_string.parse::<GeoJson>().unwrap()
}
