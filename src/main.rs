use city::{city::{print_city, export_city}};

pub mod names;
pub mod data_parser;
pub mod city;
pub mod templater;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let city = city::city::build(250);
    export_city(&city);
}
