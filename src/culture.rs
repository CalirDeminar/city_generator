pub mod culture {
    use crate::language::{
        language::*,
        nouns::{
            creatures::creatures::{CreatureCategory, CreatureFamily},
            nouns::NounTag,
            plants::plants::PlantType,
        },
    };
    use rand::{seq::SliceRandom, Rng};
    use uuid::Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub struct CultureConfig {
        pub era: Option<Era>,
        pub historical_figures: Vec<(String, String)>,
        pub landlocked: bool,
        pub staple_meats: Vec<Word>,
        pub staple_plants: Vec<Word>,
        pub adult_age: u32,
        pub species_avg_lifespan: u32,
        pub species_avg_lifespan_variance: u32,
        // Format (Man's last name, Woman's last name, Male Child's name, Female Child's name)
        pub parental_naming_formats: Vec<(String, String, String, String)>,
        pub avg_building_footprint: i32,
        pub avg_building_floors: i32,
    }

    fn paternal_naming_lists() -> Vec<(String, String, String, String)> {
        return vec![
            (
                String::from("{{ML}}"),
                String::from("{{ML}}"),
                String::from("{{ML}}"),
                String::from("{{ML}}"),
            ),
            (
                String::from("{{FL}}"),
                String::from("{{FL}}"),
                String::from("{{FL}}"),
                String::from("{{FL}}"),
            ),
            (
                String::from("{{ML}}"),
                String::from("{{FL}}"),
                String::from("{{ML}}"),
                String::from("{{FL}}"),
            ),
            (
                String::from("{{ML}}-{{FL}}"),
                String::from("{{ML}}-{{FL}}"),
                String::from("{{ML}}-{{FL}}"),
                String::from("{{ML}}-{{FL}}"),
            ),
            (
                String::from("{{ML}}"),
                String::from("{{FL}}"),
                String::from("{{MF}}sson"),
                String::from("{{MF}}dotter"),
            ),
            (
                String::from("{{ML}}"),
                String::from("{{FL}}"),
                String::from("{{MF}}sen"),
                String::from("{{MF}}datter"),
            ),
        ];
    }

    fn gen_historical_figures(dict: &Vec<Word>, era: &Option<Era>) -> Vec<(String, String)> {
        let mut rng = rand::thread_rng();
        let figure_count = (rng.gen::<f32>() * 8.0) as usize;

        let mut output: Vec<(String, String)> = Vec::new();
        for _i in 0..figure_count.max(3) {
            let title = random_word_by_tag(
                &dict,
                WordType::Noun,
                &vec![String::from("Title")],
                &vec![],
                &vec![],
                era,
            )
            .unwrap()
            .text;
            let name = random_word_by_tag(
                &dict,
                WordType::Noun,
                &vec![String::from("LastName")],
                &vec![],
                &vec![],
                era,
            )
            .unwrap()
            .text;
            output.push((title, name));
            true;
        }
        return output;
    }

    fn random_animals(dict: &Vec<Word>, landlocked: bool, era: &Option<Era>) -> Vec<Word> {
        let mut rng = rand::thread_rng();
        let len = (rng.gen::<f32>() * 5.0) as usize;
        let mut output: Vec<Word> = Vec::new();
        let mut animal_types = vec![CreatureFamily::CreatureFamilyMammal.to_string()];
        if !landlocked {
            animal_types.push(CreatureFamily::CreatureFamilyFish.to_string());
        }
        for _i in 0..len.max(2) {
            output.push(
                random_word_by_tag(
                    &dict,
                    WordType::Noun,
                    &vec![CreatureCategory::CreatureAnimal.to_string()],
                    &animal_types,
                    &vec![
                        CreatureCategory::CreatureMagical.to_string(),
                        CreatureCategory::CreatureSentient.to_string(),
                        Era::Fantasy.to_string(),
                    ],
                    era,
                )
                .unwrap(),
            );
        }

        return output;
    }

    fn random_crops(dict: &Vec<Word>, era: &Option<Era>) -> Vec<Word> {
        let mut rng = rand::thread_rng();
        let len = (rng.gen::<f32>() * 7.0) as usize;
        let mut output: Vec<Word> = Vec::new();
        for _i in 0..len.max(3) {
            output.push(
                random_word_by_tag(
                    &dict,
                    WordType::Noun,
                    &vec![PlantType::PlantTypeCrop.to_string()],
                    &vec![],
                    &vec![],
                    era,
                )
                .unwrap(),
            );
        }
        return output;
    }

    pub fn random_culture(dict: &Vec<Word>, era: &Option<Era>) -> CultureConfig {
        let mut rng = rand::thread_rng();
        let landlocked = rng.gen::<f32>() > 0.5;
        let naming_system_count = ((rng.gen::<f32>() * 3.0) as usize).max(1);
        let mut naming_systems = paternal_naming_lists();
        naming_systems.shuffle(&mut rng);
        let avg_building_footprint = match era {
            Some(Era::Future) => 36,
            Some(Era::Fantasy) | Some(Era::Medieval) => 6,
            _ => 12,
        };
        let avg_building_floors = match era {
            Some(Era::Future) => 15,
            Some(Era::Fantasy) | Some(Era::Medieval) => 2,
            _ => 6,
        };
        return CultureConfig {
            era: era.clone(),
            historical_figures: gen_historical_figures(&dict, era),
            landlocked,
            staple_meats: random_animals(&dict, landlocked, era),
            staple_plants: random_crops(&dict, era),
            adult_age: 18,
            species_avg_lifespan: 70,
            species_avg_lifespan_variance: 5,
            parental_naming_formats: naming_systems
                .iter()
                .take(naming_system_count)
                .map(|c| c.clone())
                .collect(),
            avg_building_footprint,
            avg_building_floors,
        };
    }

    pub fn build_culture_dictionary(dict: &Vec<Word>, culture: &CultureConfig) -> Vec<Word> {
        let mut output = dict.clone();
        for (first_name, last_name) in culture.historical_figures.clone() {
            output.push(Word {
                id: Uuid::new_v4(),
                word_type: WordType::Noun,
                text: first_name.clone(),
                tags: vec![
                    NounTag::FirstName.to_string(),
                    NounTag::HistoricalFigure.to_string(),
                ],
                related_forms: vec![],
            });
            output.push(Word {
                id: Uuid::new_v4(),
                word_type: WordType::Noun,
                text: last_name.clone(),
                tags: vec![
                    NounTag::LastName.to_string(),
                    NounTag::HistoricalFigure.to_string(),
                ],
                related_forms: vec![],
            });
            // println!(
            //     "{:?}",
            //     Word {
            //         id: Uuid::new_v4(),
            //         word_type: WordType::Noun,
            //         text: first_name.clone(),
            //         tags: vec![
            //             NounTag::Title.to_string(),
            //             NounTag::HistoricalFigure.to_string(),
            //         ],
            //         related_forms: vec![],
            //     }
            // );
            // println!(
            //     "{:?}",
            //     Word {
            //         id: Uuid::new_v4(),
            //         word_type: WordType::Noun,
            //         text: last_name.clone(),
            //         tags: vec![
            //             NounTag::LastName.to_string(),
            //             NounTag::HistoricalFigure.to_string(),
            //         ],
            //         related_forms: vec![],
            //     }
            // );
        }
        return output;
    }

    #[test]
    fn test_random_culture() {
        let dict = build_dictionary();
        println!("{:#?}", random_culture(&dict, &None));
    }
}
