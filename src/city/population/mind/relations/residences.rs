pub mod residences {
    use rand::seq::SliceRandom;
    use rand::Rng;
    use uuid::Uuid;

    use crate::city::building::building::BuildingFloorArea;
    use crate::city::city::City;
    use crate::city::population::mind::mind::{add_residence_to_mind_log, Mind};
    use crate::city::population::mind::relations::relations::{
        find_relation, RelationVerb, ADULT_AGE_FROM,
    };

    const EVICITON_RATE: f32 = 0.05;

    pub fn random_evictions<'a>(city: &'a mut City) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let r = city.citizens.clone();
        let ref_pop: Vec<&Mind> = r
            .values()
            .filter(|c| c.alive && c.residence.is_some())
            .collect();
        let random_eviction_apartments: Vec<Uuid> = city
            .buildings
            .values()
            .flat_map(|b| {
                b.floors
                    .iter()
                    .flat_map(|f| f.areas.iter().map(|a| a.id.clone()))
            })
            .filter(|_a| rng.gen::<f32>() < EVICITON_RATE)
            .collect();
        for id in random_eviction_apartments {
            for m in &ref_pop {
                if m.residence.is_some() && m.residence.unwrap().eq(&id) {
                    let m_mut = city.citizens.get_mut(&m.id).unwrap();
                    m_mut.residence = None;

                    drop(m_mut);
                }
            }
        }
        for m in ref_pop {
            if m.age == 18 {
                let mind = city.citizens.get_mut(&m.id).unwrap();
                mind.residence = None;
                drop(mind);
            }
        }
        return city;
    }

    pub fn assign_residences<'a>(city: &'a mut City) -> &'a mut City {
        let ref_pop = city.citizens.clone();
        let mut owned_ids: Vec<Uuid> = city
            .citizens
            .values()
            .filter(|c| c.residence.is_some())
            .map(|c| c.residence.unwrap().clone())
            .collect();

        let mut all_areas: Vec<(&BuildingFloorArea, String, Uuid)> = city
            .buildings
            .values()
            .flat_map(|b| {
                b.floors.iter().flat_map(|f| {
                    f.areas
                        .iter()
                        .map(|a| (a, b.name.clone(), b.location_id.unwrap().clone()))
                })
            })
            .collect();
        all_areas.shuffle(&mut rand::thread_rng());

        for citizen in ref_pop
            .values()
            .filter(|c| c.alive && c.residence.is_none())
        {
            let mut target_res_id: Option<Uuid> = None;
            let guardian = if citizen.age < ADULT_AGE_FROM {
                find_relation(&citizen, RelationVerb::Parent, &ref_pop)
            } else {
                None
            };
            let guardian_res = if guardian.is_some() && guardian.unwrap().alive {
                guardian.unwrap().residence
            } else {
                None
            };
            // let ward = find_relation_minor(&citizen, RelationVerb::Child, &ref_pop);
            // let ward_res: Option<Uuid> = if ward.is_some() && ward.unwrap().alive {
            //     ward.unwrap().residence
            // } else {
            //     None
            // };
            let spouse = find_relation(&citizen, RelationVerb::Spouse, &ref_pop);
            let spouse_res: Option<Uuid> = if spouse.is_some() && spouse.unwrap().alive {
                spouse.unwrap().residence
            } else {
                None
            };
            // TODO - Currently broken, output looks very wrong
            if guardian_res.is_some() {
                target_res_id = guardian_res.clone();
            } else if spouse_res.is_some() {
                target_res_id = spouse_res.clone();
            }

            let apartment = all_areas.iter().find(|a| {
                (target_res_id.is_some() && a.0.id.eq(&target_res_id.unwrap()))
                    || (target_res_id.is_none()
                        && a.0.owning_institution.is_none()
                        && !owned_ids.contains(&a.0.id))
            });

            if apartment.is_some() {
                let (area, building_name, location_id) = apartment.unwrap();
                let location = city.areas.get(location_id);
                owned_ids.push(area.id.clone());
                let mind = city.citizens.get_mut(&citizen.id).unwrap();
                mind.residence = Some(area.id.clone());
                add_residence_to_mind_log(
                    mind,
                    city.year,
                    &area.name,
                    building_name,
                    &location.unwrap().name,
                );
                drop(mind);
            }
        }
        return city;
    }
}
