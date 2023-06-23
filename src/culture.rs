pub mod culture {
    use crate::language::{
        language::*,
        nouns::{
            creatures::creatures::{CreatureCategory, CreatureFamily},
            nouns::NounTag,
            plants::plants::PlantType,
        },
    };
    use rand::Rng;
    use uuid::Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub struct CultureConfig {
        historical_figures: Vec<(String, String)>,
        landlocked: bool,
        staple_meats: Vec<String>,
        staple_plants: Vec<String>,
    }

    fn gen_historical_figures(dict: &Vec<Word>) -> Vec<(String, String)> {
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
            )
            .unwrap()
            .text;
            let name = random_word_by_tag(
                &dict,
                WordType::Noun,
                &vec![String::from("LastName")],
                &vec![],
                &vec![],
            )
            .unwrap()
            .text;
            output.push((title, name));
            true;
        }
        return output;
    }

    fn random_animals(dict: &Vec<Word>, landlocked: bool) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let len = (rng.gen::<f32>() * 5.0) as usize;
        let mut output: Vec<String> = Vec::new();
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
                )
                .unwrap()
                .text,
            );
        }

        return output;
    }

    fn random_crops(dict: &Vec<Word>) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let len = (rng.gen::<f32>() * 7.0) as usize;
        let mut output: Vec<String> = Vec::new();
        for _i in 0..len.max(3) {
            output.push(
                random_word_by_tag(
                    &dict,
                    WordType::Noun,
                    &vec![PlantType::PlantTypeCrop.to_string()],
                    &vec![],
                    &vec![],
                )
                .unwrap()
                .text,
            );
        }
        return output;
    }

    pub fn random_culture(dict: &Vec<Word>) -> CultureConfig {
        let mut rng = rand::thread_rng();
        let landlocked = rng.gen::<f32>() > 0.5;
        return CultureConfig {
            historical_figures: gen_historical_figures(&dict),
            landlocked,
            staple_meats: random_animals(&dict, landlocked),
            staple_plants: random_crops(&dict),
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
        }
        return output;
    }

    #[test]
    fn test_random_culture() {
        let dict = build_dictionary();
        println!("{:#?}", random_culture(&dict));
    }
}
