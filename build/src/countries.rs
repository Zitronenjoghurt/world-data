use crate::geometry::parse_geometry;
use crate::models::all_countries_entry::AllCountriesEntry;
use crate::models::countries_extra::CountriesExtra;
use geojson::GeoJson;
use std::collections::HashMap;
use std::path::Path;
use world_data_types::data::country::Country;

pub fn build_countries(data_path: &Path) -> HashMap<String, Country> {
    let countries_geojson = get_countries_geojson(data_path);
    let all_countries = get_all_countries_data(data_path);
    let countries_extra = get_countries_extra_data(data_path);

    let map_a3_to_a2: HashMap<String, String> = all_countries
        .values()
        .map(|v| (v.cca3.clone(), v.cca2.clone()))
        .collect();

    let mut countries = HashMap::new();

    let GeoJson::FeatureCollection(collection) = countries_geojson else {
        return countries;
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

        let Some(data) = all_countries.get(code).cloned() else {
            continue;
        };

        let is_enclave = countries_extra.enclaves.contains(code);
        let polygons = parse_geometry(&geometry);

        let bordering_countries: Vec<String> = data
            .borders
            .iter()
            .map(|c| map_a3_to_a2[c].clone())
            .collect();

        let country = Country {
            common_name: data.name.common,
            official_name: data.name.official,
            region: data.region,
            subregion: data.subregion,
            continents: data.continents,
            bordering_countries,
            area: data.area as u32,
            population: data.population,
            iso_a2: data.cca2,
            iso_a3: data.cca3,
            is_enclave,
            is_landlocked: data.landlocked,
            is_independent: data.independent,
            is_un_member: data.un_member,
            capitals: data.capital,
            top_level_domains: data.tld,
            flag_svg: get_country_flag(data_path, code),
            polygons,
        };

        countries.insert(code.to_owned(), country);
    }

    countries
}

fn get_countries_geojson(data_path: &Path) -> GeoJson {
    let file_path = data_path.join("ne_10m_admin_0_countries.geojson");
    let content_string = std::fs::read_to_string(file_path).unwrap();
    content_string.parse::<GeoJson>().unwrap()
}

fn get_all_countries_data(data_path: &Path) -> HashMap<String, AllCountriesEntry> {
    let file_path = data_path.join("all_countries.json");
    let entries: Vec<AllCountriesEntry> =
        serde_json::from_reader(std::fs::File::open(file_path).unwrap()).unwrap();
    entries
        .into_iter()
        .map(|entry| (entry.cca2.clone(), entry))
        .collect()
}

fn get_countries_extra_data(data_path: &Path) -> CountriesExtra {
    let file_path = data_path.join("countries_extra.json");
    serde_json::from_reader(std::fs::File::open(file_path).unwrap()).unwrap()
}

fn get_country_flag(data_path: &Path, code: &str) -> Vec<u8> {
    let flags_path = data_path.join("flags");
    let path = flags_path.join(format!("{}.svg", code.to_lowercase()));
    std::fs::read(path).unwrap_or_else(|_| panic!("Did not find flag svg for country: {}", code))
}
