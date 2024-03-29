pub mod friends {
    use std::collections::HashMap;

    use crate::city::city::City;
    use crate::city::institutions::visits::visits::get_habitual_institutions;
    use crate::city::population::mind::mind::*;
    use crate::city::population::mind::relations::relations::RelationVerb;
    use crate::city::population::population::Population;
    use crate::culture::culture::CultureConfig;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use uuid::Uuid;

    pub const SOCIAL_RELATIONS: [RelationVerb; 3] = [
        RelationVerb::Acquaintance,
        RelationVerb::Friend,
        RelationVerb::CloseFriend,
    ];

    const ACQUAINTANCE_DECAY_CHANCE: f32 = 0.6;
    const ACQUAINTANCE_UPGRADE_CHANCE: f32 = 0.25;
    const FRIEND_DECAY_CHANCE: f32 = 0.25;
    const FRIEND_UPGRADE_CHANCE: f32 = 0.125;
    const CLOSE_FRIEND_DECAY_CHANCE: f32 = 0.125;

    const FRIEND_OUTGOING_MAX: f32 = 20.0;
    const FRIEND_MULTIPLIER_SAME_GENDER: f32 = 0.66;
    const FRIEND_MULTIPLER_DIFFERENT_GENDER: f32 = 0.33;
    const FRIEND_RATE: f32 = 0.5;

    type AgeCache<'a> = HashMap<u32, Vec<&'a Mind>>;

    fn process_age_cache<'a>(
        population: &'a AgeCache,
        cache: &'a mut (AgeCache<'a>, AgeCache<'a>),
        target_age: u32,
        mind: &Mind,
    ) -> &'a mut (AgeCache<'a>, AgeCache<'a>) {
        if !(cache.0.contains_key(&target_age) && cache.1.contains_key(&target_age))
            && population.contains_key(&target_age)
        {
            let mut to_add_0: Vec<&'a Mind> = Vec::new();
            let mut to_add_1: Vec<&'a Mind> = Vec::new();
            let source = population.get(&target_age).unwrap();
            for m in source {
                if !m.id.eq(&mind.id)
                    && m.relations.len() < FRIEND_OUTGOING_MAX as usize
                    && !mind.relations.iter().any(|(_v, id)| id.eq(&m.id))
                    && !m.relations.iter().any(|(_v, id)| id.eq(&mind.id))
                {
                    if m.gender.eq(&mind.gender) {
                        to_add_0.push(m.clone());
                    } else {
                        to_add_1.push(m.clone());
                    }
                }
            }
            cache.0.insert(target_age, to_add_0);
            cache.1.insert(target_age, to_add_1);
        }
        return cache;
    }

    fn get_friend<'a>(
        mind: &Mind,
        age_population: &'a AgeCache,
        friend_cache: &'a mut (AgeCache<'a>, AgeCache<'a>),
        culture: &CultureConfig,
        pending_friends: &Vec<Uuid>,
    ) -> (Option<Uuid>, &'a mut (AgeCache<'a>, AgeCache<'a>)) {
        let mut cache = friend_cache;
        let mut rng = rand::thread_rng();
        let mut years_above: u32;
        let mut years_below: u32;
        let max_deviation = if mind.age < culture.adult_age { 3 } else { 30 };
        let ages_processed: Vec<u32> = Vec::new();
        // limit children to a max deviation of 3yrs in each direction
        for i in 0..40 {
            years_above = i.min(max_deviation);
            years_below = i.min(max_deviation);
            let target_above = mind.age + years_above;
            let target_below =
                (mind.age as i32 - years_below as i32).min(if mind.age > culture.adult_age {
                    culture.adult_age as i32
                } else {
                    0
                }) as u32;
            cache = process_age_cache(&age_population, cache, target_above, &mind);
            cache = process_age_cache(&age_population, cache, target_below, &mind);
            let mut buffer_same_gender_above: &Vec<&Mind> = &Vec::new();
            let mut buffer_same_gender_below: &Vec<&Mind> = &Vec::new();
            let mut buffer_different_gender_above: &Vec<&Mind> = &Vec::new();
            let mut buffer_different_gender_below: &Vec<&Mind> = &Vec::new();

            if cache.0.contains_key(&target_above)
                && cache.1.contains_key(&target_above)
                && !ages_processed.contains(&years_above)
            {
                buffer_same_gender_above = cache.0.get(&target_above).unwrap();
                buffer_different_gender_above = cache.1.get(&target_above).unwrap();
            }
            if cache.0.contains_key(&target_below)
                && cache.1.contains_key(&target_below)
                && !ages_processed.contains(&target_below)
            {
                buffer_same_gender_below = cache.0.get(&target_below).unwrap();
                buffer_different_gender_below = cache.1.get(&target_below).unwrap();
            }

            let same_gender_target_roll = (buffer_same_gender_above.len()
                + buffer_same_gender_below.len()) as f32
                * FRIEND_MULTIPLIER_SAME_GENDER
                * FRIEND_RATE;
            let different_gender_target_roll =
                (buffer_different_gender_above.len() + buffer_different_gender_below.len()) as f32
                    * FRIEND_MULTIPLER_DIFFERENT_GENDER
                    * FRIEND_RATE;

            let roll = rng.gen::<f32>();
            let mut working_buffers: (&Vec<&Mind>, &Vec<&Mind>) = (&Vec::new(), &Vec::new());
            if roll < same_gender_target_roll {
                working_buffers = (buffer_same_gender_below, buffer_same_gender_above);
            } else if roll < same_gender_target_roll + different_gender_target_roll {
                working_buffers = (buffer_different_gender_below, buffer_different_gender_above);
            }
            let buffer_choice_limit = working_buffers.0.len() as f32
                / (working_buffers.0.len() + working_buffers.1.len()) as f32;
            let buffer_choice = rng.gen::<f32>() < buffer_choice_limit;
            let mut target_buffer: Vec<&Mind> = if buffer_choice {
                working_buffers.0.clone()
            } else {
                working_buffers.1.clone()
            };
            target_buffer.shuffle(&mut rng);
            let r = target_buffer.first();
            if r.is_some() && !pending_friends.iter().any(|m| r.unwrap().id.eq(&m)) {
                let rtn = r.unwrap();
                cache
                    .0
                    .get_mut(&rtn.age)
                    .unwrap()
                    .retain(|m| !m.id.eq(&rtn.id));
                cache
                    .1
                    .get_mut(&rtn.age)
                    .unwrap()
                    .retain(|m| !m.id.eq(&rtn.id));
                return (Some(rtn.id), cache);
            }
        }
        return (None, cache);
    }

    fn hash_population_by_age<'a>(population: &'a Population) -> AgeCache<'a> {
        let mut output: AgeCache = HashMap::new();
        for mind in population.values() {
            if output.contains_key(&mind.age) {
                let mut current = output.get(&mind.age).unwrap().clone();
                current.push(mind);
                output.insert(mind.age, current);
            } else {
                output.insert(mind.age, vec![mind]);
            }
        }
        return output;
    }

    fn hash_population_by_inst_visits(population: &Population) {
        let mut cache: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for mind in population.values() {
            let (shopping_locs, _) = get_habitual_institutions(mind, &crate::city::institutions::visits::visits::VisitType::Shopping);
            let (social_locs, _) = get_habitual_institutions(mind, &crate::city::institutions::visits::visits::VisitType::Social);
            let locs = vec![shopping_locs, social_locs].concat();
            for loc in locs {
                if !cache.contains_key(loc) {
                    cache.insert(loc.clone(), vec![mind.id]);
                } else {
                    let current = cache.get_mut(loc).unwrap();
                    current.push(mind.id.clone());
                    drop(current);
                }
            }
        }
    }

    fn process_friend_evolution<'a>(mind_id: &Uuid, city: &'a mut City) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let mind_ref = city.citizens.get(mind_id).unwrap();
        for (verb, id) in mind_ref.relations.clone() {
            let mind = city.citizens.get_mut(&mind_id).unwrap();
            match verb {
                RelationVerb::Acquaintance => {
                    if rng.gen::<f32>() < ACQUAINTANCE_DECAY_CHANCE {
                        mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));

                        drop(mind);
                        let relation = city.citizens.get_mut(&id).unwrap();
                        relation
                            .relations
                            .retain(|(v, i)| !(v.eq(&verb) && i.eq(&mind_id)));
                    } else if rng.gen::<f32>() < ACQUAINTANCE_UPGRADE_CHANCE {
                        mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));
                        mind.relations.push((RelationVerb::Friend, id.clone()));

                        drop(mind);
                        let relation = city.citizens.get_mut(&id).unwrap();
                        relation
                            .relations
                            .retain(|(v, i)| !(v.eq(&verb) && i.eq(&mind_id)));
                        relation
                            .relations
                            .push((RelationVerb::Friend, mind_id.clone()));
                    }
                }
                RelationVerb::Friend => {
                    if rng.gen::<f32>() < FRIEND_DECAY_CHANCE {
                        mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));
                        mind.relations
                            .push((RelationVerb::Acquaintance, id.clone()));
                        drop(mind);
                        let relation = city.citizens.get_mut(&id).unwrap();
                        relation
                            .relations
                            .retain(|(v, i)| !(v.eq(&verb) && i.eq(&mind_id)));
                        relation
                            .relations
                            .push((RelationVerb::Acquaintance, mind_id.clone()));
                    } else if rng.gen::<f32>() < FRIEND_UPGRADE_CHANCE {
                        mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));
                        mind.relations.push((RelationVerb::CloseFriend, id.clone()));

                        drop(mind);
                        let relation = city.citizens.get_mut(&id).unwrap();
                        relation
                            .relations
                            .retain(|(v, i)| !(v.eq(&verb) && i.eq(&mind_id)));
                        relation
                            .relations
                            .push((RelationVerb::CloseFriend, mind_id.clone()));
                    }
                }
                RelationVerb::CloseFriend => {
                    if rng.gen::<f32>() < CLOSE_FRIEND_DECAY_CHANCE {
                        mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));
                        mind.relations.push((RelationVerb::Friend, id.clone()));
                        drop(mind);
                        let relation = city.citizens.get_mut(&id).unwrap();
                        relation
                            .relations
                            .retain(|(v, i)| !(v.eq(&verb) && i.eq(&mind_id)));
                        relation
                            .relations
                            .push((RelationVerb::Friend, mind_id.clone()));
                    }
                }
                _ => {}
            }
        }
        return city;
    }

    pub fn link_friends_within_population_by_year<'a>(city: &'a mut City) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let mut friendable_population = city.citizens.clone();
        friendable_population.retain(|_id, m| m.alive);
        let ids = friendable_population.keys();

        let population_by_age = hash_population_by_age(&friendable_population);

        for mind_id in ids {
            // city.citizens.shuffle(&mut rng);
            let mind = city.citizens.get(&mind_id).unwrap().clone();
            let mut friend_cache: (AgeCache, AgeCache) = (HashMap::new(), HashMap::new());
            let mut cache = &mut friend_cache;

            let mut pending_friends: Vec<Uuid> = vec![];

            let friend_count = mind
                .relations
                .iter()
                .filter(|(v, _id)| SOCIAL_RELATIONS.contains(&v))
                .count();

            let acquaintances_to_add_count =
                (((rng.gen::<f32>() * FRIEND_OUTGOING_MAX) - (friend_count as f32)) as u32).max(0);

            for _i in 0..acquaintances_to_add_count {
                // Extremely slow line
                let possible_friend_id: Option<Uuid>;

                (possible_friend_id, cache) = get_friend(
                    &mind,
                    &population_by_age,
                    cache,
                    &city.culture,
                    &pending_friends,
                );

                if possible_friend_id.is_some() {
                    let friend_id = possible_friend_id.unwrap();
                    pending_friends.push(friend_id.clone());

                    let friend_m = city.citizens.get_mut(&friend_id).unwrap();
                    friend_m
                        .relations
                        .push((RelationVerb::Acquaintance, mind.id.clone()));
                    // let cache_entry_0 = cache.0.get(&friend_m.age).unwrap();
                    // if cache_entry_0.iter().any(|m| m.id.eq(&friend_m.id)) {
                    //     println!("Repeat Cache Persisting - 0")
                    // }
                    // let cache_entry_1 = cache.1.get(&friend_m.age).unwrap();
                    // if cache_entry_1.iter().any(|m| m.id.eq(&friend_m.id)) {
                    //     println!("Repeat Cache Persisting - 1")
                    // }
                    drop(friend_m);

                    let mind_m = city.citizens.get_mut(&mind.id).unwrap();
                    mind_m
                        .relations
                        .push((RelationVerb::Acquaintance, friend_id.clone()));
                    drop(mind_m);
                }
            }
            
            process_friend_evolution(mind_id, city);
        }
        return city;
    }
}
