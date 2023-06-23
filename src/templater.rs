pub mod templater {
    use crate::{
        language::language::{
            build_dictionary, filter_words_by_tag_and, random_word_by_tag, Word, WordType,
        },
        names::names::*,
    };
    use regex::{Captures, Regex};
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
    pub fn render_template_2(template: &str, dictionary: &Vec<Word>) -> String {
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
                        let replaced = s.replace("!", "");
                        if replaced.len() != s.len() {
                            println!("Required: {} -> {}", s, replaced);
                            return (optional_acc, vec![required_acc, vec![replaced]].concat());
                        } else {
                            return (
                                vec![optional_acc, vec![String::from(s)]].concat(),
                                required_acc,
                            );
                        }
                    },
                );
            let word = random_word_by_tag(&dictionary, word_type, &required, &optional, &vec![]);
            output.push_str(prefix.clone());
            if word.is_some() {
                output.push_str(&word.unwrap().text);
            }
        }
        return output;
    }

    #[test]
    fn test_render_template() {
        let example_template: &str = "{{Noun(Title)}}'s {{Noun(MaterialTagMetal)}}";
        let name_dict = gen_name_dict();
        let dict = build_dictionary();
        println!("{}", render_template_2(example_template, &dict));
    }
}
