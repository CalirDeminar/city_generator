pub mod mind;
pub mod population {
    use std::{fs::File, io::Write};
    use crate::city::institutions::institutions::Institution;
    use crate::city::population::mind::mind::*;
    use crate::city::population::mind::relations::relations::*;
    use crate::names::names::*;

    pub type Population = Vec<Mind>;

    fn print_population(population: &Population, institutions: &Vec<Institution>) -> String {
        let mut output = String::from("");
        for mind in population {
            output.push_str(&print_mind(&mind, &population, &institutions));
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

    pub fn output_population(population: Population, institutions: Vec<Institution>) {
        let mut file = File::create("./export.txt").unwrap();
        let pop_log = print_population(&population, &institutions);
        file.write_all(pop_log.into_bytes().as_slice()).unwrap();
    }
}