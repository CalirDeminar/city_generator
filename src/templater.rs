pub mod templater {
    use crate::{
        culture::culture::*,
        language::{language::*, nouns::nouns::NounTag},
        names::names::*,
    };
    use regex::*;
    use strum::IntoEnumIterator; // 0.17.1

    fn string_match_name_tag(token: &str) -> Option<NameTag> {
        for tag in NameTag::iter() {
            let matcher = format!("{}", tag);
            if matcher.eq(token.trim()) {
                return Some(tag);
            }
        }
        return None;
    }

    pub fn render_template(template: &str, name_list: &Vec<NameDefinition>) -> String {
        let mut output = String::new();
        let word_regex = Regex::new(r"(\{\{[a-zA-Z]*\}\})").unwrap();
        for p in word_regex.find_iter(template) {
            let part = p.as_str().replace("{", "").replace("}", "");
            let t = string_match_name_tag(&part);
            if t.is_some() {
                let tag = t.unwrap();
                let mut name = random_name_definition_filter_tag(&name_list, &tag);
                if !name.tags.contains(&NameTag::Suffixable) {
                    output.push_str(" ");
                } else {
                    name.name = name.name.to_lowercase();
                }
                output.push_str(&name.name);
            } else {
                println!("Could Not Found NameTag: {}", &part);
            }
        }
        return String::from(output.trim());
    }

    // Tags: "{{Nuon(Tag)}}"
    pub fn render_template_2(template: &str, dictionary: &Vec<Word>, era: &Option<Era>) -> String {
        let mut output = String::new();
        let word_regex = Regex::new(r"([a-zA-Z0-9 \-\:\']*\{\{[a-zA-Z\(\) \,\!]*\}\})").unwrap();

        for term in word_regex.find_iter(template) {
            let split: Vec<&str> = term.as_str().split("{").collect();
            let prefix = split.first().unwrap();
            let suffix = split.last().unwrap();

            let part = suffix.replace("{", "").replace("}", "").replace(")", "");
            let mut p = part.split("(");
            let type_tag = p.next().unwrap();
            let possible_word_type = WordType::iter().find(|t| t.to_string().eq(type_tag));
            if possible_word_type.is_none() {
                println!("Could not find tag: {}", type_tag);
            }
            let word_type = possible_word_type.unwrap();
            let (optional, required): (Vec<String>, Vec<String>) =
                p.next().unwrap().split(",").fold(
                    (vec![], vec![]),
                    |(optional_acc, required_acc): (Vec<String>, Vec<String>), s| {
                        let replaced = String::from(s.replace("!", "").trim());
                        if replaced.len() != s.trim().len() {
                            return (optional_acc, vec![required_acc, vec![replaced]].concat());
                        } else {
                            return (
                                vec![optional_acc, vec![String::from(s)]].concat(),
                                required_acc,
                            );
                        }
                    },
                );
            let w = random_word_by_tag(&dictionary, word_type, &required, &optional, &vec![], &era);
            output.push_str(prefix.clone());
            if w.is_some() {
                let word = w.unwrap();
                if word.tags.contains(&NounTag::Suffixable.to_string()) {
                    output = output.trim_end().to_string();
                    output.push_str(&word.text.trim().to_lowercase());
                } else {
                    output.push_str(&word.text.trim());
                }
            }
        }
        return output.trim().to_string();
    }

    #[test]
    fn test_render_template() {
        let example_template: &str =
            "{{Noun(!LastName, !HistoricalFigure)}} {{Noun(GeographyFeatureSizeLocalFeature)}}";
        let d = build_dictionary();
        let dict = build_culture_dictionary(&d, &random_culture(&d, &None));
        println!("{}", render_template_2(example_template, &dict, &None));
    }
}
