# world-data

A rust crate containing country shapes as polygons, capital city positions and more.

## Usage

```rust
fn main() {
    let data = world_data::load();
    let country_us = data.countries.get("US").unwrap();
    println!("{}", country_us.official_name);

    let capital_de = data.capitals.get("Berlin").unwrap();
    println!("{:#?}", capital_de);
}
```

## Sources

- Country metadata JSON: https://restcountries.com
- Country flags: https://github.com/hampusborgos/country-flags
- geojson files: https://github.com/nvkelso/natural-earth-vector