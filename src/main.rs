use city::city::{export_city, export_city_html};

pub mod city;
pub mod data_parser;
pub mod language;
pub mod names;
pub mod parser;
pub mod templater;
pub mod utils;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let city = city::city::build(1000);
    export_city(&city);
    export_city_html(&city);
}
