use city::city::{export_city, export_city_html};

pub mod city;
pub mod culture;
pub mod data_parser;
pub mod language;
pub mod names;
pub mod parser;
pub mod templater;
pub mod utils;

pub const MULTI_THREADING_FACTOR: usize = 5;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let city = city::city::simulate(25, 200);
    export_city(&city);
    export_city_html(&city);
}
