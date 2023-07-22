pub mod appearance {
    use crate::language::{
        adjectives::adjectives::AdjectiveTag,
        language::{build_dictionary, random_word_by_tag, Word, WordType},
    };
    use rand::Rng;

    #[derive(PartialEq, Debug, Clone)]
    pub struct PhysicalDescription {
        pub hair_colour: String,
        pub hair_length: String,
        pub hair_adjectives: Vec<String>,
        pub eye_colour: String,
        pub height_adjective: String,
        pub build_adjective: String,
    }

    fn random_descriptor(dict: &Vec<Word>, target: String) -> Word {
        return random_word_by_tag(
            &dict,
            WordType::Adjective,
            &vec![target],
            &vec![],
            &vec![],
            &None,
        )
        .unwrap();
    }

    pub fn random_mind_description(dict: &Vec<Word>) -> PhysicalDescription {
        return PhysicalDescription {
            hair_colour: random_descriptor(&dict, AdjectiveTag::HairColour.to_string()).text,
            hair_length: random_descriptor(&dict, AdjectiveTag::HairLength.to_string()).text,
            hair_adjectives: vec![
                random_descriptor(&dict, AdjectiveTag::HairState.to_string()).text,
            ],
            eye_colour: random_descriptor(&dict, AdjectiveTag::Colour.to_string()).text,
            height_adjective: random_descriptor(&dict, AdjectiveTag::CreatureHeight.to_string())
                .text,
            build_adjective: random_descriptor(&dict, AdjectiveTag::CreatureBuild.to_string()).text,
        };
    }

    fn choose_or_mutate_attribute(
        dict: &Vec<Word>,
        tag: String,
        a1: &String,
        a2: &String,
    ) -> String {
        let mut rng = rand::thread_rng();
        let roll = rng.gen::<f32>();
        if roll > 0.55 {
            return a1.clone();
        } else if roll > 0.1 {
            return a2.clone();
        } else {
            return random_descriptor(&dict, tag).text;
        }
    }

    pub fn generate_child_description(
        dict: &Vec<Word>,
        p1: &PhysicalDescription,
        p2: &PhysicalDescription,
    ) -> PhysicalDescription {
        return PhysicalDescription {
            hair_colour: choose_or_mutate_attribute(
                &dict,
                AdjectiveTag::HairColour.to_string(),
                &p1.hair_colour,
                &p2.hair_colour,
            ),
            hair_length: choose_or_mutate_attribute(
                &dict,
                AdjectiveTag::HairLength.to_string(),
                &p1.hair_length,
                &p2.hair_length,
            ),
            hair_adjectives: vec![choose_or_mutate_attribute(
                &dict,
                AdjectiveTag::HairState.to_string(),
                &p1.hair_adjectives.first().unwrap(),
                &p2.hair_adjectives.first().unwrap(),
            )],
            eye_colour: choose_or_mutate_attribute(
                &dict,
                AdjectiveTag::Colour.to_string(),
                &p1.eye_colour,
                &p2.eye_colour,
            ),
            height_adjective: choose_or_mutate_attribute(
                &dict,
                AdjectiveTag::CreatureHeight.to_string(),
                &p1.height_adjective,
                &p2.height_adjective,
            ),
            build_adjective: choose_or_mutate_attribute(
                &dict,
                AdjectiveTag::CreatureBuild.to_string(),
                &p1.build_adjective,
                &p2.build_adjective,
            ),
        };
    }

    #[test]
    fn random_description_test() {
        let dict = build_dictionary();
        for _i in 0..10 {
            println!("{:#?}", random_mind_description(&dict));
        }
    }
}
