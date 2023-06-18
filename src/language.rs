pub mod nouns;
pub mod language {
    use strum_macros::{Display, EnumIter};
    use uuid::Uuid;

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

    pub fn filter_words_by_tag_and(words: &Vec<Word>, tags: Vec<WordTag>) -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        for word in words {
            if tags.iter().all(|t| word.tags.contains(&t)) {
                output.push(word.clone());
            }
        }
        return output;
    }

    pub fn filter_words_by_tag_or(words: Vec<&Word>, tags: Vec<WordTag>) -> Vec<Word> {
        let mut output: Vec<Word> = Vec::new();
        for word in words {
            if tags.iter().any(|t| word.tags.contains(&t)) {
                output.push(word.clone());
            }
        }
        return output;
    }
}
