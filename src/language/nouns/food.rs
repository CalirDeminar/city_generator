pub mod food {
    use rand::seq::SliceRandom;
    use rand::Rng;
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter};
    use uuid::Uuid;

    use crate::{
        culture::culture::CultureConfig,
        language::{
            language::{build_dictionary, Word},
            nouns::{
                creatures::creatures::{CreatureCategory, CreatureFamily},
                nouns::NounTag,
                plants::plants::PlantType,
            },
        },
    };

    const CULTURAL_FOOD_PREFERENCE_RATE: f32 = 0.33;

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

    pub fn random_vegetable(dict: &Vec<Word>, culture: &Option<CultureConfig>) -> String {
        let mut rng = rand::thread_rng();
        let mut veg: Vec<&Word> = dict
            .iter()
            .filter(|w| {
                w.tags.contains(&PlantType::PlantTypeCrop.to_string())
                    && (!w.tags.contains(&PlantType::PlantTypeFruit.to_string())
                        && !w.tags.contains(&PlantType::PlantTypeGrain.to_string()))
            })
            .collect();

        let mut staples_ids: Vec<Uuid> = Vec::new();
        if culture.is_some() {
            let c = culture.clone().unwrap();
            for s in c.staple_plants {
                if veg.contains(&&s) {
                    staples_ids.push(s.id);
                }
            }
        };
        if staples_ids.len() > 0 && rng.gen::<f32>() < CULTURAL_FOOD_PREFERENCE_RATE {
            veg.retain(|b: &&Word| staples_ids.contains(&b.id));
        }

        veg.shuffle(&mut rand::thread_rng());
        return veg.first().unwrap().text.clone();
    }

    pub fn random_meat<'a>(
        dict: &'a Vec<Word>,
        culture: &'a Option<CultureConfig>,
        creature_type: &'a CreatureFamily,
    ) -> (CreatureFamily, &'a Word) {
        let mut rng = rand::thread_rng();
        let mut creatures: Vec<&Word> = dict
            .iter()
            .filter(|w| {
                w.tags
                    .contains(&CreatureCategory::CreatureAnimal.to_string())
                    && w.tags.contains(&creature_type.to_string())
            })
            .collect();

        let mut staples_ids: Vec<Uuid> = Vec::new();
        if culture.is_some() {
            let c = culture.clone().unwrap();
            for s in c.staple_meats {
                if creatures.contains(&&s) {
                    staples_ids.push(s.id);
                }
            }
        };
        if staples_ids.len() > 0 && rng.gen::<f32>() < CULTURAL_FOOD_PREFERENCE_RATE {
            creatures.retain(|b: &&Word| staples_ids.contains(&b.id));
        }

        creatures.shuffle(&mut rand::thread_rng());
        return (creature_type.clone(), creatures.first().unwrap());
    }

    pub fn random_alcohol<'a>(
        dict: &'a Vec<Word>,
        culture: &'a Option<CultureConfig>,
    ) -> (AlcoholTypes, &'a Word) {
        let mut rng = rand::thread_rng();
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
        let mut staples_ids: Vec<Uuid> = Vec::new();
        if culture.is_some() {
            let c = culture.clone().unwrap();
            for s in c.staple_plants {
                if bases.contains(&&s) {
                    staples_ids.push(s.id);
                }
            }
        };
        if staples_ids.len() > 0 && rng.gen::<f32>() < CULTURAL_FOOD_PREFERENCE_RATE {
            bases.retain(|b: &&Word| staples_ids.contains(&b.id));
        }

        bases.shuffle(&mut rand::thread_rng());
        return (alcohol_chosen.clone(), bases.first().unwrap());
    }

    #[test]
    fn test_random_foods() {
        let dict = build_dictionary();
        println!("\nVegetables:");
        for _i in 0..10 {
            println!("{}", random_vegetable(&dict, &None));
        }
        println!("\nMeats Mammal:");
        for _i in 0..10 {
            println!(
                "{:?}",
                random_meat(&dict, &None, &CreatureFamily::CreatureFamilyMammal)
            );
        }
        println!("\nMeats Bird:");
        for _i in 0..10 {
            println!(
                "{:?}",
                random_meat(&dict, &None, &CreatureFamily::CreatureFamilyBird)
            );
        }
        println!("\nMeats Fish:");
        for _i in 0..10 {
            println!(
                "{:?}",
                random_meat(&dict, &None, &CreatureFamily::CreatureFamilyFish)
            );
        }
        println!("\nAlcohols:");
        for _i in 0..10 {
            println!("{:?}", random_alcohol(&dict, &None));
        }
    }
}
