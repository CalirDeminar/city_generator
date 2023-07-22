pub mod food_institutions {
    use rand::seq::SliceRandom;
    use rand::Rng;
    use strum::IntoEnumIterator;
    use uuid::Uuid;

    use crate::culture::culture::{random_culture, CultureConfig};
    use crate::language::language::{
        build_dictionary, random_word_by_tag, random_word_by_tag_and, Era, Word, WordType,
    };
    use crate::language::nouns::creatures::creatures::CreatureFamily;
    use crate::language::nouns::food::food::{FoodConditionTags, FoodServingTypes, MealProducts};
}
