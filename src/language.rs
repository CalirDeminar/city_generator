pub mod nouns;
pub mod language {
    use uuid::Uuid;

    use super::nouns::nouns::NounTag;
    use strum_macros::{Display, EnumIter};

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
        Noun(NounTag),
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
}
