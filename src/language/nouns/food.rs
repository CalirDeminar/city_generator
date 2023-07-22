pub mod food {
    use rand::seq::SliceRandom;
    use rand::Rng;
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter};
    use uuid::Uuid;

    use crate::{
        culture::culture::{random_culture, CultureConfig},
        language::{
            language::{
                build_dictionary, filter_words_by_tag_and, random_word_by_tag, Era, Word, WordType,
            },
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
        BrewableRum,
        BrewableWhiskey,
        Fruit,
        Grain,
        Leaf,
        Vegetable,
        MeatMammal,
        MeatBird,
        MeatFish,
        Cheese,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum FoodCourseTypes {
        Savory,
        Dessert,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum FoodServingTypes {
        FoodDish,
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

    #[derive(PartialEq, Debug, Clone, EnumIter, Display)]
    pub enum MealProducts {
        FoodDish,
        DrinkAlcohol,
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
        for tag in MealProducts::iter() {
            output.push(tag.to_string());
        }
        return output;
    }

    pub fn random_ingedient<'a>(
        dict: &'a Vec<Word>,
        culture: &'a Option<CultureConfig>,
        include: Vec<String>,
        exclude: Vec<String>,
    ) -> &'a Word {
        let base_exclude_list: Vec<String> = vec![
            CreatureCategory::CreatureSentient.to_string(),
            CreatureCategory::CreatureMagical.to_string(),
            NounTag::FoodProduct.to_string(),
        ];
        let mut rng = rand::thread_rng();
        let mut ingredients: Vec<&Word> = dict
            .iter()
            .filter(|w| {
                include.iter().all(|i| w.tags.contains(&i))
                    && !exclude.iter().any(|x| w.tags.contains(&x))
                    && !base_exclude_list.iter().any(|x| w.tags.contains(&x))
            })
            .collect();

        let mut staples_ids: Vec<Uuid> = Vec::new();
        if culture.is_some() {
            let c = culture.clone().unwrap();
            for s in c.staple_plants {
                if ingredients.contains(&&s) {
                    staples_ids.push(s.id);
                }
            }
            for s in c.staple_meats {
                if ingredients.contains(&&s) {
                    staples_ids.push(s.id);
                }
            }
        };
        if staples_ids.len() > 0 && rng.gen::<f32>() < CULTURAL_FOOD_PREFERENCE_RATE {
            ingredients.retain(|b: &&Word| staples_ids.contains(&b.id));
        }

        ingredients.shuffle(&mut rand::thread_rng());
        return ingredients.first().unwrap().clone();
    }

    pub fn random_food_product_of_type(
        dict: &Vec<Word>,
        culture: &Option<CultureConfig>,
        dish_type: &Word,
    ) -> String {
        let mut output = String::new();
        let mut dish_variations: Vec<&Word> = dict
            .iter()
            .filter(|w| {
                w.text.eq(&dish_type.text) && w.tags.contains(&NounTag::FoodProduct.to_string())
            })
            .collect();
        dish_variations.shuffle(&mut rand::thread_rng());
        let target_dish = dish_variations.first().unwrap();

        let ingredients: Vec<String> = target_dish
            .tags
            .iter()
            .filter(|t| FoodConditionTags::iter().any(|f| t.eq(&&f.to_string())))
            .map(|s| String::from(s))
            .collect();

        // println!(
        //     "Dish: {:?}: {:#?} -> {:#?}",
        //     target_dish.text, target_dish.tags, ingredients
        // );
        for (i, food) in ingredients.iter().enumerate() {
            if output.len() > 0 && i.eq(&(ingredients.len() - 1)) {
                output.push_str(" and ");
            } else if output.len() > 0 {
                output.push_str(", ");
            }
            if food.eq(&"Vegetable") {
                output.push_str(
                    &random_ingedient(
                        &dict,
                        &culture,
                        vec![PlantType::PlantTypeCrop.to_string()],
                        vec![
                            PlantType::PlantTypeFruit.to_string(),
                            PlantType::PlantTypeGrain.to_string(),
                        ],
                    )
                    .text,
                );
            }
            if food.eq(&"Fruit") {
                output.push_str(
                    &random_ingedient(
                        &dict,
                        &culture,
                        vec![PlantType::PlantTypeFruit.to_string()],
                        vec![],
                    )
                    .text,
                );
            } else if food.eq(&"MeatMammal") {
                output.push_str(
                    &random_ingedient(
                        &dict,
                        &culture,
                        vec![CreatureFamily::CreatureFamilyMammal.to_string()],
                        vec![],
                    )
                    .text,
                );
            } else if food.eq(&"MeatBird") {
                output.push_str(
                    &random_ingedient(
                        &dict,
                        &culture,
                        vec![CreatureFamily::CreatureFamilyBird.to_string()],
                        vec![],
                    )
                    .text,
                );
            } else if food.eq(&"MeatFish") {
                output.push_str(
                    &random_ingedient(
                        &dict,
                        &culture,
                        vec![CreatureFamily::CreatureFamilyFish.to_string()],
                        vec![],
                    )
                    .text,
                );
            } else if food.eq(&"Cheese") {
                output.push_str(&format!(
                    "{} Cheese",
                    random_ingedient(
                        &dict,
                        &culture,
                        vec![CreatureFamily::CreatureFamilyMammal.to_string()],
                        vec![],
                    )
                    .text
                ));
            } else if food.eq(&"BrewableBeer") {
                output.push_str(
                    &random_ingedient(
                        &dict,
                        &culture,
                        vec![FoodConditionTags::BrewableBeer.to_string()],
                        vec![],
                    )
                    .text,
                );
            } else if food.eq(&"BrewableCider") {
                output.push_str(
                    &random_ingedient(
                        &dict,
                        &culture,
                        vec![PlantType::PlantTypeFruit.to_string()],
                        vec![],
                    )
                    .text,
                );
            } else if food.eq(&"BrewableWine") {
                output.push_str(
                    &random_ingedient(
                        &dict,
                        &culture,
                        vec![PlantType::PlantTypeFruit.to_string()],
                        vec![],
                    )
                    .text,
                );
            }
        }
        output.push_str(&format!(" {}", dish_type.text));
        return String::from(output.trim());
    }

    pub fn random_dish_type(
        dict: &Vec<Word>,
        culture: &Option<CultureConfig>,
        product_type: &MealProducts,
    ) -> Word {
        let era = if culture.is_some() {
            culture.clone().unwrap().era
        } else {
            None
        };
        let mut dish_types_unique: Vec<String> = Vec::new();
        let mut target_tags = vec![NounTag::FoodProduct.to_string(), product_type.to_string()];
        if era.is_some() {
            target_tags.push(era.unwrap().to_string());
        }
        let mut dish_types_repeats = filter_words_by_tag_and(
            &dict,
            WordType::Noun,
            vec![NounTag::FoodProduct.to_string(), product_type.to_string()],
        );
        for word in &dish_types_repeats {
            if !dish_types_unique.contains(&word.text) {
                dish_types_unique.push(word.text.clone());
            }
        }
        dish_types_unique.shuffle(&mut rand::thread_rng());
        let target_dish = dish_types_unique.first().unwrap().clone();
        dish_types_repeats.retain(|w| w.text.eq(&target_dish));
        dish_types_repeats.shuffle(&mut rand::thread_rng());
        return dish_types_repeats.first().unwrap().clone();
    }

    pub fn random_food_product(
        dict: &Vec<Word>,
        culture: &Option<CultureConfig>,
        product_type: MealProducts,
    ) -> String {
        return random_food_product_of_type(
            dict,
            culture,
            &random_dish_type(&dict, &culture, &product_type),
        );
    }

    #[test]
    fn test_random_foods() {
        let dict = build_dictionary();
        let culture = random_culture(&dict, &Some(Era::Modern));
        println!("Meals:");
        for _i in 0..20 {
            println!(
                "{:?}",
                random_food_product(&dict, &Some(culture.clone()), MealProducts::FoodDish)
            );
        }
        println!("Drinks");
        for _i in 0..10 {
            println!(
                "{:?}",
                random_food_product(&dict, &Some(culture.clone()), MealProducts::DrinkAlcohol)
            );
        }
    }
}
