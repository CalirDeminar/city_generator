pub mod building;
pub mod institutions;
pub mod locations;
pub mod population;
pub mod city {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use std::time::Instant;

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
    use crate::city::population::mind::relations::friends::friends::*;
    use crate::city::population::mind::relations::parents::parents::*;
    use crate::city::population::mind::relations::partners::partners::*;
    use crate::city::population::population::*;
    use crate::culture::culture::*;
    use crate::language::language::*;

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
        let alive_population = city.citizens.values().filter(|c| c.alive).count();
        let adult_population = city.citizens.values().filter(|c| c.alive && c.age > ADULT_AGE_FROM).count();
        output.push_str(&format!("City Name: {}\n", city.name));
        output.push_str(&format!(
            "Population: {}\n",
            alive_population
        ));
        output.push_str(
    &format!(
                "Employment Rate: {}\n", 
                city
                .citizens
                .values()
                .filter(|c|c.alive && c.employer.is_some()).count() as f32
                / adult_population as f32
                )
            );
        output.push_str(&format!(
            "Dead: {}\n",
            city.citizens.iter().filter(|(id, c)| !c.alive).count()
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
        let living = city.citizens.values().filter(|c| c.alive);
        let dead = city.citizens.values().filter(|c| !c.alive);
        let mut document = Buffer::new();
        document.doctype();
        let mut html = document.html().attr("lang='en'");
        writeln!(html.head().title(), "City Name: {}", &city.name).unwrap();
        html.link().attr("rel='stylesheet' href='./style.css'");
        let mut body = html.body();
        writeln!(body.h1(), "{}", city.name).unwrap();
        writeln!(body.p(), "Population: {}", living.clone().count()).unwrap();
        writeln!(body.p(), "Population: {}", dead.clone().count()).unwrap();
        writeln!(body.p(), "Area Count: {}", city.areas.len()).unwrap();
        writeln!(body.p(), "Building Count: {}", city.buildings.len()).unwrap();
        writeln!(body.h2(), "Locations:").unwrap();
        let mut loc_list = body.ul();
        for area in &city.areas {
            print_location_html(&mut loc_list.li(), &area, &city);
        }

        writeln!(body.h2(), "Citizens").unwrap();
        let mut citizen_list = body.ul();
        for m in living {
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

    pub fn add_institution_to_city<'a>(
        city: &'a mut City,
        institution: Institution,
        dict: &Vec<Word>,
    ) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let employee_count = ((rng.gen::<f32>() * 10.0) as i32).max(1);
        // let all_workers = find_workers(&city);
        // let workers = all_workers.iter().take(employee_count as usize);
        let mut building_with_space = find_free_building(city);

        if building_with_space.is_none() {
            add_building_to_city(city, &dict);
            building_with_space = find_free_building(city);
        }

        add_institution_to_building(building_with_space.unwrap(), &institution.clone());

        // for w in workers {
        //     let mut worker = city.citizens.values_mut().find(|m| m.id.eq(&w.id)).unwrap();
        //     worker.employer = Some(institution.id.clone());
        // }
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
            let mut worker = city.citizens.values_mut().find(|m| m.id.eq(&w.id)).unwrap();
            worker.employer = Some(institution.id.clone());
        }

        city.institutions.push(institution);

        return city;
    }

    fn find_workers<'a>(city: &'a City) -> Vec<Mind> {
        let mut output: Vec<Mind> = Vec::new();
        for mind in city.citizens.values() {
            if mind.age < MAX_WORKING_AGE && mind.employer.is_none() {
                output.push(mind.clone());
            }
        }
        output.shuffle(&mut rand::thread_rng());
        return output;
    }

    pub fn assign_residences<'a>(city: &'a mut City) -> &'a mut City {
        let mut new_residences: Vec<(Uuid, Uuid)> = Vec::new();
        let mut owned_ids: Vec<Uuid> = city
            .citizens
            .values()
            .filter(|c| c.residence.is_some())
            .map(|c| c.residence.unwrap().clone())
            .collect();
        let ref_pop = city.citizens.clone();
        for citizen in city.citizens.values_mut().filter(|c| c.alive && c.residence.is_none()) {
            let guardian = if citizen.age < ADULT_AGE_FROM {
                find_relation(&citizen, RelationVerb::Parent, &ref_pop)
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
            let ward = find_relation_minor(&citizen, RelationVerb::Child, &ref_pop);
            let ward_res: Option<&(Uuid, Uuid)> = if ward.is_some() {
                new_residences
                    .iter()
                    .find(|(m_id, _r_id)| m_id.eq(&ward.unwrap().id))
            } else {
                None
            };
            let spouse = find_relation(&citizen, RelationVerb::Spouse, &ref_pop);
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
            let citizen = city.citizens.get_mut(&citizen_id).unwrap();
            citizen.residence = Some(residence_id.clone());
        }
        return city;
    }

    fn count_city_relations_proportions(city: &City, verb: RelationVerb) -> f32 {
        return city
            .citizens
            .values()
            .filter(|c| c.relations.iter().any(|(v, _id)| v.eq(&verb)))
            .count() as f32
            / city.citizens.len() as f32;
    }

    pub fn old_age_pass_per_year<'a>(city: &'a mut City, culture: &CultureConfig) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let citizen_ids: Vec<Uuid> = city
            .citizens
            .values()
            .filter(|c| c.alive)
            .map(|c| c.id)
            .collect();
        let base_death_chance: f32 = 0.5;
        let mut dead_ids: Vec<Uuid> = vec![];
        for mind_id in citizen_ids {
            let mind = city.citizens.get_mut(&mind_id).unwrap();
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
        let social_verbs = vec![RelationVerb::Acquaintance, RelationVerb::Friend, RelationVerb::CloseFriend, RelationVerb::Colleague];
        for mind in city.citizens.values_mut() {
            for (verb, id) in mind.relations.clone() {
                if dead_ids.contains(&id) {
                    match verb {
                        RelationVerb::Partner => {
                            mind.relations.retain(|(v, id)| !(id.eq(&mind.id) && v.eq(&verb)));
                            mind.relations.push((RelationVerb::LatePartner, id.clone()));
                        }
                        RelationVerb::Spouse => {
                            mind.relations.retain(|(v, id)| !(id.eq(&mind.id) && v.eq(&verb)));
                            mind.relations.push((RelationVerb::LateSpouse, id.clone()));
                        }

                        RelationVerb::Acquaintance | RelationVerb::Friend | RelationVerb::CloseFriend | RelationVerb::Colleague => {
                            mind.relations.retain(|(v, id)| !(id.eq(&mind.id) && social_verbs.contains(&v)));
                        }

                        _ => {}
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
            citizens: HashMap::new(),
            areas: Vec::new(),
            institutions: Vec::new(),
        };
        generate_population_baseline(&dict, size, &mut city);
        let public_institutions = generate_public_institutions(&dict);

        for pub_inst in public_institutions {
            add_public_institution_to_city(&mut city, pub_inst, &dict);
        }

        for i in 0..age {
            println!("Y{} - Pop: {} - Dead: {}", i, city.citizens.values().filter(|c| c.alive).count(), city.citizens.values().filter(|c| !c.alive).count());
            let old_age_start = Instant::now();
            old_age_pass_per_year(&mut city, &culture);
            let old_age_time = Instant::now().duration_since(old_age_start);
            // Very Slow
            let link_friends_start = Instant::now();
            link_friends_within_population_by_year(&mut city);
            let friend_link_time = Instant::now().duration_since(link_friends_start);

            let link_partners_start = Instant::now();
            link_partners_by_year(&mut city);
            let partner_link_time =
                Instant::now().duration_since(link_partners_start);

            let update_partners_start = Instant::now();
            update_partners_by_year(&mut city);
            let partner_update_time = Instant::now().duration_since(update_partners_start);

            let gen_children_start = Instant::now();
            generate_children_per_year(&mut city, &culture, &dict);
            let generate_children_time = Instant::now().duration_since(gen_children_start);

            let add_buildings_start = Instant::now();
            add_buildings_per_year(&mut city, &dict);
            let add_buildings_time = Instant::now().duration_since(add_buildings_start);

            let assign_residences_start = Instant::now();
            assign_residences(&mut city);
            let residence_assign_time = Instant::now().duration_since(assign_residences_start);

            let random_sackings_start = Instant::now();
            random_sackings_per_year(&mut city);
            let random_sackings_time = Instant::now().duration_since(random_sackings_start);

            let assign_employer_start = Instant::now();
            assign_employment_per_year(&mut city);
            let assign_employer_time = Instant::now().duration_since(assign_employer_start);
            let startups_start = Instant::now();
            create_startups_per_year(&mut city, &dict);       
            let startups_time = startups_start.elapsed();  // TODO - create new institutions - allow institutions to come and go
            println!("Exec Time - Old Age: {} - Friend Link: {} - Partner Link: {} - Partner Update: {} - Generate Children: {} - Add Buildlings: {} - Residence Assignment: {} - Random Sackings: {} - Assign Employer: {} - Create Startups: {}",
                old_age_time.as_millis(), 
                friend_link_time.as_millis(), 
                partner_link_time.as_millis(), 
                partner_update_time.as_millis(), 
                generate_children_time.as_millis(), 
                add_buildings_time.as_millis(),
                residence_assign_time.as_millis(),
                random_sackings_time.as_millis(),
                assign_employer_time.as_millis(),
                startups_time.as_millis(),
            );
            for citizen in city.citizens.values_mut().filter(|c| c.alive) {
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
