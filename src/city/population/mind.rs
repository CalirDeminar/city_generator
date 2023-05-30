pub mod relations;
pub mod mind {
    use html_builder::*;
    use std::fmt::Write as fmtWrite;

    use rand::Rng;
    use rand_distr::{Normal, Distribution};
    use strum_macros::Display;
    use uuid::Uuid;

    use crate::city::city::City;
    use crate::names::names::*;
    
    use crate::city::population::mind::relations::relations::*;


    #[derive(PartialEq, Debug, Clone, Display)]
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
        pub relations: Vec<Relation>,
        pub employer: Option<Uuid>
    }

    pub fn get_name_from_id(id: &Uuid, population: &Vec<Mind>) -> String {
        let result = population.iter().find(|m| m.id.eq(id));
        if result.is_some() {
            return format!("{} {}", String::from(&result.unwrap().first_name), String::from(&result.unwrap().last_name));
        }
        return format!("Missing ID: {}", id);
    }
    pub fn print_mind(mind: &Mind, city: &City) -> String {
        let mut output = String::from("");
        output.push_str("====Mind===\n");
        let workplace = city.institutions.iter().find(|i| mind.employer.is_some() && mind.employer.unwrap().eq(&i.id));
        let workplace_location = city.areas.iter().find(|a| workplace.is_some() && workplace.unwrap().location_id.eq(&a.id));
        let relations: Vec<(&RelationVerb, String)> = mind.relations.iter().map(|(verb, id)| (verb, get_name_from_id(&id, &city.citizens))).collect();
        output.push_str(&format!("Name: {} {}\n", mind.first_name, mind.last_name));
        output.push_str(&format!("Gender: {:?}\n", mind.gender));
        output.push_str(&format!("Age: {}\n", mind.age));
        if workplace.is_some() {
            output.push_str(&format!("Employer: {} in {}\n", workplace.unwrap().name, workplace_location.unwrap().name));
        }else{
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

    pub fn print_mind_html<'a>(node: &'a mut Node<'a>, mind: &Mind, city: &City) -> &'a mut Node<'a> {
        let workplace = city.institutions.iter().find(|i| mind.employer.is_some() && mind.employer.unwrap().eq(&i.id));
        let workplace_location = city.areas.iter().find(|a| workplace.is_some() && workplace.unwrap().location_id.eq(&a.id));

        let relations: Vec<(&RelationVerb, String, Uuid)> = mind.relations.iter().map(|(verb, id)| (verb, get_name_from_id(&id, &city.citizens), id.clone())).collect();

        let mut list_element = node.div().attr(&format!("id='{}'", mind.id));
        writeln!(list_element.h3(), "Name: {} {}", &mind.first_name, &mind.last_name).unwrap();
        writeln!(list_element.p(), "Gender: {}", &mind.gender).unwrap();
        writeln!(list_element.p(), "Age: {}", &mind.age).unwrap();

        if workplace.is_some() && workplace_location.is_some() {
            let mut p = list_element.p();
            writeln!(p, "Employer: {} in", workplace.unwrap().name).unwrap();
            writeln!(p.a().attr(&format!("href='#{}'", workplace_location.unwrap().id)), "{}", workplace_location.unwrap().name).unwrap();
        } else {
            writeln!(list_element.p(), "Employer: None").unwrap();
        }

        if relations.len() < 1 {
            writeln!(list_element.p(), "Relations: None").unwrap();
        } else {
            writeln!(list_element.p(), "Relations:").unwrap();
            let mut relation_list = list_element.ul();
            for (verb, name, id) in  relations {
                let mut list_el = relation_list.li();
                let mut list_el_para = list_el.p();
                writeln!(list_el_para, "{:?}:", verb).unwrap();
                writeln!(list_el_para.a().attr(&format!("href='#{}'", id)), "{}", name).unwrap();
            }
        }

        return node;
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
            age: (rng.gen::<f32>() * 40.0) as u32 + 15 + distribution.sample(&mut rand::thread_rng()) as u32,
            employer: None
        }
    }
}