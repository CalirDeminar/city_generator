pub mod relations;
pub mod mind {
    use html_builder::*;
    use std::fmt::Write as fmtWrite;

    use rand::Rng;
    use rand_distr::{Distribution, Normal};
    use strum_macros::Display;
    use uuid::Uuid;

    use crate::city::building::building::{Building, BuildingFloorArea};
    use crate::city::city::City;
    use crate::city::institutions::institutions::*;
    use crate::city::locations::locations::Location;
    use crate::city::population::population::Population;
    use crate::language::language::{random_word_by_tag_and, Era, Word, WordType};

    use crate::city::population::mind::relations::relations::*;

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
    }

    pub fn find_address<'a>(
        mind: &Mind,
        city: &'a City,
    ) -> (&'a Building, &'a BuildingFloorArea, &'a Location) {
        let city_floor_areas: Vec<&BuildingFloorArea> = city
            .buildings
            .iter()
            .flat_map(|b| b.floors.iter().flat_map(|f| f.areas.iter()))
            .collect();
        let area = city_floor_areas
            .iter()
            .find(|a| a.id.eq(&mind.residence.unwrap()))
            .unwrap();
        let building = city
            .buildings
            .iter()
            .find(|b| {
                b.floors
                    .iter()
                    .any(|f| f.areas.iter().any(|a| a.id.eq(&area.id)))
            })
            .unwrap();
        let location = city
            .areas
            .iter()
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
        return city
            .institutions
            .iter()
            .find(|i| mind.employer.is_some() && mind.employer.unwrap().eq(&i.id));
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
        output.push_str(&format!("===========\n"));
        return output;
    }

    pub fn print_mind_html<'a>(
        node: &'a mut Node<'a>,
        mind: &Mind,
        city: &City,
    ) -> &'a mut Node<'a> {
        let workplace = city
            .institutions
            .iter()
            .find(|i| mind.employer.is_some() && mind.employer.unwrap().eq(&i.id));

        let mut relations: Vec<(&RelationVerb, String, Uuid)> = mind
            .relations
            .iter()
            .map(|(verb, id)| (verb, get_name_from_id(&id, &city.citizens), id.clone()))
            .collect();
        relations.sort_by_key(|a| a.0.to_string());

        let mut list_element = node.div().attr(&format!("id='{}'", mind.id));
        writeln!(
            list_element.h3(),
            "Name: {} {}",
            &mind.first_name,
            &mind.last_name
        )
        .unwrap();
        writeln!(list_element.p(), "Gender: {}", &mind.gender).unwrap();
        writeln!(list_element.p(), "Age: {}", &mind.age).unwrap();

        if workplace.is_some() {
            let (building, _floor, _area, location) =
                find_institution_address(&workplace.unwrap(), &city);
            let mut p = list_element.p();
            writeln!(p, "Employer: {} at", workplace.unwrap().name).unwrap();
            writeln!(
                p.a().attr(&format!("href='#{}'", building.id)),
                "{}",
                building.name
            )
            .unwrap();
            writeln!(p, " in ").unwrap();
            writeln!(
                p.a().attr(&format!("href='#{}'", location.id)),
                "{}",
                location.name
            )
            .unwrap();
        } else {
            writeln!(list_element.p(), "Employer: None").unwrap();
        }
        if mind.residence.is_some() {
            let (building, apartment, residential_location) = find_address(mind, city);
            let mut line = list_element.p();
            writeln!(line, "Lives at: ").unwrap();
            writeln!(
                line.a().attr(&format!("href='#{}'", apartment.id)),
                "{}",
                apartment.name
            )
            .unwrap();
            writeln!(line, " - ").unwrap();
            writeln!(
                line.a().attr(&format!("href='#{}'", building.id)),
                "{}",
                building.name
            )
            .unwrap();
            writeln!(line, " - ").unwrap();
            writeln!(
                line.a()
                    .attr(&format!("href='#{}'", residential_location.id)),
                "{}",
                residential_location.name
            )
            .unwrap();
        }

        if relations.len() < 1 {
            writeln!(list_element.p(), "Relations: None").unwrap();
        } else {
            writeln!(list_element.p(), "Relations:").unwrap();
            let mut relation_list = list_element.ul();
            for (verb, name, id) in relations {
                let mut list_el = relation_list.li();
                let mut list_el_para = list_el.p();
                writeln!(list_el_para, "{:?}:", verb).unwrap();
                writeln!(
                    list_el_para.a().attr(&format!("href='#{}'", id)),
                    "{}",
                    name
                )
                .unwrap();
            }
        }

        return node;
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
        };
    }
}
