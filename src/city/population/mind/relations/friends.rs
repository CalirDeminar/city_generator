pub mod friends {
    use crate::city::city::City;
    use crate::city::population::mind::mind::*;
    use crate::city::population::mind::relations::relations::RelationVerb;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use uuid::Uuid;

    pub const SOCIAL_RELATIONS: [RelationVerb; 3] = [
        RelationVerb::Acquaintance,
        RelationVerb::Friend,
        RelationVerb::CloseFriend,
    ];

    const ACQUAINTANCE_DECAY_CHANCE: f32 = 0.5;
    const ACQUAINTANCE_UPGRADE_CHANCE: f32 = 0.25;
    const FRIEND_DECAY_CHANCE: f32 = 0.25;
    const FRIEND_UPGRADE_CHANCE: f32 = 0.125;
    const CLOSE_FRIEND_DECAY_CHANCE: f32 = 0.125;

    const FRIEND_OUTGOING_MAX: f32 = 20.0;
    const FRIEND_MULTIPLIER_SAME_GENDER: f32 = 0.66;
    const FRIEND_MULTIPLER_DIFFERENT_GENDER: f32 = 0.33;
    const FRIEND_RATE: f32 = 0.01;

    fn match_friend(mind_1: &Mind, mind_2: &Mind) -> bool {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let gender_modifier: f32;
        if mind_1.gender.eq(&mind_2.gender) {
            gender_modifier = FRIEND_MULTIPLIER_SAME_GENDER;
        } else {
            gender_modifier = FRIEND_MULTIPLER_DIFFERENT_GENDER;
        }
        let age_gap_modifier = 1.0
            / ((mind_1.age.abs_diff(mind_2.age) as f32) * 5.0)
                .min(1.0)
                .max(0.0);
        let mind_1_knows_2 = mind_1.relations.iter().any(|(_r, id)| id.eq(&mind_2.id));
        return !mind_1_knows_2 && roll > (FRIEND_RATE * gender_modifier * age_gap_modifier);
    }

    pub fn link_friends_within_population<'a>(city: &'a mut City) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let ids: Vec<Uuid> = city
            .citizens
            .iter()
            .filter(|c| c.alive)
            .map(|c| c.id)
            .collect();
        for mind_id in ids {
            city.citizens.shuffle(&mut rng);
            let mut citizens = city.citizens.iter_mut().filter(|c| c.alive);
            let mind = citizens.find(|c| c.id.eq(&mind_id)).unwrap();
            let friend_count = (rng.gen::<f32>() * FRIEND_OUTGOING_MAX) as u32;
            for _i in 0..friend_count {
                let possible_friend = citizens.find(|c| match_friend(&mind, c));
                if possible_friend.is_some() {
                    let friend = possible_friend.unwrap();
                    mind.relations
                        .push((RelationVerb::Friend, friend.id.clone()));
                    friend
                        .relations
                        .push((RelationVerb::Friend, mind.id.clone()));
                }
            }
        }
        return city;
    }

    pub fn link_friends_within_population_by_year<'a>(city: &'a mut City) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let ids: Vec<Uuid> = city
            .citizens
            .iter()
            .filter(|c| c.alive)
            .map(|c| c.id)
            .collect();
        for mind_id in ids {
            city.citizens.shuffle(&mut rng);
            let mut citizens = city.citizens.iter_mut().filter(|c| c.alive);
            let mind = citizens.find(|c| c.id.eq(&mind_id)).unwrap();
            let friend_count = mind
                .relations
                .iter()
                .filter(|(v, _id)| SOCIAL_RELATIONS.contains(&v))
                .count();
            let acquaintances_to_add_count =
                (((rng.gen::<f32>() * FRIEND_OUTGOING_MAX) - (friend_count as f32)) as u32).max(0);
            for _i in 0..acquaintances_to_add_count {
                let possible_friend = citizens.find(|c| match_friend(&mind, c));
                if possible_friend.is_some() {
                    let friend = possible_friend.unwrap();
                    mind.relations
                        .push((RelationVerb::Acquaintance, friend.id.clone()));
                    friend
                        .relations
                        .push((RelationVerb::Acquaintance, mind.id.clone()));
                }
            }
            for (verb, id) in mind.relations.clone() {
                match verb {
                    RelationVerb::Acquaintance => {
                        if rng.gen::<f32>() < ACQUAINTANCE_DECAY_CHANCE {
                            mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));
                        } else if rng.gen::<f32>() < ACQUAINTANCE_UPGRADE_CHANCE {
                            mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));
                            mind.relations.push((RelationVerb::Friend, id.clone()));
                        }
                    }
                    RelationVerb::Friend => {
                        if rng.gen::<f32>() < FRIEND_DECAY_CHANCE {
                            mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));
                        } else if rng.gen::<f32>() < FRIEND_UPGRADE_CHANCE {
                            mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));
                            mind.relations.push((RelationVerb::CloseFriend, id.clone()));
                        }
                    }
                    RelationVerb::CloseFriend => {
                        if rng.gen::<f32>() < CLOSE_FRIEND_DECAY_CHANCE {
                            mind.relations.retain(|(v, i)| !(v.eq(&verb) && i.eq(&id)));
                        }
                    }
                    _ => {}
                }
            }
        }
        return city;
    }
}
