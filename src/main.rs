use city::population::population::output_population;

pub mod names;
pub mod data_parser;
pub mod city;

fn main() {
    let city = city::city::build(500);
    output_population(city.citizens, city.institutions);
}
