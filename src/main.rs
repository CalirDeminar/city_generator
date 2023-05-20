use city::{city::{print_city, export_city}};

pub mod names;
pub mod data_parser;
pub mod city;

fn main() {
    let city = city::city::build(250);
    export_city(&city);
}
