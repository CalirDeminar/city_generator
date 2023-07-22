pub mod food_institutions {
    use rand::seq::SliceRandom;
    use rand::Rng;
    use strum::IntoEnumIterator;
    use uuid::Uuid;

    use crate::city::institutions::institutions::{Institution, InstituteType, PRIVATE_INSTITUTE_BASE_SIZE};
    use crate::culture::culture::{random_culture, CultureConfig};
    use crate::language::language::{
        build_dictionary, Era, Word, WordType,
    };

    use crate::language::nouns::food::food::{
        random_dish_type, random_food_product_of_type, 
        MealProducts,
    };
    use crate::templater::templater::render_template_2;
    use crate::utils::utils::random_pick;

    pub fn random_specialist_food_outlet(dict: &Vec<Word>, culture: &Option<CultureConfig>) -> Institution {
        let mut rng = rand::thread_rng();
        let era = if culture.is_some() {
            culture.clone().unwrap().era
        } else {
            None
        };

        let random_food_type = random_dish_type(&dict, &culture, &MealProducts::FoodDish);
        let mut menu: Vec<String> = Vec::new();
        for _i in 0..(rng.gen::<f32>() * 8.0) as usize {
            let menu_item = random_food_product_of_type(
                &dict,
                &culture,
                &random_food_type,
            );
            if !menu.contains(&menu_item){
                menu.push(menu_item);
            }
            
        }

        let templates_pre: Vec<String> = vec![format!(
            "{{{{Adjective(Position, Quality, Age, Colour)}}}} {{{{Noun(LastName)}}}} {} {{{{Noun(SpecialistFoodService)}}}}",
            random_food_type.text
        ),
        format!(
            "{{{{Adjective(Position, Quality, Age, Colour)}}}} {{{{Noun(LastName)}}}}'s {} {{{{Noun(SpecialistFoodService)}}}}",
            
            random_food_type.text
        ),
        format!(
            "{{{{Noun(LastName)}}}} {} {{{{Noun(SpecialistFoodService)}}}}",
            
            random_food_type.text
        ),format!(
            "{{{{Noun(LastName)}}}}'s {} {{{{Noun(SpecialistFoodService)}}}}",
            random_food_type.text
        ),];
        let templates: Vec<&str> = templates_pre.iter().map(|i| i.as_str()).collect();
        return Institution {
            id: Uuid::new_v4(),
            name: render_template_2(random_pick(&templates), &dict, &era),
            public: false,
            institute_type: InstituteType::SpecialistFoodService,
            size: (rng.gen::<f32>() * PRIVATE_INSTITUTE_BASE_SIZE as f32) as usize,
            serves: menu,
        }
    }

        #[test]
    fn test_random_spec_food_inst() {
        let dict = build_dictionary();
        let culture = random_culture(&dict, &Some(Era::Modern));
        println!("Specialist Food Outlets:");
        for _i in 0..10 {
            println!(
                "{:?}",
                random_specialist_food_outlet(&dict, &Some(culture.clone()))
            );
        }
    }
}
