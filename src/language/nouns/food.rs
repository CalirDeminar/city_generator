pub mod food {
    use rand::seq::SliceRandom;
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter};

    use crate::language::{
        language::{build_dictionary, Word},
        nouns::{
            creatures::creatures::{CreatureCategory, CreatureFamily},
            nouns::NounTag,
            plants::plants::PlantType,
        },
    };

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum FoodConditionTags {
        Food,
        BrewableWine,
        BrewableBeer,
        BrewableCider,
        BrewableMead,
        BrewableAle,
        BrewableRum,
        BrewableWhiskey,
        Fruit,
        Grain,
        Leaf,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum FoodCourseTypes {
        Savory,
        Dessert,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum FoodServingTypes {
        Vegetable,
        MeatMammal,
        MeatBird,
        MeatFish,
        DrinkSoft,
        DrinkAlcohol,
        DrinkSpirit,
        DrinkHot,
        Baked,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum AlcoholTypes {
        Wine,
        Beer,
        Cider,
        // Mead,
        Ale,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum AlcoholSpiritTypes {
        Whisky,
        Rum,
    }

    pub fn match_food_serving_type_to_tag(tag: FoodServingTypes) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        return output;
    }

    pub fn food_tags() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for tag in FoodServingTypes::iter() {
            output.push(tag.to_string());
        }
        for tag in FoodCourseTypes::iter() {
            output.push(tag.to_string());
        }
        for tag in FoodConditionTags::iter() {
            output.push(tag.to_string());
        }
        return output;
    }

    pub fn random_vegetable(dict: &Vec<Word>) -> String {
        let mut veg: Vec<&Word> = dict
            .iter()
            .filter(|w| {
                w.tags.contains(&PlantType::PlantTypeCrop.to_string())
                    && (!w.tags.contains(&PlantType::PlantTypeFruit.to_string())
                        || !w.tags.contains(&PlantType::PlantTypeGrain.to_string()))
            })
            .collect();
        veg.shuffle(&mut rand::thread_rng());
        return veg.first().unwrap().text.clone();
    }

    pub fn random_meat(dict: &Vec<Word>, creature_type: &CreatureFamily) -> String {
        let mut creatures: Vec<&Word> = dict
            .iter()
            .filter(|w| {
                w.tags
                    .contains(&CreatureCategory::CreatureAnimal.to_string())
                    && w.tags.contains(&creature_type.to_string())
            })
            .collect();
        creatures.shuffle(&mut rand::thread_rng());
        return creatures.first().unwrap().text.clone();
    }

    pub fn random_alcohol(dict: &Vec<Word>) -> String {
        let mut alcohol_types: Vec<AlcoholTypes> = AlcoholTypes::iter().collect();
        alcohol_types.shuffle(&mut rand::thread_rng());
        let alcohol_chosen = alcohol_types.first().unwrap();
        let brewing_tag = match alcohol_chosen {
            AlcoholTypes::Beer => FoodConditionTags::BrewableBeer,
            AlcoholTypes::Ale => FoodConditionTags::BrewableBeer,
            AlcoholTypes::Cider => FoodConditionTags::BrewableCider,
            AlcoholTypes::Wine => FoodConditionTags::BrewableWine,
        };
        let mut bases: Vec<&Word> = dict
            .iter()
            .filter(|w| w.tags.contains(&brewing_tag.to_string()))
            .collect();
        bases.shuffle(&mut rand::thread_rng());
        let base = bases.first().unwrap().text.clone();
        return format!("{} {}", base, alcohol_chosen.to_string());
    }

    #[test]
    fn test_random_foods() {
        let dict = build_dictionary();
        println!("\nVegetables:");
        for _i in 0..10 {
            println!("{}", random_vegetable(&dict));
        }
        println!("\nMeats Mammal:");
        for _i in 0..10 {
            println!(
                "{}",
                random_meat(&dict, &CreatureFamily::CreatureFamilyMammal)
            );
        }
        println!("\nMeats Bird:");
        for _i in 0..10 {
            println!(
                "{}",
                random_meat(&dict, &CreatureFamily::CreatureFamilyBird)
            );
        }
        println!("\nMeats Fish:");
        for _i in 0..10 {
            println!(
                "{}",
                random_meat(&dict, &CreatureFamily::CreatureFamilyFish)
            );
        }
        println!("\nAlcohols:");
        for _i in 0..10 {
            println!("{}", random_alcohol(&dict));
        }
    }
}
