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
    use uuid::Uuid;

    use super::building::building::*;
    use super::population::mind::mind::*;
    use super::population::mind::relations::relations::*;
    use crate::city::institutions::institutions::*;
    use crate::city::locations::{locations, locations::*};
    use crate::city::population::mind::relations::friends::friends::link_friends_within_population_by_year;
    use crate::city::population::mind::relations::parents::parents::generate_children_per_year;
    use crate::city::population::mind::relations::partners::partners::{
        link_partners_by_year, update_partners_by_year,
    };
    use crate::city::population::population::*;
    use crate::culture::culture::{build_culture_dictionary, random_culture, CultureConfig};
    use crate::language::language::{
        build_dictionary, random_word_by_tag, random_word_by_tag_and, Word, WordType,
    };

    const MAX_WORKING_AGE: u32 = 60;

    #[derive(PartialEq, Debug, Clone)]
    pub struct City {
        pub name: String,
        pub citizens: Population,
        pub institutions: Vec<Institution>,
        pub areas: Vec<Location>,
        pub buildings: Vec<Building>,
    }

    pub fn print_city(city: &City) -> String {
        let mut output: String = String::new();
        output.push_str(&format!("City Name: {}\n", city.name));
        output.push_str(&format!(
            "Population: {}",
            city.citizens.iter().filter(|c| c.alive).count()
        ));
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
        writeln!(body.p(), "Population: {}", city.citizens.len()).unwrap();
        writeln!(body.p(), "Area Count: {}", city.areas.len()).unwrap();
        writeln!(body.p(), "Building Count: {}", city.buildings.len()).unwrap();
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

    fn count_residential_apartments(city: &City) -> usize {
        let apartments: Vec<&BuildingFloorArea> = city
            .buildings
            .iter()
            .flat_map(|b| {
                b.floors
                    .iter()
                    .filter(|f| {
                        f.floor_type
                            .eq(&super::building::building::FloorType::Residential)
                    })
                    .flat_map(|f| f.areas.iter())
            })
            .collect();
        return apartments.len();
    }

    fn find_free_building<'a>(city: &'a mut City) -> Option<&'a mut Building> {
        return city.buildings.iter_mut().find(|b| {
            b.floors.iter().any(|f| {
                f.floor_type
                    .eq(&super::building::building::FloorType::Commercial)
                    && f.areas
                        .iter()
                        .any(|a| a.owning_institution.is_none() && a.owning_institution.is_none())
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
            .find(|a| a.owning_institution.is_none());
        free_area.unwrap().owning_institution = Some(institution.id.clone());
        return building;
    }

    fn add_building_to_city<'a>(city: &'a mut City, dict: &Vec<Word>) -> &'a mut City {
        let mut free_location = find_free_area(city);
        if free_location.is_none() {
            city.areas.push(gen_location(&dict));
            free_location = find_free_area(city);
        }
        let new_building = new_building(&dict, Some(free_location.unwrap().id.clone()));
        city.buildings.push(new_building);
        return city;
    }

    fn add_institution_to_city<'a>(
        city: &'a mut City,
        institution: Institution,
        dict: &Vec<Word>,
    ) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let employee_count = ((rng.gen::<f32>() * 10.0) as i32).max(1);
        let all_workers = find_workers(&city);
        let workers = all_workers.iter().take(employee_count as usize);
        let mut building_with_space = find_free_building(city);

        if building_with_space.is_none() {
            add_building_to_city(city, &dict);
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

    fn add_public_institution_to_city<'a>(
        city: &'a mut City,
        institution: Institution,
        dict: &Vec<Word>,
    ) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let employee_count = ((rng.gen::<f32>() * 10.0) as i32).max(1);
        let all_workers = find_workers(&city);
        let workers = all_workers.iter().take(employee_count as usize);

        add_building_to_city(city, &dict);
        add_institution_to_building(city.buildings.last_mut().unwrap(), &institution);

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

    fn assign_residences<'a>(city: &'a mut City) -> &'a mut City {
        let mut new_residences: Vec<(Uuid, Uuid)> = Vec::new();
        let mut owned_ids: Vec<Uuid> = city
            .citizens
            .iter()
            .filter(|c| c.residence.is_some())
            .map(|c| c.residence.unwrap().clone())
            .collect();
        for citizen in city.citizens.iter().filter(|c| c.residence.is_none()) {
            let guardian = if citizen.age < ADULT_AGE_FROM {
                find_relation(&citizen, RelationVerb::Parent, &city)
            } else {
                None
            };
            let guardian_res = if guardian.is_some() {
                new_residences
                    .iter()
                    .find(|(m_id, _r_id)| m_id.eq(&guardian.unwrap().id))
            } else {
                None
            };
            let ward = find_relation_minor(&citizen, RelationVerb::Child, &city);
            let ward_res: Option<&(Uuid, Uuid)> = if ward.is_some() {
                new_residences
                    .iter()
                    .find(|(m_id, _r_id)| m_id.eq(&ward.unwrap().id))
            } else {
                None
            };
            let spouse = find_relation(&citizen, RelationVerb::Spouse, &city);
            let spouse_res: Option<&(Uuid, Uuid)> = if spouse.is_some() {
                new_residences
                    .iter()
                    .find(|(m_id, _r_id)| m_id.eq(&spouse.unwrap().id))
            } else {
                None
            };
            // TODO - Currently broken, output looks very wrong
            if guardian_res.is_some() {
                new_residences.push((citizen.id.clone(), guardian_res.unwrap().clone().1));
            } else if ward_res.is_some() {
                new_residences.push((citizen.id.clone(), ward_res.unwrap().clone().1));
            } else if spouse_res.is_some() {
                new_residences.push((citizen.id.clone(), spouse_res.unwrap().clone().1));
            } else {
                let mut all_areas: Vec<&BuildingFloorArea> = city
                    .buildings
                    .iter()
                    .flat_map(|b| b.floors.iter().flat_map(|f| f.areas.iter()))
                    .collect();

                all_areas.shuffle(&mut rand::thread_rng());

                let apartment = all_areas
                    .iter()
                    .find(|a| a.owning_institution.is_none() && !owned_ids.contains(&a.id));
                if apartment.is_some() {
                    owned_ids.push(apartment.unwrap().id);
                    new_residences.push((citizen.id.clone(), apartment.unwrap().id.clone()));
                }
            }
        }
        for (citizen_id, residence_id) in new_residences {
            let citizen = city
                .citizens
                .iter_mut()
                .find(|c| c.id.eq(&citizen_id))
                .unwrap();
            citizen.residence = Some(residence_id.clone());
        }
        return city;
    }

    pub fn build(size: usize) -> City {
        let dict = build_dictionary();
        let culture = random_culture(&dict);

        println!("{:?}", culture);
        let language_dict = build_culture_dictionary(&dict, &culture);
        let mut city = City {
            name: locations::gen_location_name(&language_dict, false),
            buildings: Vec::new(),
            citizens: Vec::new(),
            areas: Vec::new(),
            institutions: Vec::new(),
        };

        generate_population_full_relation(&language_dict, size, &mut city);

        let public_institutions = generate_public_institutions(&language_dict);

        for pub_inst in public_institutions {
            add_public_institution_to_city(&mut city, pub_inst, &language_dict);
        }

        let mut workers = find_workers(&city);
        while workers.len() > 0 {
            let institution = generate_population_institution(&language_dict);
            add_institution_to_city(&mut city, institution, &language_dict);
            workers = find_workers(&city);
        }
        link_colleagues(&mut city);
        let mut apartment_count = count_residential_apartments(&city);
        while (apartment_count as f32) < (city.citizens.len() as f32 * 1.1) {
            city.areas.shuffle(&mut rand::thread_rng());
            city.buildings.push(new_building(
                &language_dict,
                Some(city.areas.first().unwrap().id.clone()),
            ));
            apartment_count = count_residential_apartments(&city);
        }
        assign_residences(&mut city);
        println!("{:#?}", city.institutions);
        return city;
    }

    fn count_city_relations_proportions(city: &City, verb: RelationVerb) -> f32 {
        return city
            .citizens
            .iter()
            .filter(|c| c.relations.iter().any(|(v, _id)| v.eq(&verb)))
            .count() as f32
            / city.citizens.len() as f32;
    }

    pub fn old_age_pass_per_year<'a>(city: &'a mut City, culture: &CultureConfig) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let citizen_ids: Vec<Uuid> = city
            .citizens
            .iter()
            .filter(|c| c.alive)
            .map(|c| c.id)
            .collect();
        let base_death_chance: f32 = 0.5;
        let mut dead_ids: Vec<Uuid> = vec![];
        for mind_id in citizen_ids {
            let mut citizens = city.citizens.iter_mut();
            let mind = citizens.find(|c| c.id.eq(&mind_id)).unwrap();
            let death_odds = base_death_chance
                + (((mind.age as f32 - culture.species_avg_lifespan_variance as f32)
                    - (culture.species_avg_lifespan as f32
                        - culture.species_avg_lifespan_variance as f32))
                    / 10.0);
            if rng.gen::<f32>() < death_odds {
                mind.alive = false;
                mind.employer = None;
                mind.residence = None;
                dead_ids.push(mind.id.clone());
            }
        }
        for mind in city.citizens.iter_mut() {
            for (verb, id) in mind.relations.clone() {
                if dead_ids.contains(&id) {
                    match verb {
                        RelationVerb::Partner => {
                            mind.relations.retain(|(_v, id)| !id.eq(&id));
                            mind.relations.push((RelationVerb::LatePartner, id.clone()));
                        }
                        RelationVerb::Spouse => {
                            mind.relations.retain(|(_v, id)| !id.eq(&id));
                            mind.relations.push((RelationVerb::LateSpouse, id.clone()));
                        }

                        _ => {
                            mind.relations.retain(|(_v, id)| !id.eq(&id));
                        }
                    }
                }
            }
        }
        return city;
    }

    pub fn simulate(size: usize, age: usize) -> City {
        let dict = build_dictionary();
        let culture = random_culture(&dict);

        println!("{:?}", culture);
        let dict = build_culture_dictionary(&dict, &culture);
        let mut city = City {
            name: locations::gen_location_name(&dict, false),
            buildings: Vec::new(),
            citizens: Vec::new(),
            areas: Vec::new(),
            institutions: Vec::new(),
        };

        generate_population_baseline(&dict, size, &mut city);
        for i in 0..age {
            println!("Y{}", i);
            old_age_pass_per_year(&mut city, &culture);
            link_friends_within_population_by_year(&mut city);
            link_partners_by_year(&mut city);
            update_partners_by_year(&mut city);
            generate_children_per_year(&mut city, &culture, &dict);
            // println!(
            //     "Year: {} - Partners: {} - ExPartners: {} - Spouse: {} - ExSpouse: {}",
            //     i,
            //     count_city_relations_proportions(&city, RelationVerb::Partner),
            //     count_city_relations_proportions(&city, RelationVerb::ExPartner),
            //     count_city_relations_proportions(&city, RelationVerb::Spouse),
            //     count_city_relations_proportions(&city, RelationVerb::ExSpouse)
            // );

            for citizen in city.citizens.iter_mut() {
                citizen.age += 1;
            }
        }

        return city;
    }

    #[test]
    fn test_simulation() {
        simulate(1000, 20);
    }
}
