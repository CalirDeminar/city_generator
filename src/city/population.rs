pub mod mind;
pub mod population {
    use std::{fs::File, io::Write};
    use crate::city::city::City;
    use crate::city::institutions::institutions::Institution;
    use crate::city::population::mind::mind::*;
    use crate::city::population::mind::relations::relations::*;
    use crate::names::names::*;

    pub type Population = Vec<Mind>;

    pub fn print_population(city: &City) -> String {
        let mut output = String::from("");
        for mind in &city.citizens {
            output.push_str(&print_mind(&mind, &city));
        }
        return output;
    }

    fn generate_base_population<'a>(i: usize, name_dict: &NameDictionary) -> Population {
        let mut output: Population = vec![];
        for _i in 0..i {
            output.push(random_char(&name_dict));
        }
        return output;
    }

    pub fn generate_population(name_dict: &NameDictionary, size: usize) -> Population {
        let mut population = generate_base_population(size, &name_dict);
        population = add_partners_to_population(population, &name_dict);
        population = add_parents_to_population(population, &name_dict);
        population = link_friends_within_population(population);
        return population
    }

    pub fn output_population(city: &City) {
        let mut file = File::create("./export.txt").unwrap();
        let pop_log = print_population(&city);
        file.write_all(pop_log.into_bytes().as_slice()).unwrap();
    }
}