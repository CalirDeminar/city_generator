pub mod building;
pub mod institutions;
pub mod locations;
pub mod population;
pub mod city {
    use std::fs::File;
    use std::io::Write;

    use html_builder::*;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::fmt::Write as fmtWrite;

    use super::building::building::{new_building, Building};
    use super::population::mind::mind::*;
    use super::population::mind::relations::relations::link_colleagues;
    use crate::city::institutions::institutions::*;
    use crate::city::locations::{locations, locations::*};
    use crate::city::population::population::*;
    use crate::names::names::*;
    // use crate::city::population::mind::relations::relations::*;

    const MAX_WORKING_AGE: u32 = 60;

    #[derive(PartialEq, Debug, Clone)]
    pub struct City {
        pub name: String,
        pub citizens: Population,
        pub institutions: Vec<Institution>,
        pub areas: Vec<Location>,
        pub buildings: Vec<Building>, // buildings
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

    pub fn export_city_html(city: &City) {
        let mut document = Buffer::new();
        document.doctype();
        let mut html = document.html().attr("lang='en'");
        writeln!(html.head().title(), "City Name: {}", &city.name).unwrap();
        html.link().attr("rel='stylesheet' href='./style.css'");
        let mut body = html.body();
        writeln!(body.h1(), "{}", city.name).unwrap();
        writeln!(body.h2(), "Locations:").unwrap();
        let mut loc_list = body.ul();
        for area in &city.areas {
            print_location_html(&mut loc_list.li(), &area, &city);
        }

        writeln!(body.h2(), "Citizens").unwrap();
        let mut citizen_list = body.ul();
        for m in &city.citizens {
            print_mind_html(&mut citizen_list.li(), &m, &city);
        }

        let mut file = File::create("./export.html").unwrap();
        file.write_all(document.finish().into_bytes().as_slice())
            .unwrap();
    }

    fn find_free_building<'a>(city: &'a mut City) -> Option<&'a mut Building> {
        return city.buildings.iter_mut().find(|b| {
            b.floors.iter().any(|f| {
                f.floor_type
                    .eq(&super::building::building::FloorType::Commercial)
                    && f.areas
                        .iter()
                        .any(|a| a.owning_citizen.is_none() && a.owning_institution.is_none())
            })
        });
    }

    fn find_free_area<'a>(city: &'a mut City) -> Option<&'a mut Location> {
        return city.areas.iter_mut().find(|a| {
            city.buildings
                .iter_mut()
                .filter(|b| b.location_id.is_some() && b.location_id.unwrap().eq(&a.id))
                .count()
                < a.size
        });
    }

    fn add_institution_to_building<'a>(
        building: &'a mut Building,
        institution: &Institution,
    ) -> &'a mut Building {
        let free_area = building
            .floors
            .iter_mut()
            .filter(|f| {
                f.floor_type
                    .eq(&super::building::building::FloorType::Commercial)
            })
            .flat_map(|f| &mut f.areas)
            .find(|a| a.owning_citizen.is_none() && a.owning_institution.is_none());
        free_area.unwrap().owning_institution = Some(institution.id.clone());
        return building;
    }

    fn add_building_to_city<'a>(city: &'a mut City, name_dict: &NameDictionary) -> &'a mut City {
        let mut free_location = find_free_area(city);
        if free_location.is_none() {
            city.areas.push(gen_location(&name_dict));
            free_location = find_free_area(city);
        }
        let new_building = new_building(&name_dict, Some(free_location.unwrap().id.clone()));
        city.buildings.push(new_building);
        return city;
    }

    fn add_institution_to_city<'a>(
        city: &'a mut City,
        institution: Institution,
        name_dict: &NameDictionary,
    ) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let employee_count = ((rng.gen::<f32>() * 10.0) as i32).max(1);
        let all_workers = find_workers(&city);
        let workers = all_workers.iter().take(employee_count as usize);
        let mut building_with_space = find_free_building(city);

        if building_with_space.is_none() {
            add_building_to_city(city, &name_dict);
            building_with_space = find_free_building(city);
        }

        add_institution_to_building(building_with_space.unwrap(), &institution.clone());

        for w in workers {
            let mut worker = city.citizens.iter_mut().find(|m| m.id.eq(&w.id)).unwrap();
            worker.employer = Some(institution.id.clone());
        }
        city.institutions.push(institution);
        return city;
    }

    fn find_workers<'a>(city: &'a City) -> Population {
        let mut output: Population = Vec::new();
        for mind in &city.citizens {
            if mind.age < MAX_WORKING_AGE && mind.employer.is_none() {
                output.push(mind.clone());
            }
        }
        output.shuffle(&mut rand::thread_rng());
        return output;
    }

    pub fn build(size: usize) -> City {
        let name_dict = gen_name_dict();
        let citizens = generate_population(&name_dict, size);
        let mut city = City {
            name: locations::gen_location_name(&name_dict, false),
            buildings: Vec::new(),
            citizens,
            areas: Vec::new(),
            institutions: Vec::new(),
        };

        let mut public_institutions = generate_public_institutions(&name_dict);
        let mut workers = find_workers(&city);
        while workers.len() > 0 {
            let next_public = public_institutions.pop();
            let institution = if next_public.is_some() {
                next_public.unwrap()
            } else {
                generate_population_institution(&name_dict)
            };
            add_institution_to_city(&mut city, institution, &name_dict);
            workers = find_workers(&city);
        }
        city.citizens = link_colleagues(city.citizens);

        return city;
    }
}
