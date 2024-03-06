pub mod names;
pub mod templater2;
pub mod language2 {
    use std::{
        collections::{HashMap, HashSet},
        fs::{self, File},
        io::{self, BufRead},
    };

    use rand::seq::SliceRandom;
    use regex::Regex;
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;
    use uuid::Uuid;

    const NOUN_FLAG: &str = "NOUN";
    const ADJECTIVE_FLAG: &str = "ADJECTIVE";
    const GROUP_FLAG: &str = "GROUP";
    const TEMPLATE_FLAG: &str = "TEMPLATE";

    static GROUP_PATTERN: &str = r"HAS_GROUP\(([a-zA-Z ]*)\)";
    static HAS_ADJECTIVE_PATTERN: &str = r"HAS_ADJECTIVE\(([a-zA-Z ]*)\)";
    static HAS_NOUN_PATTERN: &str = r"HAS_NOUN\(([a-zA-Z ]*)\)";

    #[derive(PartialEq, Debug, Clone, Default)]
    pub struct Noun {
        pub id: Uuid,
        pub base: String,
        pub recipie: Option<String>,
        pub groups: HashSet<String>,
        pub adjective: Option<String>,
    }

    #[derive(PartialEq, Debug, Clone, Default)]
    pub struct Adjective {
        pub id: Uuid,
        pub base: String,
        pub groups: HashSet<String>,
        pub noun: Option<String>,
    }

    #[derive(PartialEq, Debug, Clone, Default)]
    pub struct Template {
        pub id: Uuid,
        pub template: String,
        pub groups: HashSet<String>,
    }

    #[derive(PartialEq, Debug, Clone, Default)]
    pub struct Dictionary {
        pub nouns: HashMap<Uuid, Noun>,
        pub adjectives: HashMap<Uuid, Adjective>,
        pub templates: HashMap<Uuid, Template>,
        pub noun_groups: HashMap<String, HashSet<Uuid>>,
        pub adjective_groups: HashMap<String, HashSet<Uuid>>,
        pub template_groups: HashMap<String, HashSet<Uuid>>,
        pub group_groups: HashMap<String, HashSet<String>>,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub enum WordType {
        Noun,
        Adjective
    }
    #[derive(PartialEq, Debug, Clone)]
    pub enum LogicalOperator {
        AND,
        OR
    }

    #[derive(PartialEq, Debug, Clone, Default, EnumIter)]
    pub enum Era {
        #[default]
        Modern,
        Future,
        Fantasy,
        Medieval,
    }

    impl Dictionary {
        pub fn get_word_base_with_groups(&self, word_type: WordType, groups: Vec<String>, logical_operator: LogicalOperator) -> &str {
            if word_type.eq(&WordType::Noun){
                let mut list = self.nouns_with_groups(groups, logical_operator);
                list.shuffle(&mut rand::thread_rng());
                return &list.first().unwrap().base;
            } else {
                let mut list = self.adjectives_with_groups(groups, logical_operator);
                list.shuffle(&mut rand::thread_rng());
                return &list.first().unwrap().base;
            }
        }
        pub fn nouns_with_groups(&self, groups: Vec<String>, logical_operator: LogicalOperator) -> Vec<&Noun> {
            let mut initial_ids = self
                .noun_groups
                .get(groups.first().unwrap())
                .unwrap()
                .clone();
            for i in 1..groups.len() {
                let common_ids = self.noun_groups.get(groups.get(i).unwrap()).unwrap();
                if logical_operator.eq(&LogicalOperator::AND) {
                    initial_ids.retain(|id| common_ids.contains(&id));
                } else {
                    for id in common_ids {
                        initial_ids.insert(*id);
                    }
                }
            }
            return initial_ids
                .iter()
                .map(|id| self.nouns.get(id).unwrap())
                .collect();
        }

        pub fn pick_noun_with_groups(&self, groups: Vec<String>) -> &Noun {
            let mut options = self.nouns_with_groups(groups, LogicalOperator::AND);
            options.shuffle(&mut rand::thread_rng());
            return options.first().unwrap();
        }

        pub fn adjectives_with_groups(&self, groups: Vec<String>, logical_operator: LogicalOperator) -> Vec<&Adjective> {
            let mut initial_ids = self
                .adjective_groups
                .get(groups.first().unwrap())
                .unwrap()
                .clone();
            for i in 1..groups.len() {
                let common_ids = self.adjective_groups.get(groups.get(i).unwrap()).unwrap();

                if logical_operator.eq(&LogicalOperator::AND) {
                    initial_ids.retain(|id| common_ids.contains(&id));
                } else {
                    for id in common_ids {
                        initial_ids.insert(*id);
                    }
                }
            }
            return initial_ids
                .iter()
                .map(|id| self.adjectives.get(id).unwrap())
                .collect();
        }

        fn extract_contained_term<'a>(sample: &'a str, pattern: &str) -> Option<&'a str> {
            let regex = Regex::new(pattern).unwrap();
            let capture = regex.captures(sample);
            if capture.is_some() {
                return Some(capture.unwrap().get(1).unwrap().as_str());
            } else {
                return None;
            }
        }

        fn create_noun(dict: &mut Dictionary, base: &str, data: Vec<&str>) {
            let id = Uuid::new_v4();
            let mut noun = Noun {
                id,
                base: String::from(base),
                recipie: None,
                groups: HashSet::new(),
                adjective: None,
            };

            for e in data {
                let entry = e.trim();
                let group_attempt = Self::extract_contained_term(entry, GROUP_PATTERN);
                if group_attempt.is_some() {
                    let group = group_attempt.unwrap();
                    noun.groups.insert(String::from(group));
                    if !dict.noun_groups.contains_key(group) {
                        dict.noun_groups.insert(String::from(group), HashSet::new());
                    }
                    dict.noun_groups.get_mut(group).unwrap().insert(id);
                }
                let adjective_attempt = Self::extract_contained_term(entry, HAS_ADJECTIVE_PATTERN);
                if adjective_attempt.is_some() {
                    let adjective = adjective_attempt.unwrap();
                    noun.adjective = Some(String::from(adjective));
                }
            }
            dict.nouns.insert(id.clone(), noun);
        }

        fn create_adjective(dict: &mut Dictionary, base: &str, data: Vec<&str>) {
            let id = Uuid::new_v4();
            let mut adjective = Adjective {
                id,
                base: String::from(base),
                groups: HashSet::new(),
                noun: None,
            };
            for e in data {
                let entry = e.trim();
                let group_attempt = Self::extract_contained_term(entry, GROUP_PATTERN);
                if group_attempt.is_some() {
                    let group = group_attempt.unwrap();
                    adjective.groups.insert(String::from(group));
                    if !dict.adjective_groups.contains_key(group) {
                        dict.adjective_groups
                            .insert(String::from(group), HashSet::new());
                    }
                    dict.adjective_groups.get_mut(group).unwrap().insert(id);
                }
                let noun_attempt = Self::extract_contained_term(entry, HAS_NOUN_PATTERN);
                if noun_attempt.is_some() {
                    let noun = noun_attempt.unwrap();
                    adjective.noun = Some(String::from(noun));
                }
            }
            dict.adjectives.insert(id, adjective);
        }

        fn create_template(dict: &mut Dictionary, template: &str, data: Vec<&str>) {
            let id = Uuid::new_v4();
            let mut template = Template {
                id,
                template: String::from(template),
                groups: HashSet::new()
            };
            for e in data {
                let entry = e.trim();
                let group_attempt = Self::extract_contained_term(entry, GROUP_PATTERN);
                if group_attempt.is_some() {
                    let group = group_attempt.unwrap();
                    template.groups.insert(String::from(group));
                    if !dict.template_groups.contains_key(group) {
                        dict.template_groups.insert(String::from(group), HashSet::new());
                    }
                    dict.template_groups.get_mut(group).unwrap().insert(id);
                }
            }
            dict.templates.insert(id, template);
        }

        fn append_group_groups(&mut self) {
            for (parent_tag, child_tags) in self.group_groups.iter() {
                let noun_groups = self.noun_groups.clone();
                let adjective_groups = self.adjective_groups.clone();
                for child_tag in child_tags {
                    // nouns
                    if self.noun_groups.contains_key(child_tag) {
                        for noun_id in noun_groups.get(child_tag).unwrap() {
                            if !self.noun_groups.contains_key(parent_tag) {
                                self.noun_groups
                                    .insert(String::from(parent_tag), HashSet::new());
                            }
                            self.noun_groups
                                .get_mut(parent_tag)
                                .unwrap()
                                .insert(noun_id.clone());
                            let noun = self.nouns.get_mut(noun_id).unwrap();
                            noun.groups.insert(String::from(parent_tag));
                            drop(noun);
                        }
                    }
                    if self.adjective_groups.contains_key(child_tag) {
                        for adjective_id in adjective_groups.get(child_tag).unwrap() {
                            if !self.adjective_groups.contains_key(parent_tag) {
                                self.adjective_groups
                                    .insert(String::from(parent_tag), HashSet::new());
                            }
                            self.adjective_groups
                                .get_mut(parent_tag)
                                .unwrap()
                                .insert(adjective_id.clone());
                            let adjective = self.nouns.get_mut(adjective_id).unwrap();
                            adjective.groups.insert(String::from(parent_tag));
                            drop(adjective);
                        }
                    }
                }
            }
        }

        pub fn parse_datafile_line(&mut self, line: String) {
            let mut elements = line.split(",");
            let base = elements.next().unwrap();
            let data: Vec<&str> = elements.map(|element| element.trim()).collect();
            
            if data.contains(&GROUP_FLAG) {
                for e in data {
                    let entry = e.trim();
                    let group_attempt = Self::extract_contained_term(entry, GROUP_PATTERN);
                    if group_attempt.is_some() {
                        let group = group_attempt.unwrap();
                        if self.group_groups.contains_key(group) {
                            self.group_groups
                                .get_mut(group)
                                .unwrap()
                                .insert(String::from(base));
                        } else {
                            let mut set = HashSet::new();
                            set.insert(String::from(base));
                            self.group_groups.insert(String::from(group), set);
                        }
                    }
                }
            } else if data.iter().any(|e| e.trim().eq(NOUN_FLAG)) {
                Self::create_noun(self, base, data);
            } else if data.iter().any(|e| e.trim().eq(ADJECTIVE_FLAG)) {
                Self::create_adjective(self, base, data);
            } else if data.iter().any(|e| e.trim().eq(TEMPLATE_FLAG)) {
                Self::create_template(self, base, data);
            }
        }
    }

    pub fn build_dictionary() -> Dictionary {
        let paths = fs::read_dir("./src/language2/data_files").unwrap();
        let mut dict = Dictionary {
            nouns: HashMap::new(),
            noun_groups: HashMap::new(),
            adjectives: HashMap::new(),
            adjective_groups: HashMap::new(),
            group_groups: HashMap::new(),
            templates: HashMap::new(),
            template_groups: HashMap::new(),
        };
        for path in paths {
            let filename = path.unwrap().file_name();
            let filename_string = filename.to_str().unwrap();
            let data = File::open(&format!("./src/language2/data_files/{}", filename_string))
                .expect(&format!("Cannot Open: {}", filename_string));
            let lines = io::BufReader::new(data).lines();
            for l in lines {
                if l.is_ok() {
                    let line = l.unwrap();
                    dict.parse_datafile_line(line);
                }
            }
        }
        for _i in 0..10 {
            dict.append_group_groups();
        }
        return dict;
    }

    #[cfg(test)]
    pub mod tests {
        use std::collections::HashMap;

        use crate::language2::language2::{build_dictionary, LogicalOperator};

        use super::Dictionary;

        // #[test]
        // fn test_parsing() {
        //     let mut dict = Dictionary {
        //         nouns: HashMap::new(),
        //         noun_groups: HashMap::new(),
        //         adjectives: HashMap::new(),
        //         adjective_groups: HashMap::new(),
        //         group_groups: HashMap::new(),
        //     };
        //     dict.parse_datafile_line(String::from(
        //         "Gold, NOUN, HAS_GROUP(Metal), HAS_ADJECTIVE(Golden)",
        //     ));
        //     dict.parse_datafile_line(String::from("Metal, GROUP, HAS_GROUP(Material)"));
        //     dict.append_group_groups();
        //     println!("After: {:#?}", dict);
        // }

        // #[test]
        // fn full_parsing() {
        //     let dict = build_dictionary();
        //     // println!("Full Dict: {:#?}", dict);
        //     println!("Noun Groups: {:#?}", dict.noun_groups.keys());
        //     println!("Adjective Groups: {:#?}", dict.adjective_groups.keys());
        // }

        #[test]
        fn tag_and_filter() {
            let dict = build_dictionary();
            println!("{:#?}", dict.templates);
            // println!("{:#?}", dict.nouns_with_groups(vec![String::from("Metal")], LogicalOperator::AND));
        }
    }
}
