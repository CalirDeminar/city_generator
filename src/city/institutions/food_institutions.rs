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
    use crate::language::nouns::food::food::{
        random_alcohol, random_meat, random_vegetable, FoodServingTypes,
    };

    pub fn create_food_institution(dict: &Vec<Word>, culture: &Option<CultureConfig>) {
        let mut rng = rand::thread_rng();
        // let mut output = Institution {
        //     id: Uuid::new_v4(),
        //     name: String::new(),
        //     public: false,
        //     institute_type: InstituteType::FoodService,
        //     size: (rng.gen::<f32>() * PRIVATE_INSTITUTE_BASE_SIZE as f32) as usize,
        //     serves: Vec::new(),
        // };
        let ftype =
            random_word_by_tag_and(&dict, WordType::Noun, vec![String::from("RetailerFood")])
                .unwrap();
        let food_types: Vec<String> = ftype
            .tags
            .iter()
            .filter(|t| FoodServingTypes::iter().any(|f| t.eq(&&f.to_string())))
            .map(|s| String::from(s))
            .collect();
        println!("\nInst Type: {}", ftype.text);
        for food in food_types {
            println!("Food Type: {}", food);
            for _i in 0..((rng.gen::<f32>() * 4.0) as usize).max(1) {
                if food.eq(&"Vegetable") {
                    println!("{:?}", random_vegetable(&dict, &culture));
                } else if food.eq(&"MeatMammal") {
                    println!(
                        "{:?}",
                        random_meat(&dict, &culture, &CreatureFamily::CreatureFamilyMammal)
                    )
                } else if food.eq(&"MeatBird") {
                    println!(
                        "{:?}",
                        random_meat(&dict, &culture, &CreatureFamily::CreatureFamilyBird)
                    )
                } else if food.eq(&"MeatFish") {
                    println!(
                        "{:?}",
                        random_meat(&dict, &culture, &CreatureFamily::CreatureFamilyFish)
                    )
                } else if food.eq(&"DrinkAlcohol") {
                    println!("{:?}", random_alcohol(&dict, &culture))
                }
            }
        }
    }

    #[test]
    fn gen_institutions_test() {
        let dict = build_dictionary();
        let culture = Some(random_culture(&dict, &None));
        for _i in 0..10 {
            create_food_institution(&dict, &culture);
        }
    }
}
