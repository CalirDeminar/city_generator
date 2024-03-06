pub mod templater2 {
    use regex::Regex;

    use crate::language2::language2::{Dictionary, LogicalOperator, WordType};

    // const AND_TOKEN: &str = "AND";

    // const OR_TOKEN: &str = "OR";

    const REGEXP: &str = r"([^NOUN_AND|^NOUN_OR|^ADJECTIVE_AND|^ADJECTIVE_OR]*)([NOUN_AND|NOUN_OR|ADJECTIVE_AND|ADJECTIVE_OR]+\([a-zA-Z ,-:]+\))([^(NOUN_AND|NOUN_OR|ADJECTIVE_AND|ADJECTIVE_OR)]*)";

    fn convert_token_to_word<'a>(token: &str, dict: &'a Dictionary) -> &'a str {
        let arguments: Vec<String> = token
            .replace("NOUN_AND", "")
            .replace("NOUN_OR", "")
            .replace("ADJECTIVE_AND", "")
            .replace("ADJECTIVE_OR", "")
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|s| String::from(s.trim()))
            .collect();
        if token.contains("NOUN_AND(") {
            return dict.get_word_base_with_groups(WordType::Noun, arguments, LogicalOperator::AND);
        } else if token.contains("NOUN_OR(") {
            return dict.get_word_base_with_groups(WordType::Noun, arguments, LogicalOperator::OR);
        } else if token.contains("ADJECTIVE_AND(") {
            return dict.get_word_base_with_groups(WordType::Adjective, arguments, LogicalOperator::AND);
        } else if token.contains("ADJECTIVE_OR(") {
            return dict.get_word_base_with_groups(WordType::Adjective, arguments, LogicalOperator::OR);
        } else {
            return "";
        }
    }

    pub fn render_template(template: &str, dict: &Dictionary) -> String {
        let mut output = String::new();
        let match_regex = Regex::new(REGEXP).unwrap();
        for p in match_regex.find_iter(template) {
            for (i, cap) in match_regex.captures(p.as_str()).unwrap().iter().enumerate() {
                if cap.is_some() && i > 0 {
                    // println!("Capture: {:?}", cap.unwrap().as_str());
                    let capture = cap.unwrap();
                    if i.eq(&2) {
                        output.push_str(convert_token_to_word(capture.as_str(), dict));
                    } else {
                        output.push_str(capture.as_str());
                    }
                }
            }
        }
        return output;
    }

    #[cfg(test)]
    pub mod tests {
        use crate::language2::{
            language2::build_dictionary, templater2::templater2::render_template,
        };

        #[test]
        fn test_templater() {
            let dict = build_dictionary();
            for _i in 0..10 {
                println!(
                    "{:?}",
                    render_template("ADJECTIVE_AND(Quality) NOUN_OR(Metal, Wood) and NOUN_OR(Metal,Wood) Harpoon", &dict)
                );
            }
        }
    }
}
