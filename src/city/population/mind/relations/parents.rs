pub mod parents {
    use std::collections::HashMap;

    use rand::seq::SliceRandom;
    use rand::Rng;
    use uuid::Uuid;

    use crate::{
        city::{
            city::City,
            population::{
                mind::{
                    mind::{random_char, Mind},
                    relations::{partners::partners::TAKEN_VERBS, relations::RelationVerb},
                },
                population::Population,
            },
        },
        culture::{self, culture::CultureConfig},
        language::language::Word,
    };

    const PARENT_PRESENCE_CHANCE: f32 = 0.3;
    const MIN_CHILD_BEARING_AGE: u32 = 20;
    const CHILD_LIMIT: usize = 5;

    const SPOUSE_CHILD_CHANCE: f32 = 0.1;
    const PARTNER_CHILD_CHANCE: f32 = 0.05;

    fn find_couples(population: Vec<&Mind>) -> Vec<(&Mind, &Mind)> {
        let mut output: Vec<(&Mind, &Mind)> = Vec::new();
        let ref_pop = population.clone();
        for mind in population {
            let possible_partner_relation = mind
                .relations
                .iter()
                .find(|(v, _id)| TAKEN_VERBS.contains(&v));
            if possible_partner_relation.is_some() {
                let partner_id = possible_partner_relation.unwrap().1;
                if ref_pop.iter().find(|c| c.id.eq(&partner_id)).is_none() {
                    println!(
                        "Missing Partner: ID: {} for {} {}",
                        partner_id, mind.first_name, mind.last_name
                    );
                }
                let partner = ref_pop.iter().find(|c| c.id.eq(&partner_id)).unwrap();
                let already_contained = output.iter().any(|(a, b)| {
                    let a_matches = a.id.eq(&mind.id) || a.id.eq(&partner_id);
                    let b_matches = b.id.eq(&mind.id) || b.id.eq(&partner_id);
                    return a_matches || b_matches;
                });
                if !already_contained {
                    output.push((mind, partner));
                }
            }
        }

        return output;
    }

    fn find_parent_ids(
        mind: &Mind,
        population: &Population,
        lockout_ids: &Vec<Uuid>,
    ) -> Option<Vec<Uuid>> {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < PARENT_PRESENCE_CHANCE {
            return None;
        }
        let filtered_parents: Vec<&Mind> = population
            .iter()
            .filter(|c| !lockout_ids.contains(&c.id))
            .collect();
        let mut potential_parents = find_couples(filtered_parents);
        potential_parents.shuffle(&mut rng);
        let target_age_range = (mind.age + MIN_CHILD_BEARING_AGE)..(u32::MAX);
        return potential_parents
            .iter()
            .find(|(a, b)| {
                return target_age_range.contains(&a.age) || target_age_range.contains(&b.age);
            })
            .map(|(a, b)| vec![a.id, b.id]);
    }

    fn get_lockout_parents(input: &Vec<(Uuid, Vec<Uuid>)>) -> Vec<Uuid> {
        let mut output: Vec<Uuid> = Vec::new();
        let mut frequency_table: HashMap<Uuid, usize> = HashMap::new();
        for (_, parents) in input {
            for id in parents {
                frequency_table
                    .entry(*id)
                    .and_modify(|i| *i += 1)
                    .or_insert(1);
                if frequency_table.get(&id).unwrap() >= &CHILD_LIMIT && !output.contains(&id) {
                    for id_all in parents {
                        output.push(*id_all);
                    }
                }
            }
        }
        return output;
    }

    pub fn link_parents<'a>(city: &'a mut City) -> &'a mut City {
        let citizen_ids: Vec<Uuid> = city.citizens.iter().map(|c| c.id).collect();
        let mut relations_to_add: Vec<(Uuid, Vec<Uuid>)> = Vec::new();

        for mind_id in citizen_ids {
            let mind = city.citizens.iter().find(|c| c.id.eq(&mind_id)).unwrap();
            let lockout_ids = get_lockout_parents(&relations_to_add);
            // println!("{:#?}", lockout_ids);
            let possible_parents = find_parent_ids(mind, &city.citizens, &lockout_ids);
            if possible_parents.is_some() {
                let parents = possible_parents.unwrap();
                let lockout_failed = parents.iter().any(|p| lockout_ids.contains(p));
                if lockout_failed {
                    println!("Lockout IDs Failed");
                }
                relations_to_add.push((mind.id.clone(), parents.clone()));
            }
        }

        for (target_id, parent_ids) in relations_to_add {
            let mut citizens = city.citizens.iter_mut();
            let target = citizens.find(|c| c.id.eq(&target_id)).unwrap();
            let parents: Vec<&mut Mind> = citizens.filter(|c| parent_ids.contains(&c.id)).collect();
            for parent in parents {
                target
                    .relations
                    .push((RelationVerb::Parent, parent.id.clone()));
                parent
                    .relations
                    .push((RelationVerb::Child, target.id.clone()));
            }
        }

        return city;
    }

    fn couple_will_bear(m1: &Mind, m2: &Mind, culture: &CultureConfig) -> bool {
        let mut rng = rand::thread_rng();
        let (verb, _id) = m1
            .relations
            .iter()
            .find(|(v, _id)| TAKEN_VERBS.contains(&v))
            .unwrap();
        let m1_child_count = m1
            .relations
            .iter()
            .filter(|(v, _id)| v.eq(&RelationVerb::Child))
            .count();
        let m2_child_count = m2
            .relations
            .iter()
            .filter(|(v, _id)| v.eq(&RelationVerb::Child))
            .count();
        if m1.age < culture.adult_age || m2.age < culture.adult_age {
            return false;
        }
        let base_chance = if verb.eq(&RelationVerb::Spouse) {
            SPOUSE_CHILD_CHANCE
        } else {
            PARTNER_CHILD_CHANCE
        };
        let chance = base_chance / ((m1_child_count + m2_child_count) as f32 / 2.0);
        return rng.gen::<f32>() < chance;
    }

    pub fn generate_children_per_year<'a>(
        city: &'a mut City,
        culture: &CultureConfig,
        dict: &Vec<Word>,
    ) -> &'a mut City {
        let citizen_ref: Vec<Mind> = city
            .citizens
            .iter()
            .filter(|c| c.alive)
            .map(|c| c.clone())
            .collect();
        let couples = find_couples(citizen_ref.iter().collect());

        for (m1, m2) in couples {
            if couple_will_bear(m1, m2, &culture) {
                let mut citizens = city.citizens.iter_mut().filter(|c| c.alive);
                let pm1 = citizens.find(|m| m.id.eq(&m1.id));
                let pm2 = citizens.find(|m| m.id.eq(&m2.id));
                if pm1.is_some() && pm2.is_some() {
                    let mut child = random_char(&dict);
                    child.age = 1;
                    let mind_1 = pm1.unwrap();
                    let mind_2 = pm2.unwrap();
                    mind_1
                        .relations
                        .push((RelationVerb::Child, child.id.clone()));
                    mind_2
                        .relations
                        .push((RelationVerb::Child, child.id.clone()));
                    child
                        .relations
                        .push((RelationVerb::Parent, mind_1.id.clone()));
                    child
                        .relations
                        .push((RelationVerb::Parent, mind_2.id.clone()));
                    city.citizens.push(child.clone());
                }
            }
        }
        return city;
    }
}
