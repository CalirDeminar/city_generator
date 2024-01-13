pub mod names {
    use regex::Regex;

    use crate::{
        city::population::mind::mind::{Gender, Mind},
        language2::language2::Dictionary,
    };

    // const MALE_LAST_KEY: &str = "ML";
    // const MALE_FIRST_KEY: &str = "MF";
    // const FEMALE_LAST_KEY: &str = "FL";
    // const FEMALE_FIRST_KEY: &str = "FF";
    // const ANY_LAST_KEY: &str = "*L";
    // const ANY_FIRST_KEY: &str = "*F";

    const JOINERS: [char; 4] = ['-', '_', ' ', ':'];

    const NAME_PATTERN_REGEX_UPPER: &str =
        r"[^(ML|MF|FL|FF|\*F\*L|{|})]*\{\{(ML|MF|FL|FF|\*F\*L)\}\}[^(ML|MF|FL|FF|\*F\*L|{|})]*";
    const NAME_PATTERN_REGEX_LOWER: &str = r"([^(ML|MF|FL|FF|\*F\*L|{|})]*)(\{\{[ML|MF|FL|FF|\*F\*L]*\}\})([^(ML|MF|FL|FF|\*F\*L|{|})]*)";

    pub fn name(dict: &Dictionary, gender: &Gender, era: Option<String>) -> (String, String) {
        let mut first_name_tags = vec![String::from("First Name")];
        let mut last_name_tags = vec![String::from("Last Name")];
        if gender.eq(&Gender::Male) {
            first_name_tags.push(String::from("Male Gender"));
        } else if gender.eq(&Gender::Female) {
            first_name_tags.push(String::from("Female Gender"));
        }

        if era.is_some() {
            let e = era.unwrap();
            first_name_tags.push(e.clone());
            last_name_tags.push(e.clone())
        }
        let first_name = dict.pick_noun_with_groups(first_name_tags);

        let last_name = dict.pick_noun_with_groups(last_name_tags);
        return (first_name.base.clone(), last_name.base.clone());
    }

    pub fn parse_name_format(format: &str, mind_1: &Mind, mind_2: &Mind) -> String {
        let mut output = String::new();
        let mut last_mind: Option<((&str, &str), &Gender)> = None;

        let block_regex = Regex::new(NAME_PATTERN_REGEX_UPPER).unwrap();
        let subblock_regex = Regex::new(NAME_PATTERN_REGEX_LOWER).unwrap();

        for m in block_regex.find_iter(format) {
            let captures = subblock_regex.captures(m.as_str()).unwrap();

            for (i, cap) in captures.iter().enumerate() {
                if i > 0 && cap.is_some() {
                    let capture = cap.unwrap().as_str();

                    if capture.starts_with("{{") && capture.ends_with("}}") {
                        let token = capture.replace("{{", "").replace("}}", "");
                        let parent = fetch_keyed_parent(
                            ((&mind_1.first_name, &mind_1.last_name), &mind_1.gender),
                            ((&mind_2.first_name, &mind_2.last_name), &mind_2.gender),
                            &token,
                            last_mind,
                        );
                        output.push_str(fetch_keyed_name(parent, &token));
                        last_mind = Some(parent.clone());
                    } else {
                        output.push_str(capture);
                    }
                    // println!("{:?}", capture);
                }
            }
        }
        return output;
    }

    fn fetch_keyed_parent<'a>(
        mind_1: ((&'a str, &'a str), &'a Gender),
        mind_2: ((&'a str, &'a str), &'a Gender),
        key: &str,
        exclude: Option<((&'a str, &'a str), &'a Gender)>,
    ) -> ((&'a str, &'a str), &'a Gender) {
        let gender_token = key.chars().next().unwrap();
        if exclude.is_some() {
            let e = exclude.unwrap();
            if mind_1.eq(&e) {
                return mind_2;
            } else if mind_2.eq(&e) {
                return mind_1;
            }
        }
        match (gender_token, mind_1.1, mind_2.1) {
            ('M', _, Gender::Male) => mind_2,
            ('M', _, _) => mind_1,
            ('F', _, Gender::Female) => mind_2,
            ('F', _, _) => mind_1,
            _ => mind_1,
        }
    }

    fn fetch_keyed_name<'a>(mind_1: ((&'a str, &'a str), &Gender), key: &str) -> &'a str {
        let name_token = key.chars().last().unwrap();
        if name_token.eq(&'F') {
            return clean_name(mind_1.0 .0);
        } else {
            return clean_name(mind_1.0 .1);
        }
    }

    fn clean_name(name: &str) -> &str {
        let mut split = name.split(|s| {
            return JOINERS.contains(&s);
        });
        return split.next().unwrap();
    }

    #[cfg(test)]
    pub mod tests {

        use crate::{
            city::population::mind::mind::{random_char2, Gender},
            language2::{language2::build_dictionary, names::names::name},
        };

        use super::parse_name_format;

        #[test]
        fn gen_names() {
            let dict = build_dictionary();
            for _i in 0..25 {
                let _i = name(&dict, &Gender::Ambiguous, Some(String::from("Modern")));
            }
            for _i in 0..25 {
                let _i = name(&dict, &Gender::Ambiguous, Some(String::from("Medieval")));
            }
        }

        #[test]
        fn last_name_formats() {
            let dict = build_dictionary();
            let mut mind_1 = random_char2(&dict, &None);
            let mut mind_2 = random_char2(&dict, &None);
            mind_1.gender = Gender::Male;
            mind_2.gender = Gender::Female;
            mind_1.last_name = String::from("Henry-Packel");
            mind_2.last_name = String::from("Harvey-Steel");
            assert!(parse_name_format("{{ML}}-{{FL}}", &mind_1, &mind_2).eq(&"Henry-Harvey"));

            mind_2.gender = Gender::Male;
            mind_1.gender = Gender::Female;
            assert!(parse_name_format("{{ML}}-{{FL}}", &mind_1, &mind_2).eq(&"Harvey-Henry"));
        }
    }
}
