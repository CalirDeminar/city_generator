pub mod appearance;
pub mod relations;
pub mod mind {

    use std::collections::HashMap;

    use rand::Rng;
    use rand_distr::{Distribution, Normal};
    use strum_macros::Display;
    use uuid::Uuid;

    use crate::city::building::building::{Building, BuildingFloorArea};
    use crate::city::city::City;
    use crate::city::institutions::institutions::*;
    use crate::city::institutions::visits::visits::get_habitual_institutions;
    use crate::city::locations::locations::Location;
    use crate::city::population::population::Population;
    use crate::language::language::{random_word_by_tag_and, Era, Word, WordType};

    use crate::city::population::mind::relations::relations::*;
    use crate::language2::language2::Dictionary;
    use crate::language2::names::names::name;

    use super::appearance::appearance::{
        empty_description, random_mind_description, PhysicalDescription,
    };

    const HOMOSEXUALITY_CHANCE: f32 = 0.2;

    #[derive(PartialEq, Debug, Clone, Display)]
    pub enum Gender {
        Male,
        Female,
        Ambiguous,
    }

    // #[derive(PartialEq, Debug, Clone)]
    // pub struct Relation<'a> {
    //     relation_type: RelationVerb,
    //     entity: &'a Mind<'a>
    // }

    pub type Relation = (RelationVerb, Uuid);

    #[derive(PartialEq, Debug, Clone)]
    pub enum Sexuality {
        Hetrosexual,
        Homosexual,
        Asexual,
        Bisexual,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Mind {
        pub id: Uuid,
        pub first_name: String,
        pub last_name: String,
        pub gender: Gender,
        pub age: u32,
        pub relations: Vec<Relation>,
        pub employer: Option<Uuid>,
        pub residence: Option<Uuid>,
        pub sexuality: Sexuality,
        pub alive: bool,
        pub activity_log: Vec<String>,
        pub physical_description: PhysicalDescription,
        pub institution_shopping_visits: HashMap<Uuid, usize>,
        pub institution_social_visits: HashMap<Uuid, usize>,
    }

    pub fn find_address<'a>(
        mind: &Mind,
        city: &'a City,
    ) -> (&'a Building, &'a BuildingFloorArea, &'a Location) {
        let city_floor_areas: Vec<&BuildingFloorArea> = city
            .buildings
            .values()
            .flat_map(|b| b.floors.iter().flat_map(|f| f.areas.iter()))
            .collect();
        let area = city_floor_areas
            .iter()
            .find(|a| a.id.eq(&mind.residence.unwrap()))
            .unwrap();
        let building = city
            .buildings
            .values()
            .find(|b| {
                b.floors
                    .iter()
                    .any(|f| f.areas.iter().any(|a| a.id.eq(&area.id)))
            })
            .unwrap();
        let location = city
            .areas
            .values()
            .find(|a| a.id.eq(&building.location_id.unwrap()))
            .unwrap();
        return (building, area, location);
    }

    pub fn get_name_from_id(id: &Uuid, population: &Population) -> String {
        let result = population.get(id);
        if result.is_some() {
            return format!(
                "{} {} {}",
                String::from(&result.unwrap().first_name),
                String::from(&result.unwrap().last_name),
                if result.unwrap().alive {
                    ""
                } else {
                    "  (Dead)"
                }
            );
        }
        return format!("Missing ID: {}", id);
    }

    pub fn find_employer<'a>(mind: &Mind, city: &'a City) -> Option<&'a Institution> {
        return if mind.employer.is_some() {
            city.institutions.get(&mind.employer.unwrap())
        } else {
            None
        };
    }

    pub fn print_mind(mind: &Mind, city: &City) -> String {
        let mut output = String::from("");
        output.push_str("====Mind===\n");
        let workplace = find_employer(&mind, &city);
        // let workplace_location = city.areas.iter().find(|a| workplace.is_some() && workplace.unwrap().location_id.eq(&a.id));
        let mut relations: Vec<(&RelationVerb, String)> = mind
            .relations
            .iter()
            .map(|(verb, id)| (verb, get_name_from_id(&id, &city.citizens)))
            .collect();
        relations.sort_by_key(|a| a.0.to_string());
        output.push_str(&format!("Name: {} {}\n", mind.first_name, mind.last_name));
        output.push_str(&format!("Gender: {:?}\n", mind.gender));
        output.push_str(&format!("Age: {}\n", mind.age));
        let description = &mind.physical_description;
        output.push_str(&format!(
            "Description: They have {}, {} {} hair and {} eyes. They are {} with a {} build.\n",
            description.hair_adjectives.first().unwrap(),
            description.hair_colour,
            description.hair_length,
            description.eye_colour,
            description.height_adjective,
            description.build_adjective
        ));
        if workplace.is_some() {
            let (building, _floor, area, workplace_location) =
                find_institution_address(&workplace.unwrap(), &city);
            output.push_str(&format!(
                "Employer: {} at {} {} in {}\n",
                workplace.unwrap().name,
                area.name,
                building.name,
                workplace_location.name
            ));
            if mind.residence.is_some() {
                let (building, apartment, residential_location) = find_address(mind, city);
                output.push_str(&format!(
                    "Lives at: {} {} in {}\n",
                    apartment.name, building.name, residential_location.name
                ));
            }
        } else {
            output.push_str("Employer: None\n");
        }
        output.push_str(&format!("Relations:\n"));
        if relations.len() < 1 {
            output.push_str(&format!("  None\n"));
        } else {
            for (verb, name) in relations {
                output.push_str(&format!("  {:?}: {}\n", verb, name));
            }
        }
        let (habitual_inst_ids, habit_scale) = get_habitual_institutions(
            mind,
            &crate::city::institutions::visits::visits::VisitType::Shopping,
        );
        if habitual_inst_ids.len() > 0 {
            output.push_str(&format!(
                "Habitual Shopping Institutions ({}):\n",
                habit_scale
            ));
        }

        for id in habitual_inst_ids {
            let inst = city.institutions.get(id).unwrap();
            output.push_str(&format!("  {}\n", inst.name));
        }
        output.push_str(&format!("===========\n"));

        let (habitual_inst_ids, habit_scale) = get_habitual_institutions(
            mind,
            &crate::city::institutions::visits::visits::VisitType::Social,
        );
        if habitual_inst_ids.len() > 0 {
            output.push_str(&format!(
                "Habitual Social Institutions ({}):\n",
                habit_scale
            ));
        }

        for id in habitual_inst_ids {
            let inst = city.institutions.get(id).unwrap();
            output.push_str(&format!("  {}\n", inst.name));
        }
        output.push_str(&format!("===========\n"));
        return output;
    }

    fn gen_sexuality() -> Sexuality {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        if roll < HOMOSEXUALITY_CHANCE {
            return Sexuality::Homosexual;
        } else {
            return Sexuality::Hetrosexual;
        }
    }

    pub fn random_char<'a>(dict: &Vec<Word>, era: &Option<Era>, gen_last_name: bool) -> Mind {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let mut gender = Gender::Ambiguous;
        if roll > 0.6 {
            gender = Gender::Male;
        } else if roll > 0.2 {
            gender = Gender::Female;
        }
        let mut first_name_tags = if gender.eq(&Gender::Ambiguous) {
            vec![String::from("FirstName")]
        } else {
            vec![String::from("FirstName"), format!("Gender{}", gender)]
        };
        if era.is_some() {
            first_name_tags.push(era.unwrap().to_string());
        }
        let first_name = random_word_by_tag_and(&dict, WordType::Noun, first_name_tags)
            .unwrap()
            .text
            .clone();

        let last_name = if gen_last_name {
            random_word_by_tag_and(&dict, WordType::Noun, vec![String::from("LastName")])
                .unwrap()
                .text
                .clone()
        } else {
            String::new()
        };
        let distribution = Normal::new(5.0, 10.0).unwrap();
        return Mind {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            gender,
            relations: Vec::new(),
            age: (rng.gen::<f32>() * 40.0) as u32
                + 15
                + distribution.sample(&mut rand::thread_rng()) as u32,
            employer: None,
            residence: None,
            sexuality: gen_sexuality(),
            alive: true,
            activity_log: Vec::new(),
            physical_description: random_mind_description(&dict),
            institution_shopping_visits: HashMap::new(),
            institution_social_visits: HashMap::new(),
        };
    }

    pub fn random_char2<'a>(dict: &Dictionary, era: &Option<Era>) -> Mind {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let mut gender = Gender::Ambiguous;
        if roll > 0.6 {
            gender = Gender::Male;
        } else if roll > 0.2 {
            gender = Gender::Female;
        }
        let era_string = if era.is_some() {
            Some(era.unwrap().to_string())
        } else {
            None
        };
        let (first_name, last_name) = name(&dict, &gender, era_string);

        let distribution = Normal::new(5.0, 10.0).unwrap();
        return Mind {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            gender,
            relations: Vec::new(),
            age: (rng.gen::<f32>() * 40.0) as u32
                + 15
                + distribution.sample(&mut rand::thread_rng()) as u32,
            employer: None,
            residence: None,
            sexuality: gen_sexuality(),
            alive: true,
            activity_log: Vec::new(),
            physical_description: empty_description(),
            institution_shopping_visits: HashMap::new(),
            institution_social_visits: HashMap::new(),
        };
    }

    pub fn add_new_relation_to_mind_log<'a>(
        mind: &'a mut Mind,
        year: usize,
        verb: RelationVerb,
        relation: &Mind,
    ) -> &'a mut Mind {
        mind.activity_log.push(format!(
            "Year: {} - {} {} aged {} gained {} {} as a {}",
            year,
            mind.first_name,
            mind.last_name,
            mind.age,
            relation.first_name,
            relation.last_name,
            verb
        ));
        return mind;
    }

    pub fn add_romatic_event_to_mind_log<'a>(
        mind: &'a mut Mind,
        year: usize,
        verb: RelationVerb,
        relation: &Mind,
        action: &str,
    ) -> &'a mut Mind {
        mind.activity_log.push(format!(
            "Year: {} - {} {} aged {} {} {} {} {}",
            year,
            mind.first_name,
            mind.last_name,
            mind.age,
            action,
            verb,
            relation.first_name,
            relation.last_name
        ));
        return mind;
    }

    pub fn add_residence_to_mind_log<'a>(
        mind: &'a mut Mind,
        year: usize,
        area_name: &str,
        building_name: &str,
        location_name: &str,
    ) -> &'a mut Mind {
        mind.activity_log.push(format!(
            "Year: {} - {} {} aged {} moved into {} {} in {}",
            year,
            mind.first_name,
            mind.last_name,
            mind.age,
            area_name,
            building_name,
            location_name
        ));
        return mind;
    }

    pub fn add_birth_to_mind_log<'a>(
        mind: &'a mut Mind,
        year: usize,
        parent_1: &Mind,
        parent_2: &Mind,
    ) -> &'a mut Mind {
        mind.activity_log.push(format!(
            "Year: {} - {} {} born to {} {} and {} {}",
            year,
            mind.first_name,
            mind.last_name,
            parent_1.first_name,
            parent_1.last_name,
            parent_2.first_name,
            parent_2.last_name
        ));
        return mind;
    }

    pub fn add_new_workplace_to_mind_log<'a>(
        mind: &'a mut Mind,
        year: usize,
        company_name: &str,
    ) -> &'a mut Mind {
        mind.activity_log.push(format!(
            "Year: {} - {} {} aged {} started work at {}",
            year, mind.first_name, mind.last_name, mind.age, company_name
        ));
        return mind;
    }

    pub fn add_leaving_workplace_to_mind_log<'a>(
        mind: &'a mut Mind,
        year: usize,
        company_name: &str,
    ) -> &'a mut Mind {
        mind.activity_log.push(format!(
            "Year: {} - {} {} aged {} left company {}",
            year, mind.first_name, mind.last_name, mind.age, company_name
        ));
        return mind;
    }

    pub fn add_startup_creation_to_mind_log<'a>(
        mind: &'a mut Mind,
        year: usize,
        company_name: &str,
    ) -> &'a mut Mind {
        mind.activity_log.push(format!(
            "Year: {} - {} {} aged {} created the company {}",
            year, mind.first_name, mind.last_name, mind.age, company_name
        ));
        return mind;
    }
}
