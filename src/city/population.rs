pub mod mind;
pub mod population {
    use uuid::Uuid;

    use crate::city::population::mind::mind::*;
    use crate::language::language::Era;
    use crate::{city::city::City, language::language::Word};
    use std::collections::HashMap;
    use std::{fs::File, io::Write};

    pub type Population = HashMap<Uuid, Mind>;

    pub fn print_population(city: &City) -> String {
        let mut output = String::from("");
        for mind in city.citizens.values().filter(|c| c.alive) {
            output.push_str(&print_mind(&mind, &city));
        }
        return output;
    }

    fn generate_base_population<'a>(i: usize, dict: &Vec<Word>, era: &Option<Era>) -> Population {
        let mut output: Population = HashMap::new();
        for _i in 0..i {
            let char = random_char(&dict, era, true);
            output.insert(char.id.clone(), char);
        }
        return output;
    }

    pub fn generate_population_baseline<'a>(
        dict: &Vec<Word>,
        size: usize,
        c: &'a mut City,
    ) -> &'a mut City {
        let mut city = c;
        city.citizens = generate_base_population(size, &dict, &city.culture.era);
        return city;
    }

    pub fn output_population(city: &City) {
        let mut file = File::create("./export.txt").unwrap();
        let pop_log = print_population(&city);
        file.write_all(pop_log.into_bytes().as_slice()).unwrap();
    }
}
