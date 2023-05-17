// pub mod relations;
pub mod relations;
pub mod mind {
    use rand::Rng;
    use rand_distr::{Normal, Distribution};
    use uuid::Uuid;

    use crate::names::names::*;
    // use super::relations::relations::*;
    
    use crate::mind::relations::relations::*;


    #[derive(PartialEq, Debug, Clone)]
    pub enum Gender {
        Male,
        Female,
        Ambiguous
    }

    // #[derive(PartialEq, Debug, Clone)]
    // pub struct Relation<'a> {
    //     relation_type: RelationVerb,
    //     entity: &'a Mind<'a>
    // }

    pub type Relation = (RelationVerb, Uuid);

    #[derive(PartialEq, Debug, Clone)]
    pub struct Mind {
        pub id: Uuid,
        pub first_name: String,
        pub last_name: String,
        pub gender: Gender,
        pub age: u32,
        pub relations: Vec<Relation>
    }

    pub fn get_name_from_id(id: &Uuid, population: &Vec<Mind>) -> String {
        let result = population.iter().find(|m| m.id.eq(id));
        if result.is_some() {
            return format!("{} {}", String::from(&result.unwrap().first_name), String::from(&result.unwrap().last_name));
        }
        return format!("Missing ID: {}", id);
    }
    pub fn print_mind(mind: &Mind, population: &Vec<Mind>) -> String {
        let mut output = String::from("");
        output.push_str("===========\n");
        let relations: Vec<(&RelationVerb, String)> = mind.relations.iter().map(|(verb, id)| (verb, get_name_from_id(&id, &population))).collect();
        // println!("ID: {}", mind.id);
        output.push_str(&format!("Name: {} {}\n", mind.first_name, mind.last_name));
        output.push_str(&format!("Gender: {:?}\n", mind.gender));
        output.push_str(&format!("Age: {}\n", mind.age));
        output.push_str(&format!("Relations:\n"));
        if relations.len() < 1 {
            output.push_str(&format!("  None\n"));
        } else {
            for (verb, name) in relations {
                output.push_str(&format!("  {:?}: {}\n", verb, name));
            }
        }
        output.push_str(&format!("==========\n"));
        return output;
    }


    pub fn random_char<'a>(name_dict: &NameDictionary) -> Mind {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let mut gender = Gender::Ambiguous;
        if roll > 0.6 {
            gender = Gender::Male;
        }
        if roll > 0.2 {
            gender = Gender::Female;
        }
        let (first_name, last_name) = random_mind_name(&name_dict, &gender);
        let distribution = Normal::new(5.0, 10.0).unwrap();
        return Mind {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            gender,
            relations: Vec::new(),
            age: (rng.gen::<f32>() * 40.0) as u32 + 15 + distribution.sample(&mut rand::thread_rng()) as u32
        }
    }
}