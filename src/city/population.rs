pub mod mind;
pub mod population {
    use uuid::Uuid;

    use crate::city::population::mind::mind::*;
    use crate::city::population::mind::relations::relations::*;
    use crate::{city::city::City, language::language::Word};
    use std::collections::HashMap;
    use std::{fs::File, io::Write};

    use super::mind::relations::friends::friends::link_friends_within_population;
    use super::mind::relations::{
        parents::parents::link_parents, partners::partners::link_partners,
    };

    pub type Population = HashMap<Uuid, Mind>;

    pub fn print_population(city: &City) -> String {
        let mut output = String::from("");
        for mind in city.citizens.values().filter(|c| c.alive) {
            output.push_str(&print_mind(&mind, &city));
        }
        return output;
    }

    fn generate_base_population<'a>(i: usize, dict: &Vec<Word>) -> Population {
        let mut output: Population = HashMap::new();
        for _i in 0..i {
            let char = random_char(&dict);
            output.insert(char.id.clone(), char);
        }
        return output;
    }

    pub fn generate_population_full_relation<'a>(
        dict: &Vec<Word>,
        size: usize,
        c: &'a mut City,
    ) -> &'a mut City {
        let mut city = c;
        city.citizens = generate_base_population(size, &dict);
        city = link_partners(city);
        city = link_parents(city);
        city = link_colleagues(city);
        city = link_friends_within_population(city);
        city = link_siblings(city);
        city = link_grandparents(city);
        return city;
    }

    pub fn generate_population_baseline<'a>(
        dict: &Vec<Word>,
        size: usize,
        c: &'a mut City,
    ) -> &'a mut City {
        let mut city = c;
        city.citizens = generate_base_population(size, &dict);
        // city = link_colleagues(city);
        // city = link_friends_within_population(city);
        return city;
    }

    // pub fn generate_population(name_dict: &NameDictionary, size: usize) -> Population {
    //     let mut population = generate_base_population(size, &name_dict);
    //     population = add_partners_to_population(population, &name_dict);
    //     population = add_parents_to_population(population, &name_dict);
    //     population = link_friends_within_population(population);
    //     return population;
    // }

    pub fn output_population(city: &City) {
        let mut file = File::create("./export.txt").unwrap();
        let pop_log = print_population(&city);
        file.write_all(pop_log.into_bytes().as_slice()).unwrap();
    }
}
