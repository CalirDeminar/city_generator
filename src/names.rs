// pub mod data_parser;
pub mod names {
    use std::{fs::File};
    use std::io::{self, BufRead};
    use rand::Rng;
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::{EnumIter, Display}; // 0.17.1

    use crate::city::population::mind::mind::*;

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum NameTag {
        MaleGender,
        FemaleGender,
        AmbigiousGender,
        FirstName,
        LastName,
        StyleUK,
        InstitutionFoodServiceSuffix,
        InstitutionRetailSpecificSuffix,
        InstititutionRetailGeneralSuffix,
        Suffixable,
        Prefixable,
        LocationMajorFeature,
        LocationMinorFeature,
        LocationDesciptor
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct NameDefinition {
        pub name: String,
        pub tags: Vec<NameTag>
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct NameDictionary {
        pub first_names: Vec<NameDefinition>,
        pub last_names: Vec<NameDefinition>,
        pub food_service_suffixes: Vec<NameDefinition>,
        pub location_descriptors: Vec<NameDefinition>,
        pub specialist_retail_suffixes: Vec<NameDefinition>,
        pub descriptors: Vec<NameDefinition>,
        pub major_features: Vec<NameDefinition>,
        pub minor_features: Vec<NameDefinition>,

    }

    pub fn gen_name_dict() -> NameDictionary {
        return NameDictionary {
            first_names: parse_file(String::from("./static_data/english_first_names.csv")),
            last_names: parse_file(String::from("./static_data/english_last_names.csv")),
            food_service_suffixes: parse_file(String::from("./static_data/institutions_food_service_suffixes.csv")),
            location_descriptors: parse_file(String::from("./static_data/location_descriptors.csv")),
            specialist_retail_suffixes: parse_file(String::from("./static_data/institutions_specialist_retail_suffixes.csv")),
            descriptors: parse_file(String::from("./static_data/location_descriptors.csv")), 
            major_features: parse_file(String::from("./static_data/location_major_features.csv")), 
            minor_features: parse_file(String::from("./static_data/location_minor_features.csv")),
        }
    }

    pub fn filter_on_tag(input: &Vec<NameDefinition>, tag: &NameTag) -> Vec<NameDefinition> {
        let mut output: Vec<NameDefinition> = Vec::new();
        for name in input {
            if name.tags.contains(tag) {
                output.push(name.clone());
            }
        }
        return output;
    }

    pub fn exclude_on_tag(input: &Vec<NameDefinition>, tag: &NameTag) -> Vec<NameDefinition> {
        let mut output: Vec<NameDefinition> = Vec::new();
        for name in input {
            if !name.tags.contains(tag) {
                output.push(name.clone());
            }
        }
        return output;
    }

    fn random_name_for_gender(input: &Vec<NameDefinition>, gender: &Gender) -> String {
        let working: Vec<NameDefinition> = match gender {
            &Gender::Male => exclude_on_tag(&input, &NameTag::FemaleGender),
            &Gender::Female => exclude_on_tag(&input, &NameTag::MaleGender),
            _ => input.clone()
        };
       return random_name(&working);
    }

    pub fn random_name(list: &Vec<NameDefinition>) -> String {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let result = list[(roll*list.len() as f32) as usize].clone();
        return result.name;
    }

    pub fn random_name_definition(list: &Vec<NameDefinition>) -> NameDefinition {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let result = list[(roll*list.len() as f32) as usize].clone();
        return result;
    }

    pub fn random_mind_name<'a>(dict: &'a NameDictionary, gender: &Gender) -> (String, String) {
        return (random_name_for_gender(&dict.first_names, &gender), random_name(&dict.last_names));
    }

    fn string_match_name_tag(token: &str) -> Option<NameTag> {
        for tag in NameTag::iter() {
            let matcher = format!("{}", tag);
            if matcher.eq(token.trim()) {
                return Some(tag);
            }
        }
        return None;
    }

    fn parse_file(filename: String) -> Vec<NameDefinition> {
        let mut output: Vec<NameDefinition> = vec![];
        let file = File::open(&filename).expect(&format!("Cannot open: {}", &filename));
        let lines = io::BufReader::new(file).lines();
        for l in lines {
            if l.is_ok() {
                let line = l.unwrap();
                let splits = line.split(",");

                let mut i = 0;
                let mut name: String = String::new();
                let mut tags: Vec<NameTag> = vec![];
                for entry in splits {
                    if i == 0 {
                        name = String::from(entry);
                    } else {
                        let tag = string_match_name_tag(&entry);
                        if tag.is_some() {
                            tags.push(tag.unwrap());
                        }
                    }
                    i += 1;
                }
                if name.len() > 0 {
                    output.push(NameDefinition { name: name.clone(), tags: tags.clone() });
                }
            }
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