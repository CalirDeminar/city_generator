pub mod building;
pub mod html_exporter;
pub mod institutions;
pub mod locations;
pub mod population;
pub mod city {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use std::time::Instant;

    use rand::seq::SliceRandom;
    use rand::Rng;
    use uuid::Uuid;

    use super::building::building::*;
    use super::population::mind::mind::*;
    use super::population::mind::relations::relations::*;
    use crate::city::institutions::institutions::*;
    use crate::city::institutions::visits::visits::run_citizen_shopping;
    use crate::city::locations::{locations, locations::*};
    use crate::city::population::mind::relations::friends::friends::*;
    use crate::city::population::mind::relations::parents::parents::*;
    use crate::city::population::mind::relations::partners::partners::*;
    use crate::city::population::mind::relations::residences::residences::{
        assign_residences, random_evictions,
    };
    use crate::city::population::population::*;
    use crate::culture::culture::*;
    use crate::language::language::*;

    const MAX_WORKING_AGE: u32 = 60;

    #[derive(PartialEq, Debug, Clone)]
    pub struct City {
        pub name: String,
        pub citizens: Population,
        pub institutions: HashMap<Uuid, Institution>,
        pub areas: Vec<Location>,
        pub buildings: Vec<Building>,
        pub culture: CultureConfig,
        pub year: usize,
    }

    pub fn print_city(city: &City) -> String {
        let mut output: String = String::new();
        let alive_population = city.citizens.values().filter(|c| c.alive).count();
        let adult_population = city
            .citizens
            .values()
            .filter(|c| c.alive && c.age > ADULT_AGE_FROM)
            .count();
        output.push_str(&format!("City Name: {}\n", city.name));
        output.push_str(&format!("Population: {}\n", alive_population));
        output.push_str(&format!(
            "Employment Rate: {}\n",
            city.citizens
                .values()
                .filter(|c| c.alive && c.employer.is_some())
                .count() as f32
                / adult_population as f32
        ));
        output.push_str(&format!(
            "Dead: {}\n",
            city.citizens.iter().filter(|(_id, c)| !c.alive).count()
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

    pub fn export_city_stories(city: &City) {
        let mut file = File::create("./stories_export.txt").unwrap();
        let mut output = String::new();
        for citizen in city.citizens.values() {
            output.push_str(&format!(
                "==== {} {} - {} ====\n",
                citizen.first_name,
                citizen.last_name,
                citizen.gender.to_string()
            ));
            for line in citizen.activity_log.iter() {
                output.push_str(&format!("  {}\n", line));
            }
        }
        file.write_all(output.into_bytes().as_slice()).unwrap();
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
        let mut building_with_space = find_free_building(city);

        if building_with_space.is_none() {
            add_building_to_city(city, &dict, false);
            building_with_space = find_free_building(city);
        }

        add_institution_to_building(building_with_space.unwrap(), &institution.clone());

        // for w in workers {
        //     let mut worker = city.citizens.values_mut().find(|m| m.id.eq(&w.id)).unwrap();
        //     worker.employer = Some(institution.id.clone());
        // }
        city.institutions.insert(institution.id.clone(), institution);
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

        add_building_to_city(city, &dict, false);
        add_institution_to_building(city.buildings.last_mut().unwrap(), &institution);

        for w in workers {
            let mut worker = city.citizens.values_mut().find(|m| m.id.eq(&w.id)).unwrap();
            worker.employer = Some(institution.id.clone());
        }

        city.institutions.insert(institution.id.clone(), institution);

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
                mind.activity_log
                    .push(format!("Died in year {} age {}", city.year, mind.age));
                mind.alive = false;
                mind.employer = None;
                mind.residence = None;
                dead_ids.push(mind.id.clone());
            }
        }
        let social_verbs = vec![
            RelationVerb::Acquaintance,
            RelationVerb::Friend,
            RelationVerb::CloseFriend,
            RelationVerb::Colleague,
        ];
        for mind in city.citizens.values_mut() {
            for (verb, id) in mind.relations.clone() {
                if dead_ids.contains(&id) {
                    match verb {
                        RelationVerb::Partner => {
                            mind.relations
                                .retain(|(v, id)| !(id.eq(&mind.id) && v.eq(&verb)));
                            mind.relations.push((RelationVerb::LatePartner, id.clone()));
                        }
                        RelationVerb::Spouse => {
                            mind.relations
                                .retain(|(v, id)| !(id.eq(&mind.id) && v.eq(&verb)));
                            mind.relations.push((RelationVerb::LateSpouse, id.clone()));
                        }

                        RelationVerb::Acquaintance
                        | RelationVerb::Friend
                        | RelationVerb::CloseFriend
                        | RelationVerb::Colleague => {
                            mind.relations
                                .retain(|(v, id)| !(id.eq(&mind.id) && social_verbs.contains(&v)));
                        }

                        _ => {}
                    }
                }
            }
        }
        return city;
    }

    fn create_benchmarker(label: String) -> impl Fn() -> u128 {
        let label_padding: usize = 30;
        let epoch = Instant::now();
        return move || {
            let duration = Instant::now().duration_since(epoch.clone()).as_millis();
            println!(
                "{}:{}{}ms",
                label,
                " ".repeat(label_padding - label.len()),
                duration
            );
            return duration;
        };
    }

    pub fn simulate(size: usize, age: usize, era: Option<Era>) -> City {
        let dict = build_dictionary();
        let culture = random_culture(&dict, &era);

        println!("{:#?}", culture);
        let dict = build_culture_dictionary(&dict, &culture);
        let mut city = City {
            name: locations::gen_location_name(&dict, false, &era),
            buildings: Vec::new(),
            citizens: HashMap::new(),
            areas: Vec::new(),
            institutions: HashMap::new(),
            culture: culture.clone(),
            year: 0,
        };
        generate_population_baseline(&dict, size, &mut city);
        let public_institutions = generate_public_institutions(&dict, &era);

        for pub_inst in public_institutions {
            add_public_institution_to_city(&mut city, pub_inst, &dict);
        }

        for i in 0..age {
            println!("\n\nYear: {}", i);
            println!(
                "Population: {}",
                city.citizens.values().filter(|c| c.alive).count()
            );
            println!(
                "Dead: {}",
                city.citizens.values().filter(|c| !c.alive).count()
            );

            let old_age_benchmarker = create_benchmarker(String::from("Old Age"));
            old_age_pass_per_year(&mut city, &culture);
            old_age_benchmarker();
            // Very Slow
            let friend_linking_benchmarker = create_benchmarker(String::from("Link Friends"));
            link_friends_within_population_by_year(&mut city);
            friend_linking_benchmarker();

            let partner_linking_benchmarker = create_benchmarker(String::from("Link Partners"));
            link_partners_by_year(&mut city);
            partner_linking_benchmarker();

            let partner_update_benchmarker = create_benchmarker(String::from("Update Partners"));
            update_partners_by_year(&mut city);
            partner_update_benchmarker();

            let generate_children_benchmarker =
                create_benchmarker(String::from("Generate Children"));
            generate_children_per_year(&mut city, &culture, &dict);
            generate_children_benchmarker();

            let add_buildings_benchmarker = create_benchmarker(String::from("Add Buildings"));
            add_buildings_per_year(&mut city, &dict);
            add_buildings_benchmarker();

            let eviction_benchmarker = create_benchmarker(String::from("Evictions"));
            random_evictions(&mut city);
            assign_residences(&mut city);
            eviction_benchmarker();

            let sackings_benchmarker = create_benchmarker(String::from("Sackings"));
            random_sackings_per_year(&mut city);
            sackings_benchmarker();

            let employee_asignment_benchmarker =
                create_benchmarker(String::from("Assign Employers"));
            assign_employment_per_year(&mut city);
            employee_asignment_benchmarker();

            let create_startups_benchmarker = create_benchmarker(String::from("Create Startups"));
            create_startups_per_year(&mut city, &dict);
            create_startups_benchmarker();

            let create_shopping_benchmarker = create_benchmarker(String::from("Shopping"));
            run_citizen_shopping(&mut city);
            create_shopping_benchmarker();

            for citizen in city.citizens.values_mut().filter(|c| c.alive) {
                citizen.age += 1;
            }
            city.year += 1;
        }

        return city;
    }

    #[test]
    fn test_simulation() {
        simulate(1000, 20, None);
    }
}
