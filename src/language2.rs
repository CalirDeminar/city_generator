pub mod language2 {
    use std::collections::{HashMap, HashSet};

    use regex::Regex;
    use uuid::Uuid;

    const NOUN_FLAG: &str = "NOUN";
    const ADJECTIVE_FLAG: &str = "ADJECTIVE";

    static GROUP_PATTERN: &str = r"GROUP\(([a-zA-Z]*\))";
    static HAS_ADJECTIVE_PATTERN: &str = r"HAS_ADJECTIVE\(([a-zA-Z]*\))";
    static HAS_NOUN_PATTERN: &str = r"HAS_NOUN\(([a-zA-Z]*\))";

    #[derive(PartialEq, Debug, Clone, Default)]
    pub struct Noun {
        pub id: Uuid,
        pub base: String,
        pub recipie: Option<String>,
        pub groups: Vec<String>,
        pub adjective: Option<String>
    }

    #[derive(PartialEq, Debug, Clone, Default)]
    pub struct Adjective {
        pub id: Uuid,
        pub base: String,
        pub groups: Vec<String>,
        pub noun: Option<String>
    }

    #[derive(PartialEq, Debug, Clone, Default)]
    pub struct Dictionary {
        pub nouns: HashMap<Uuid, Noun>,
        pub adjectives: HashMap<Uuid, Adjective>,
        pub noun_groups: HashMap<String, HashSet<Uuid>>,
        pub adjective_groups: HashMap<String, HashSet<Uuid>>
    }

    impl Dictionary {
        pub fn nouns_with_groups(&self, groups: Vec<String>) -> Vec<&Noun> {
            let mut initial_ids = self.noun_groups.get(groups.first().unwrap()).unwrap().clone();
            for i in 1..groups.len() {
                let common_ids = self.noun_groups.get(groups.get(i).unwrap()).unwrap();
                initial_ids.retain(|id| common_ids.contains(&id));
            }
            return initial_ids.iter().map(|id| self.nouns.get(id).unwrap()).collect();
        }

        pub fn adjectives_with_groups(&self, groups: Vec<String>) -> Vec<&Adjective> {
            let mut initial_ids = self.adjective_groups.get(groups.first().unwrap()).unwrap().clone();
            for i in 1..groups.len() {
                let common_ids = self.adjective_groups.get(groups.get(i).unwrap()).unwrap();
                initial_ids.retain(|id| common_ids.contains(&id));
            }
            return initial_ids.iter().map(|id| self.adjectives.get(id).unwrap()).collect();
        }

        fn parse_datafile_line(&mut self, line: String) {
            let mut elements = line.split(", ");
            let base = elements.next().unwrap();
            let data: Vec<&str> = elements.map(|element| element).collect();

            let id = Uuid::new_v4();
            let group_regex = Regex::new(GROUP_PATTERN).unwrap();
            let has_adjective_regex: Result<Regex, regex::Error> = Regex::new(HAS_ADJECTIVE_PATTERN);
            let has_noun_regex: Result<Regex, regex::Error> = Regex::new(HAS_NOUN_PATTERN);

            if data.contains(&NOUN_FLAG) {
                let mut noun = Noun {
                    id,
                    base: String::from(base),
                    recipie: None,
                    groups: Vec::new(),
                    adjective: None
                };

                for entry in data {
                    let group_attempt = group_regex.captures(entry);
                    if group_attempt.is_some() {
                        let group = group_attempt.unwrap().get(0).unwrap().as_str();
                        noun.groups.push(String::from(group));
                        if !self.noun_groups.contains_key(group) {
                            self.noun_groups.insert(String::from(group), HashSet::new());
                        }
                        self.noun_groups.get_mut(group).unwrap().insert(id);
                    }
                }
                self.nouns.insert(id, noun);
            }
            else if data.contains(&ADJECTIVE_FLAG) {
                let mut adjective = Adjective {
                    id,
                    base: String::from(base),
                    groups: Vec::new(),
                    noun: None
                };
                self.adjectives.insert(id, adjective);
            }
        }
    }

    #[cfg(test)]
    pub mod tests {
        use std::collections::HashMap;

        use super::Dictionary;

        #[test]
        fn test_parsing() {
            let mut dict = Dictionary {
                nouns: HashMap::new(),
                noun_groups: HashMap::new(),
                adjectives: HashMap::new(),
                adjective_groups: HashMap::new()
            };
            println!("Before: {:#?}", dict);
            dict.parse_datafile_line(String::from("SomeNoun, NOUN, GROUP(METAL)"));
            println!("After: {:#?}", dict);
        }
    }
}