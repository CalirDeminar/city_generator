use city::{city::{ export_city}};

pub mod names;
pub mod data_parser;
pub mod city;
pub mod templater;
pub mod utils;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let city = city::city::build(250);
    export_city(&city);
}
