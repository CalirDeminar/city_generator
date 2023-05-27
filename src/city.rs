pub mod institutions;
pub mod population;
pub mod locations;
pub mod city {
    use std::{fs::File, io::Write};

    use rand::Rng;
    use rand::seq::SliceRandom;
    use crate::city::population::population::*;
    use crate::city::institutions::institutions::*;
    use crate::city::locations::{locations, locations::*};
    use crate::names::names::*;
    use super::population::mind::relations::relations::link_colleagues;
    // use crate::city::population::mind::relations::relations::*;

    #[derive(PartialEq, Debug, Clone)]
    pub struct City {
        pub name: String,
        pub citizens: Population,
        pub institutions: Vec<Institution>,
        pub areas: Vec<Location>
        // buildings
        // areas
    }

    pub fn print_city(city: &City) -> String {
        let mut output: String = String::new();
        output.push_str(&format!("City Name: {}\n", city.name));
        for a in &city.areas {
            output.push_str(&print_location(&a, &city));
        }
        output.push_str(&print_population(&city));
        return output;
    }

    pub fn export_city(city: &City) {
        let mut file = File::create("./export.txt").unwrap();
        let output = print_city(&city);
        file.write_all(output.into_bytes().as_slice()).unwrap();
    }

    pub fn build(size: usize) -> City {
        let name_dict = gen_name_dict();
        let mut citizens = generate_population(&name_dict, size);
        let institutions: Vec<Institution>;
        (citizens, institutions) = assign_workplaces(&name_dict,  citizens);
        citizens = link_colleagues(citizens);
        let output = City {
            citizens,
            areas: gen_locations_from_institutions(&name_dict, &institutions),
            institutions: institutions,
            name: locations::gen_location_name(&name_dict, false),
        };
        return output;
    }

    fn assign_workplaces(name_dict: &NameDictionary, population: Population) -> (Population, Vec<Institution>) {
        let mut public_institutions = generate_public_institutions(name_dict);
        let mut output_institutions: Vec<Institution> = Vec::new();
        let mut output_minds: Population = Vec::new();

        let mut p = population;


        let mut rng = rand::thread_rng();
        let mut inst = public_institutions.pop().unwrap();
        output_institutions.push(inst.clone());
        let mut remaining_employees = ((rng.gen::<f32>() * 10.0) as usize).max(1);

        p.shuffle(&mut rng);
        for m in p {
            // println!("Remaining Employees: {:?}", remaining_employees);
            if remaining_employees < 1 {
                inst = if public_institutions.len() > 0 {public_institutions.pop().unwrap()} else {generate_population_institution(&name_dict)};
                output_institutions.push(inst.clone());
                remaining_employees = ((rng.gen::<f32>() * 10.0) as usize).max(1);
            }
            let mut mind = m.clone();
            if mind.age < 60 {
                mind.employer = Some(inst.id.clone());
                remaining_employees -= 1;
            }
            // println!("{:#?}", mind);
            output_minds.push(mind);

        }

        return (output_minds, output_institutions);
    }
}