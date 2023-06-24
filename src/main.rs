use city::city::{export_city, export_city_html};

pub mod city;
pub mod culture;
pub mod data_parser;
pub mod language;
pub mod names;
pub mod parser;
pub mod templater;
pub mod utils;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let city = city::city::simulate(500, 20);
    export_city(&city);
    export_city_html(&city);
}
