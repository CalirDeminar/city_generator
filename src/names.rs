// pub mod data_parser;
pub mod names {
    use std::fs::File;
    use rand::Rng;

    use crate::city::population::mind::mind::*;

    #[derive(PartialEq, Debug, Clone)]
    pub struct NameDefinition {
        pub name: String,
        pub gender: Gender
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct NameDictionary {
        pub first_names: Vec<NameDefinition>,
        pub last_names: Vec<NameDefinition>,
        pub food_service_suffixes: Vec<NameDefinition>,
        pub location_prefixes: Vec<NameDefinition>,
        pub specialist_retail_suffixes: Vec<NameDefinition>,

    }

    pub fn gen_name_dict() -> NameDictionary {
        return NameDictionary {
            first_names: parse_file(String::from("./static_data/english_first_names.csv")),
            last_names: parse_file(String::from("./static_data/english_last_names.csv")),
            food_service_suffixes: parse_file(String::from("./static_data/food_service_suffixes.csv")),
            location_prefixes: parse_file(String::from("./static_data/location_prefixes.csv")),
            specialist_retail_suffixes: parse_file(String::from("./static_data/specialist_retail_suffixes.csv"))
        }
    }

    fn random_name_for_gender<'a>(input: &'a Vec<NameDefinition>, gender: &Gender) -> String {
        let mut working: Vec<&'a NameDefinition> = vec![];
        for name in input {
            if name.gender.eq(&Gender::Ambiguous) {
                working.push(name);
            }
            if name.gender.eq(gender) {
                working.push(name);
            }
        }
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let result = working[(roll * working.len() as f32) as usize];
        return String::from(&result.name);
    }

    pub fn random_name(list: &Vec<NameDefinition>) -> String {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let result = list[(roll*list.len() as f32) as usize].clone();
        return result.name;
    }

    pub fn random_mind_name<'a>(dict: &'a NameDictionary, gender: &Gender) -> (String, String) {
        return (random_name_for_gender(&dict.first_names, &gender), random_name_for_gender(&dict.last_names, &gender));
    }

    fn parse_file(filename: String) -> Vec<NameDefinition> {
        let mut output: Vec<NameDefinition> = vec![];
        let file = File::open(&filename).expect(&format!("Cannot open: {}", &filename));
        let mut csv_reader = csv::ReaderBuilder::new().from_reader(file);
        for l in csv_reader.records() {
            let line = l.unwrap();
            let mut gender = Gender::Ambiguous;
            let gender_str = line.get(1).unwrap().trim_start().to_lowercase();
            if gender_str.eq("m") {
                gender = Gender::Male;
            }
            if gender_str.eq("f") {
                gender = Gender::Female;
            }
            output.push(NameDefinition{
                name: String::from(line.get(0).unwrap().trim_start()),
                gender
            });
        }
        return output;
    }



    // #[test]
    // fn random_name_test() {
    //     let dict = gen_name_dict();
    //     for _i in 0..10 {
    //         println!("{:?}", random_name(&dict, &Gender::Male));
    //     }
    // }
}