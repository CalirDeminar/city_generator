pub mod food_institutions {
    
    use rand::Rng;
    use uuid::Uuid;

    use crate::city::institutions::institutions::{Institution, InstituteType, PRIVATE_INSTITUTE_BASE_SIZE};
    use crate::culture::culture::*;
    use crate::language::language::*;

    use crate::language::nouns::food::food::{
        random_dish_type, random_food_product_of_type, 
        MealProducts,
    };
    use crate::language::nouns::nouns::NounTag;
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
            "{{{{Adjective(Position, Quality, Age, Colour)}}}} {{{{Noun(LastName)}}}} {} {{{{Noun(RetailerFoodSpecialist)}}}}",
            random_food_type.text
        ),
        format!(
            "{{{{Adjective(Position, Quality, Age, Colour)}}}} {{{{Noun(LastName)}}}}'s {} {{{{Noun(RetailerFoodSpecialist)}}}}",
            
            random_food_type.text
        ),
        format!(
            "{{{{Noun(LastName)}}}} {} {{{{Noun(RetailerFoodSpecialist)}}}}",
            
            random_food_type.text
        ),format!(
            "{{{{Noun(LastName)}}}}'s {} {{{{Noun(RetailerFoodSpecialist)}}}}",
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
            customer_cost_multipler: rng.gen::<f32>() * 5.0,
            annual_visits: 0,
            wealth: 0
        }
    }

    pub fn random_general_food_outlet(dict: &Vec<Word>, culture: &Option<CultureConfig>) -> Institution  {
        let mut rng = rand::thread_rng();
        let era = if culture.is_some() {
            culture.clone().unwrap().era
        } else {
            None
        };
        let mut menu: Vec<String> = Vec::new();
        let inst_type = random_word_by_tag(&dict, WordType::Noun, &vec![NounTag::RetailerFood.to_string()], &vec![], &vec![],& era).unwrap();
        let menu_base: Vec<String> = inst_type.tags.iter().filter(|t| t.contains("Serves(")).map(|t| t.replace("Serves(","").replace(")","")).collect();
        for item in menu_base {
            let base_food_type = dict.iter().find(|w| w.tags.contains(&NounTag::FoodProduct.to_string()) && w.text.eq(&item)).unwrap();
            for _i in 0..(rng.gen::<f32>() * 3.0).max(1.0) as usize {
                let item = random_food_product_of_type(&dict, &culture, base_food_type);
                if !menu.contains(&item) {
                    menu.push(item);
                }
            }
        }
        let templates = vec![
            "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(LastName)}} ",
             "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(HistoricalFigure)}}'s ",
             "{{Noun(LastName)}} ",
             "{{Noun(LastName)}}'s ",
             "{{Noun(HistoricalFigure)}}'s ",
        ];
        let name = format!("{} {}", render_template_2(random_pick(&templates), &dict, &era), inst_type.text);
        return Institution {
            id: Uuid::new_v4(),
            name,
            public: false,
            institute_type: InstituteType::FoodService,
            size: (rng.gen::<f32>() * PRIVATE_INSTITUTE_BASE_SIZE as f32) as usize,
            serves: menu,
            customer_cost_multipler: rng.gen::<f32>() * 5.0,
            annual_visits: 0,
            wealth: 0
        };
    }

        #[test]
    fn test_random_spec_food_inst() {
        let dict = build_dictionary();
        let culture = random_culture(&dict, &Some(Era::Modern));
        println!("Specialist Food Outlets:");
        for _i in 0..10 {
            let inst = random_specialist_food_outlet(&dict, &Some(culture.clone()));
            println!(
                "{:?}: {:?}",
                inst.name, inst.serves
            );

        }
        println!("General Food Outlets:");
        for _i in 0..10 {
            let inst = random_general_food_outlet(&dict, &Some(culture.clone()));
            println!(
                "{:?}: {:?}",
                inst.name, inst.serves
            );
        }
    }
}
