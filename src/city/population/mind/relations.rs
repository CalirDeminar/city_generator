pub mod parents;
pub mod partners;
pub mod relations {
    use crate::city::city::City;
    use crate::city::population::{mind::mind::*, population::Population};
    use rand::seq::SliceRandom;
    use rand::Rng;
    use uuid::Uuid;

    #[derive(PartialEq, Debug, Clone)]
    pub enum RelationVerb {
        // family
        Parent,
        Child,
        Partner,
        ExPartner,
        Spouse,
        ExSpouse,
        Sibling,
        Grandparent,
        Grandchild,
        Cousin,
        // business
        Employer,
        Employee,
        Colleague,
        // social
        Acquaintance,
        Friend,
        CloseFriend,
        Grudge,
        // religion
        Diety,
        Priest,
    }

    pub const ADULT_AGE_FROM: u32 = 18;

    const FRIEND_OUTGOING_MAX: f32 = 3.0;
    const FRIEND_MULTIPLIER_SAME_GENDER: f32 = 0.66;
    const FRIEND_MULTIPLER_DIFFERENT_GENDER: f32 = 0.33;
    const FRIEND_RATE: f32 = 0.01;

    pub fn invert_gender(gender: &Gender) -> Gender {
        let mut rng = rand::thread_rng();
        if gender.eq(&Gender::Male) {
            return Gender::Female;
        }
        if gender.eq(&Gender::Female) {
            return Gender::Male;
        } else {
            if rng.gen::<f32>() > 0.5 {
                return Gender::Male;
            } else {
                return Gender::Female;
            }
        }
    }

    pub fn find_relation<'a>(
        mind: &Mind,
        relation: RelationVerb,
        city: &'a City,
    ) -> Option<&'a Mind> {
        let match_relation = mind.relations.iter().find(|(r, _id)| r.eq(&relation));
        if match_relation.is_some() {
            let (_verb, id) = match_relation.unwrap();
            return city.citizens.iter().find(|c| c.id.eq(id));
        }
        return None;
    }

    pub fn find_relation_minor<'a>(
        mind: &Mind,
        relation: RelationVerb,
        city: &'a City,
    ) -> Option<&'a Mind> {
        let match_relation: Vec<&(RelationVerb, Uuid)> = mind
            .relations
            .iter()
            .filter(|(r, _id)| r.eq(&relation))
            .collect();
        for (_relation, id) in match_relation {
            let rel = city.citizens.iter().find(|c| c.id.eq(&id)).unwrap();
            if rel.age < ADULT_AGE_FROM {
                return Some(rel);
            }
        }
        return None;
    }

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
        let ids: Vec<Uuid> = city.citizens.iter().map(|c| c.id).collect();
        for mind_id in ids {
            city.citizens.shuffle(&mut rng);
            let mut citizens = city.citizens.iter_mut();
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

    pub fn link_colleagues<'a>(city: &'a mut City) -> &'a mut City {
        let ref_pop = city.citizens.clone();
        let mut output: Population = Vec::new();

        for m in city.citizens.iter_mut() {
            let mut mind = m.clone();
            if mind.employer.is_some() {
                let colleagues: Vec<&Mind> = ref_pop
                    .iter()
                    .filter(|c| {
                        !c.id.eq(&mind.id)
                            && c.employer.is_some()
                            && c.employer.unwrap().eq(&mind.employer.unwrap())
                    })
                    .collect();
                for c in colleagues {
                    mind.relations.push((RelationVerb::Colleague, c.id.clone()))
                }
            }
            output.push(mind);
        }

        return city;
    }

    pub fn link_siblings<'a>(city: &'a mut City) -> &'a mut City {
        let ref_pop = city.citizens.clone();

        for m in city.citizens.iter_mut() {
            let parents: Vec<&Uuid> = m
                .relations
                .iter()
                .filter(|(v, _id)| v.eq(&RelationVerb::Parent))
                .map(|(_v, id)| id)
                .collect();
            let siblings: Vec<Uuid> = ref_pop
                .iter()
                .filter(|m| {
                    m.relations
                        .iter()
                        .any(|(v, id)| v.eq(&RelationVerb::Parent) && parents.contains(&id))
                })
                .map(|m| m.id)
                .collect();
            for sibling in siblings {
                m.relations.push((RelationVerb::Sibling, sibling));
            }
        }

        return city;
    }

    pub fn link_grandparents<'a>(city: &'a mut City) -> &'a mut City {
        let ref_pop = city.citizens.clone();
        for m in &city.citizens.clone() {
            let mut citizens = city.citizens.iter_mut();
            let parents: Vec<&Mind> = m
                .relations
                .iter()
                .filter(|(v, _id)| v.eq(&RelationVerb::Parent))
                .map(|(_v, id)| ref_pop.iter().find(|c| c.id.eq(id)).unwrap())
                .collect();
            let grandparent_ids: Vec<&Uuid> = parents
                .iter()
                .map(|c| {
                    c.relations
                        .iter()
                        .filter(|(v, _id)| v.eq(&RelationVerb::Parent))
                        .map(|(_v, id)| id)
                })
                .flatten()
                .collect();
            for id in grandparent_ids {
                let mind_opt = citizens.find(|c| c.id.eq(&&m.id));
                let grandparent_opt = citizens.find(|c| c.id.eq(&id));
                if mind_opt.is_some() && grandparent_opt.is_some() {
                    let mind = mind_opt.unwrap();
                    let grandparent = grandparent_opt.unwrap();
                    mind.relations
                        .push((RelationVerb::Grandparent, grandparent.id.clone()));
                    grandparent
                        .relations
                        .push((RelationVerb::Grandchild, mind.id.clone()));
                }
            }
        }
        return city;
    }
}
