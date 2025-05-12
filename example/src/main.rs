fn main() {
    let world_data = world_data::load();
    let de = world_data.countries.get("DE").unwrap();
    println!("{:#?}", de);
}
