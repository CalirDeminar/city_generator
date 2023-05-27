// pub mod data_parser;
pub mod names {
    use std::{fs::File};
    use std::io::{self, BufRead};
    use rand::Rng;
    use regex::Regex;
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::{EnumIter, Display}; // 0.17.1

    use crate::city::population::mind::mind::*;

    // name files have a line comment start of //

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
        pub location_descriptors: Vec<NameDefinition>,
        pub institution_suffixes: Vec<NameDefinition>,
        pub major_features: Vec<NameDefinition>,
        pub minor_features: Vec<NameDefinition>,
        pub total_list: Vec<NameDefinition>
    }

    pub fn gen_name_dict() -> NameDictionary {
        let first_names = parse_file(String::from("./static_data/english_first_names.csv"));
        let last_names = parse_file(String::from("./static_data/english_last_names.csv"));
        let institution_suffixes = parse_file(String::from("./static_data/institution_suffixes.csv"));
        let location_descriptors = parse_file(String::from("./static_data/location_descriptors.csv"));
        let major_features =  parse_file(String::from("./static_data/location_major_features.csv"));
        let minor_features = parse_file(String::from("./static_data/location_minor_features.csv"));
        return NameDictionary {
            total_list: vec![first_names.clone(), last_names.clone(), institution_suffixes.clone(), location_descriptors.clone(), major_features.clone(), minor_features.clone()].concat(),
            first_names,
            last_names,
            institution_suffixes,
            location_descriptors, 
            major_features, 
            minor_features,
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
        return match gender {
            &Gender::Male => random_name_definition_exclude_tag(&input, &NameTag::FemaleGender).name,
            &Gender::Female => random_name_definition_exclude_tag(&input, &NameTag::MaleGender).name,
            _ => random_name_definition(&input).name
        };
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

    pub fn random_name_definition_filter_tag(list: &Vec<NameDefinition>, tag: &NameTag) -> NameDefinition {
        let filtered_list = filter_on_tag(&list, &tag);
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let result = filtered_list[(roll*filtered_list.len() as f32) as usize].clone();
        return result;
    }

    pub fn random_name_definition_exclude_tag(list: &Vec<NameDefinition>, tag: &NameTag) -> NameDefinition {
        let filtered_list = exclude_on_tag(&list, &tag);
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let result = filtered_list[(roll*filtered_list.len() as f32) as usize].clone();
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
                let line_value = l.unwrap();
                let line = Regex::replace_all(&Regex::new(r"\/\/[a-zA-Z ]*$").unwrap(), &line_value, "");
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