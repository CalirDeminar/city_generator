pub mod templater {
    use crate::names::names::*;
    use regex::Regex;
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
            let part = p.as_str().replace("{","").replace("}", "");
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

    #[test]
    fn test_render_template() {
        let example_template: &str = "{{LocationDesciptor}}{{LastName}}{{InstitutionFoodServiceSuffix}}";
        let name_dict = gen_name_dict();
        println!("{}", render_template(example_template, &name_dict.total_list));
    }
}