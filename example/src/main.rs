use world_data::load_world_data;

fn main() {
    let world_data = load_world_data();
    let de = world_data.countries.get("DE").unwrap();
    println!("{:?}", de);
}
