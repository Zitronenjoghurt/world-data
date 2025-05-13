fn main() {
    let world_data = world_data::load();
    let de = world_data.countries.get("DE").unwrap();
    println!("{:#?}", de);

    let berlin = world_data.capitals.get("Berlin").unwrap();
    println!("{:#?}", berlin);
}
