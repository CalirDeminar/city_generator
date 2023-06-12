pub mod nouns {
    use crate::{
        language::language::{filter_words_by_tag_and, Era, Word, WordTag, WordType},
        parser::parser::parse_file,
    };
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::{Display, EnumIter};
    use uuid::Uuid; // 0.17.1

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
        Emotion,
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
        Material,
        Cloth,
        Metal,
        Plant,
        Food,
        Tree,
        Flower,
        Creature,
        CreatureCategory,
        GreatCreature,
        Era(Era),
        GlobalSingular,
    }

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
        let data = parse_file(String::from("language_nouns.csv"));
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
        return output;
    }

    #[test]
    fn test_noun_parser() {
        println!(
            "{:#?}",
            filter_words_by_tag_and(&build_nouns(), vec![WordTag::Noun(NounTag::Food)])
        );
    }
}
