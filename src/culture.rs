pub mod culture {
    use crate::language::{
        language::{
            build_dictionary, random_word_by_tag, random_word_by_tag_and, Word, WordTag, WordType,
        },
        nouns::{
            creatures::creatures::{CreatureCategory, CreatureFamily},
            plants::plants::PlantType,
        },
    };
    use rand::Rng;

    #[derive(PartialEq, Debug, Clone)]
    pub struct CultureConfig {
        historical_figures: Vec<(String, String)>,
        landlocked: bool,
        stapleMeats: Vec<String>,
        staplePlants: Vec<String>,
    }

    fn gen_historical_figures(dict: &Vec<Word>) -> Vec<(String, String)> {
        let mut rng = rand::thread_rng();
        let figure_count = (rng.gen::<f32>() * 8.0) as usize;

        let mut output: Vec<(String, String)> = Vec::new();
        for _i in 0..figure_count.max(3) {
            let title = random_word_by_tag(
                &dict,
                WordType::Noun,
                &vec![WordTag::Noun(String::from("Title"))],
                &vec![],
            )
            .unwrap()
            .text;
            let name = random_word_by_tag(
                &dict,
                WordType::Noun,
                &vec![WordTag::Noun(String::from("LastName"))],
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
        let mut animalTypes = vec![WordTag::Noun(
            CreatureFamily::CreatureFamilyMammal.to_string(),
        )];
        if !landlocked {
            animalTypes.push(WordTag::Noun(
                CreatureFamily::CreatureFamilyFish.to_string(),
            ));
        }
        for _i in 0..len.max(2) {
            output.push(
                random_word_by_tag(
                    &dict,
                    WordType::Noun,
                    &vec![WordTag::Noun(CreatureCategory::CreatureAnimal.to_string())],
                    &animalTypes,
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
                    &vec![WordTag::Noun(PlantType::PlantTypeCrop.to_string())],
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
            stapleMeats: random_animals(&dict, landlocked),
            staplePlants: random_crops(&dict),
        };
    }

    #[test]
    fn test_random_culture() {
        let dict = build_dictionary();
        println!("{:#?}", random_culture(&dict));
    }
}
