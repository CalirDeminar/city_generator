pub mod adjectives;
pub mod nouns;
pub mod language {
    use std::time::Instant;

    use rand::seq::SliceRandom;
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter};
    use uuid::Uuid;

    use super::{adjectives::adjectives::build_adjectives, nouns::nouns::build_nouns};

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum Era {
        #[default]
        Modern,
        Future,
        Fantasy,
        Medieval,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum WordType {
        Noun,
        Adjective,
        Verb,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Word {
        pub id: Uuid,
        pub word_type: WordType,
        pub related_forms: Vec<Word>,
        pub text: String,
        pub tags: Vec<String>,
    }

    pub fn filter_words_by_tag_and(
        words: &Vec<Word>,
        word_type: WordType,
        tags: Vec<String>,
    ) -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        for word in words {
            if word.word_type.eq(&word_type) && tags.iter().all(|t| word.tags.contains(&t)) {
                output.push(word.clone());
            }
        }
        return output;
    }

    pub fn random_word_by_tag_and(
        words: &Vec<Word>,
        word_type: WordType,
        tags: Vec<String>,
    ) -> Option<Word> {
        let mut output: Vec<Word> = Vec::new();
        for word in words {
            if word.word_type.eq(&word_type) && tags.iter().all(|t| word.tags.contains(&t)) {
                output.push(word.clone());
            }
        }
        output.shuffle(&mut rand::thread_rng());
        for word in output {
            return Some(word);
        }
        return None;
    }

    pub fn filter_words_by_tag_or(
        words: Vec<&Word>,
        word_type: WordType,
        tags: Vec<String>,
    ) -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        for word in words {
            if word.word_type.eq(&word_type) && tags.iter().any(|t| word.tags.contains(&t)) {
                output.push(word.clone());
            }
        }
        return output;
    }

    pub fn random_word_by_tag(
        words: &Vec<Word>,
        word_type: WordType,
        all_of: &Vec<String>,
        one_of: &Vec<String>,
        none_of: &Vec<String>,
        era: &Option<Era>,
    ) -> Option<Word> {
        let mut output: Vec<Word> = Vec::new();
        let possible_eras: Vec<String> = Era::iter().map(|e| e.to_string()).collect();
        for word in words {
            if word.word_type.eq(&word_type)
                && (all_of.iter().all(|t| word.tags.contains(&t)) || all_of.len() == 0)
                && (one_of.iter().any(|t| word.tags.contains(&t)) || one_of.len() == 0)
                && (!none_of.iter().all(|t| word.tags.contains(&t)) || none_of.len() == 0)
                && (era.is_none() // match if we haven't selected an era
                    || word.tags.contains(&era.unwrap().to_string()) // match if the word has this era tag
                    || possible_eras.iter().all(|e| !word.tags.contains(e)))
            // match if this word has no era tag of any sirt
            {
                output.push(word.clone());
            }
        }
        output.shuffle(&mut rand::thread_rng());
        for word in output {
            return Some(word);
        }
        return None;
    }

    pub fn build_dictionary() -> Vec<Word> {
        let start = Instant::now();
        let mut output: Vec<Vec<Word>> = Vec::new();
        output.push(build_nouns());
        output.push(build_adjectives());
        let rtn = output.concat();
        println!(
            "Dictionary Build in {}ms for {} words",
            start.elapsed().as_millis(),
            rtn.len()
        );
        return rtn;
    }
}
