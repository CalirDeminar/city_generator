pub mod adjectives {
    use std::fs;
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter};
    use uuid::Uuid; // 0.17.1

    use crate::{language::language::*, parser::parser::parse_file};

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy)]
    pub enum AdjectiveTag {
        Age,
        Position,
        Quality,
        Colour,
        Size,
        Taste,
        Positive,
        Negative,
    }

    fn string_match_adjective_tag(token: &str) -> Option<String> {
        for tag in AdjectiveTag::iter() {
            let matcher = format!("{}", tag);
            if matcher.eq(token.trim()) {
                return Some(tag.to_string());
            }
        }
        return None;
    }

    pub fn build_adjectives() -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        let paths = fs::read_dir("./static_data/adjectives").unwrap();
        for path in paths {
            let filename = path.unwrap().file_name();
            let data = parse_file(format!("adjectives/{}", filename.to_str().unwrap()));
            for (subject, incoming_tags) in data {
                let mut tags: Vec<String> = Vec::new();
                for incoming_tag in incoming_tags {
                    let tag = string_match_adjective_tag(&incoming_tag);
                    if tag.is_some() {
                        tags.push(tag.unwrap());
                    }
                }
                output.push(Word {
                    id: Uuid::new_v4(),
                    word_type: WordType::Adjective,
                    text: subject,
                    tags,
                    related_forms: Vec::new(),
                });
            }
        }
        return output;
    }

    #[test]
    fn test_adjective_parser() {
        let adjectives = build_adjectives();
        for adjective in filter_words_by_tag_or(
            adjectives.iter().collect(),
            WordType::Adjective,
            vec![String::from("Colour")],
        ) {
            println!("{:#?}", adjective);
        }
    }
}
