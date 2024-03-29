use city::{
    city::{export_city, export_city_stories},
    html_exporter::html_exporter::export_city_html,
};
use language::language::Era;

pub mod city;
pub mod culture;
pub mod language;
pub mod language2;
pub mod names;
pub mod parser;
pub mod templater;
pub mod utils;

pub const MULTI_THREADING_FACTOR: usize = 5;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let city = city::city::simulate(250, 150, Some(Era::Modern));
    export_city(&city);
    export_city_html(&city);
    export_city_stories(&city);
}
