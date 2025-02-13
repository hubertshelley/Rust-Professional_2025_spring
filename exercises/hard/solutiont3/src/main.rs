// I AM NOT DONE

mod district;
pub mod graph_parser;
mod json_parser;

fn main() {
    let provinces = district::count_provinces();
    println!("provinces: {provinces}");
}
