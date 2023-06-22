pub mod creatures;
pub mod emotions;
pub mod era;
pub mod geography;
pub mod materials;
pub mod plants;
pub mod nouns {
    use std::fs;

    use crate::{language::language::*, parser::parser::parse_file};
    use regex::Regex;
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::{Display, EnumIter};
    use uuid::Uuid;

    use super::{
        creatures::creatures::creature_tags, emotions::emotions::emotion_group_tags,
        era::eras::era_tags, geography::geography::geography_tags,
        materials::materials::material_tags, plants::plants::plant_tags,
    };

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy)]
    pub enum NounTag {
        // Cultural
        AbstractConcept,
        Profession,
        Relation,
        CulturalGroup,
        Good,
        Evil,
        Holy,
        Institution,
        Affliction,
        Symbolic,
        // Constructed Objects
        Weapon,
        Worn,
        Furniture,
        Tool,
        Product,
        Construction,
        SubConstruction,
        // World High
        WorldFeature,
        GeographicFeature,
        Settlement,
        Event,
        Weather,
        // World Specific
        BodyPart,
        Food,
        GlobalSingular,
        Direction,
        Title,
        BuildingTitle,
        FirstName,
        LastName,
        GenderMale,
        GenderFemale,
        GeneralRetailerName,
        RetailerFood,
        RetailerSpecialist,
        Suffixable,
    }

    fn build_generic_tags() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for tag in NounTag::iter() {
            output.push(tag.to_string());
        }
        return output;
    }

    pub fn build_noun_tags() -> Vec<String> {
        let mut output: Vec<Vec<String>> = Vec::new();

        output.push(build_generic_tags());
        output.push(creature_tags());
        output.push(emotion_group_tags());
        output.push(era_tags());
        output.push(geography_tags());
        output.push(material_tags());
        output.push(plant_tags());
        return output.concat();
    }

    // -- TODO - Split Noun Groups Into -
    // Material Groups - Solid / Liquid / Gas - Metal, Cloth, Normal, etc
    // Geographical Feature Sizes

    fn string_match_noun_tag(token: &str) -> Option<String> {
        for tag in build_noun_tags() {
            let matcher = format!("{}", tag);
            if matcher.eq(token.trim()) {
                return Some(tag);
            }
        }
        return None;
    }

    pub fn build_nouns() -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        let paths = fs::read_dir("./static_data/nouns").unwrap();
        for path in paths {
            let filename = path.unwrap().file_name();
            let data = parse_file(format!("nouns/{}", filename.to_str().unwrap()));
            for (subject, incoming_tags) in data {
                let mut tags: Vec<String> = Vec::new();
                let mut adjective_terms: Vec<String> = Vec::new();
                for incoming_tag in incoming_tags {
                    let tag = string_match_noun_tag(&incoming_tag);
                    if tag.is_some() {
                        tags.push(tag.unwrap());
                    }
                    if incoming_tag.eq("Adjective") {
                        adjective_terms.push(subject.clone());
                    }
                    let adjective_match =
                        Regex::captures(&Regex::new(r"Adjective\((.*)\)").unwrap(), &incoming_tag);
                    if adjective_match.is_some() {
                        adjective_terms.push(String::from(
                            adjective_match.unwrap().get(1).unwrap().as_str(),
                        ));
                    }
                }
                let adjectives: Vec<Word> = adjective_terms
                    .iter()
                    .map(|t| Word {
                        id: Uuid::new_v4(),
                        word_type: WordType::Adjective,
                        text: String::from(t),
                        tags: tags.clone(),
                        related_forms: Vec::new(),
                    })
                    .collect();
                output.push(Word {
                    id: Uuid::new_v4(),
                    word_type: WordType::Noun,
                    text: subject,
                    tags,
                    related_forms: adjectives,
                });
            }
        }
        return output;
    }

    #[test]
    fn test_noun_parser() {
        let nouns = build_nouns();
        for noun in filter_words_by_tag_or(
            nouns.iter().collect(),
            WordType::Noun,
            vec![String::from("Metal")],
        ) {
            println!("{:#?}", noun);
        }
    }
}
