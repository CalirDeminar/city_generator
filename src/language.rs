pub mod adjectives;
pub mod nouns;
pub mod language {
    use rand::seq::SliceRandom;
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

    #[derive(PartialEq, Debug, Clone)]
    pub enum WordType {
        Noun,
        Adjective,
        Verb,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum WordTag {
        Noun(String),
        Adjective,
        Verb,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Word {
        pub id: Uuid,
        pub wordType: WordType,
        pub relatedForms: Vec<Word>,
        pub text: String,
        pub tags: Vec<WordTag>,
    }

    pub fn filter_words_by_tag_and(
        words: &Vec<Word>,
        word_type: WordType,
        tags: Vec<WordTag>,
    ) -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        for word in words {
            if word.wordType.eq(&word_type) && tags.iter().all(|t| word.tags.contains(&t)) {
                output.push(word.clone());
            }
        }
        return output;
    }

    pub fn random_word_by_tag_and(
        words: &Vec<Word>,
        word_type: WordType,
        tags: Vec<WordTag>,
    ) -> Option<Word> {
        let mut output: Vec<Word> = Vec::new();
        for word in words {
            if word.wordType.eq(&word_type) && tags.iter().all(|t| word.tags.contains(&t)) {
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
        tags: Vec<WordTag>,
    ) -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        for word in words {
            if word.wordType.eq(&word_type) && tags.iter().any(|t| word.tags.contains(&t)) {
                output.push(word.clone());
            }
        }
        return output;
    }

    pub fn random_word_by_tag(
        words: &Vec<Word>,
        word_type: WordType,
        all_of: &Vec<WordTag>,
        one_of: &Vec<WordTag>,
        none_of: &Vec<WordTag>,
    ) -> Option<Word> {
        let mut output: Vec<Word> = Vec::new();
        for word in words {
            if word.wordType.eq(&word_type)
                && (all_of.iter().all(|t| word.tags.contains(&t)) || all_of.len() == 0)
                && (one_of.iter().any(|t| word.tags.contains(&t)) || one_of.len() == 0)
                && (!none_of.iter().all(|t| word.tags.contains(&t)) || none_of.len() == 0)
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
        let mut output: Vec<Vec<Word>> = Vec::new();
        output.push(build_nouns());
        output.push(build_adjectives());
        return output.concat();
    }
}
