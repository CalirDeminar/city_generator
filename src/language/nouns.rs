pub mod creatures;
pub mod emotions;
pub mod geography;
pub mod materials;
pub mod plants;
pub mod nouns {
    use std::fs;

    use crate::{language::language::*, parser::parser::parse_file};
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::{Display, EnumIter};
    use uuid::Uuid;

    use super::{
        creatures::creatures::{CreatureFamily, CreatureSize},
        emotions::emotions::EmotionGroups,
        materials::materials::*,
        plants::plants::PlantType,
    }; // 0.17.1

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
        Emotion(EmotionGroups),
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
        Material(MaterialState, SolidMaterialForm),
        MaterialTag(MaterialTag),
        Plant(PlantType),
        Food,
        Creature(CreatureSize, CreatureFamily),
        Era(Era),
        GlobalSingular,
    }

    // -- TODO - Split Noun Groups Into -
    // Material Groups - Solid / Liquid / Gas - Metal, Cloth, Normal, etc
    // Geographical Feature Sizes

    fn string_match_noun_tag(token: &str) -> Option<NounTag> {
        for tag in NounTag::iter() {
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
                let mut tags: Vec<WordTag> = Vec::new();
                for incoming_tag in incoming_tags {
                    let tag = string_match_noun_tag(&incoming_tag);
                    if tag.is_some() {
                        tags.push(WordTag::Noun(tag.unwrap()));
                    }
                }

                output.push(Word {
                    id: Uuid::new_v4(),
                    wordType: WordType::Noun,
                    text: subject,
                    tags,
                    relatedForms: Vec::new(),
                });
            }
        }
        return output;
    }

    #[test]
    fn test_noun_parser() {
        println!(
            "{:#?}",
            filter_words_by_tag_and(
                &build_nouns(),
                vec![WordTag::Noun(NounTag::Material(
                    MaterialState::Solid,
                    SolidMaterialForm::Solid
                ))]
            )
        );
    }
}
