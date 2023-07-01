pub mod parental_naming_formats {
    use rand::seq::SliceRandom;
    use rand::Rng;
    use regex::Regex;

    use crate::{
        city::population::mind::mind::{Gender, Mind},
        culture::culture::CultureConfig,
    };

    const MALE_LAST_KEY: &str = "ML";
    const MALE_FIRST_KEY: &str = "MF";
    const FEMALE_LAST_KEY: &str = "FL";
    const FEMALE_FIRST_KEY: &str = "FF";
    const ANY_LAST_KEY: &str = "*L";
    const ANY_FIRST_KEY: &str = "*F";

    fn parse_format(
        format: &str,
        partner_1: (&str, &str, &Gender),
        partner_2: (&str, &str, &Gender),
    ) -> String {
        let mut output = String::new();
        let block_regex =
            Regex::new(r"([a-zA-Z0-9 \-\:\']*\{\{(ML|MF|FL|FF|\*F\*L)\}\}[a-zA-Z0-9 \-\:\']*)")
                .unwrap();

        let (m, f) = if partner_1.2.eq(&Gender::Male) {
            (partner_1, partner_2)
        } else if partner_2.2.eq(&Gender::Male) {
            (partner_2, partner_1)
        } else if partner_1.2.eq(&Gender::Female) {
            (partner_2, partner_1)
        } else {
            (partner_1, partner_2)
        };
        let mut male_used = false;

        for term in block_regex.find_iter(format) {
            let split: Vec<&str> = term.as_str().split("{").collect();
            let prefix = if split.len() > 1 {
                split.first().unwrap()
            } else {
                ""
            };
            let body = split.last().unwrap();
            let sub_split: Vec<&str> = body.split("}").collect();
            let suffix = if sub_split.len() > 1 {
                sub_split.last().unwrap()
            } else {
                ""
            };

            let mut key = sub_split.first().unwrap().replace("{", "");
            key = key.replace("}", "");

            let m_last_name_split: Vec<&str> = m.1.split("-").collect();
            let f_last_name_split: Vec<&str> = f.1.split("-").collect();

            output.push_str(prefix.clone());
            match key.as_str() {
                MALE_LAST_KEY => {
                    output.push_str(m_last_name_split.first().unwrap());
                    male_used = true;
                }
                MALE_FIRST_KEY => {
                    output.push_str(m.0);
                    male_used = true;
                }
                FEMALE_LAST_KEY => {
                    output.push_str(f_last_name_split.first().unwrap());
                }
                FEMALE_FIRST_KEY => {
                    output.push_str(f.0);
                }
                ANY_LAST_KEY => {
                    output.push_str(if male_used {
                        f_last_name_split.first().unwrap()
                    } else {
                        m_last_name_split.first().unwrap()
                    });
                    if !male_used {
                        male_used = true;
                    }
                }
                ANY_FIRST_KEY => {
                    output.push_str(if male_used { f.0 } else { m.0 });
                    if !male_used {
                        male_used = true;
                    }
                }
                _ => {}
            }
            output.push_str(suffix);
        }
        return output;
    }

    pub fn get_new_couple_last_names(
        partner_m: &Mind,
        partner_f: &Mind,
        culture: &CultureConfig,
    ) -> (String, String) {
        let mut formats = culture.parental_naming_formats.clone();
        formats.shuffle(&mut rand::thread_rng());
        let (f1, f2, _cm, _cf) = formats.first().unwrap();
        return (
            parse_format(
                f1,
                (
                    &partner_m.first_name,
                    &partner_m.last_name,
                    &partner_m.gender,
                ),
                (
                    &partner_f.first_name,
                    &partner_f.last_name,
                    &partner_f.gender,
                ),
            ),
            parse_format(
                f2,
                (
                    &partner_m.first_name,
                    &partner_m.last_name,
                    &partner_m.gender,
                ),
                (
                    &partner_f.first_name,
                    &partner_f.last_name,
                    &partner_f.gender,
                ),
            ),
        );
    }

    pub fn get_child_last_name(
        gender: &Gender,
        partner_m: &Mind,
        partner_f: &Mind,
        culture: &CultureConfig,
    ) -> String {
        let mut rng = rand::thread_rng();
        let mut formats = culture.parental_naming_formats.clone();
        formats.shuffle(&mut rand::thread_rng());
        let (_f1, _f2, cm, cf) = formats.first().unwrap();
        let mut target_format = if rng.gen::<f32>() > 0.5 { cm } else { cf };
        if gender.eq(&Gender::Male) {
            target_format = cm;
        }
        if gender.eq(&Gender::Female) {
            target_format = cf;
        }

        let m_last_split: Vec<&str> = partner_m.last_name.split("-").collect();
        let f_last_split: Vec<&str> = partner_f.last_name.split("-").collect();

        return parse_format(
            target_format,
            (
                &partner_m.first_name,
                &partner_m.last_name,
                &partner_m.gender,
            ),
            (
                &partner_f.first_name,
                &partner_f.last_name,
                &partner_f.gender,
            ),
        );
    }

    #[test]
    fn test_parse_format() {
        println!(
            "Name: {}",
            parse_format(
                "{{ML}}-{{FL}}",
                ("James", "Freeman-Worthord", &Gender::Male),
                ("Annie", "Antwood-Ford", &Gender::Female)
            )
        );
    }
}
