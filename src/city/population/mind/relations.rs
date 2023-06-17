pub mod relations {
    use std::ops::Range;

    use crate::city::city::City;
    use crate::city::population::{mind::mind::*, population::Population};
    use crate::names::names::*;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use rand_distr::{Distribution, Normal};
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

    const PARTNER_CHANCE_GENERAL: f32 = 0.8;
    const PARTNER_MARRIAGE_RATE: f32 = 0.5;
    const PARTNER_SPLIT_RATE: f32 = 0.2;

    pub const ADULT_AGE_FROM: u32 = 18;

    const PARENT_PRESENCE_CHANCE: f32 = 0.3;

    const FRIEND_OUTGOING_MAX: f32 = 3.0;
    const FRIEND_MULTIPLIER_SAME_GENDER: f32 = 0.66;
    const FRIEND_MULTIPLER_DIFFERENT_GENDER: f32 = 0.33;
    const FRIEND_RATE: f32 = 0.01;

    struct MindSearchFilter<'a> {
        target_gender: Option<Gender>,
        age_range: Range<u32>,
        required_relations: Vec<RelationVerb>,
        without_relations: Vec<RelationVerb>,
        ignored_ids: Vec<&'a Uuid>,
        sexuality: Vec<Sexuality>,
    }

    fn mind_without_these_relations(mind: &Mind, relations: &Vec<RelationVerb>) -> bool {
        return relations.len() == 0
            || mind
                .relations
                .iter()
                .all(|(v, _id)| !relations.contains(&v));
    }

    fn mind_with_these_relations(mind: &Mind, relations: &Vec<RelationVerb>) -> bool {
        return relations.len() == 0
            || mind.relations.iter().any(|(v, _id)| relations.contains(&v));
    }

    fn find_id_for_relation(population: &Population, filter: MindSearchFilter) -> Option<Uuid> {
        let mut rng = rand::thread_rng();

        let mut target_gender_population: Vec<&Mind> = population
            .iter()
            .filter(|c| {
                let gender_match = filter.target_gender.eq(&Some(c.gender.clone()));
                // println!("Gender Match: {} - In Age Range: {}", gender_match, filter.age_range);
                gender_match
                    && mind_without_these_relations(&c, &filter.without_relations)
                    && mind_with_these_relations(&c, &filter.required_relations)
                    && filter.age_range.contains(&c.age)
                    && !filter.ignored_ids.contains(&&c.id)
            })
            .collect();
        target_gender_population.shuffle(&mut rng);
        let rtn = target_gender_population.first();
        if rtn.is_some() {
            return Some(rtn.unwrap().id);
        }
        return None;
    }

    fn compatible_sexuality(input: &Sexuality) -> Vec<Sexuality> {
        return match input {
            &Sexuality::Homosexual => vec![Sexuality::Homosexual, Sexuality::Bisexual],
            &Sexuality::Bisexual => vec![Sexuality::Homosexual, Sexuality::Bisexual],
            &Sexuality::Hetrosexual => vec![Sexuality::Hetrosexual],
            _ => Vec::new(),
        };
    }

    fn find_partner_id(mind: &Mind, population: &Population, to_ignore: &Uuid) -> Option<Uuid> {
        let mut rng = rand::thread_rng();
        let target_gender = determine_partner_gender(&mind);
        for i in 0..20 {
            let max_age_gap = (rng.gen::<f32>() * 2.0 * (i as f32)) as u32;
            let underflowing_min_age =
                (mind.age as i32 - max_age_gap as i32) < ADULT_AGE_FROM as i32;
            let min_age = if underflowing_min_age {
                ADULT_AGE_FROM as u32
            } else {
                mind.age - max_age_gap
            };
            let possible_partner = find_id_for_relation(
                population,
                MindSearchFilter {
                    target_gender: Some(target_gender.clone()),
                    age_range: min_age..(mind.age + max_age_gap),
                    without_relations: vec![RelationVerb::Partner, RelationVerb::Spouse],
                    required_relations: vec![],
                    ignored_ids: vec![to_ignore],
                    sexuality: compatible_sexuality(&mind.sexuality),
                },
            );
            if possible_partner.is_some() {
                return possible_partner;
            }
        }
        return None;
    }

    fn find_parent_id(mind: &Mind, city: &City) -> Option<Uuid> {
        return find_id_for_relation(
            &city.citizens,
            MindSearchFilter {
                target_gender: None,
                age_range: (mind.age + 18)..(mind.age + 50),
                without_relations: Vec::new(),
                required_relations: vec![
                    RelationVerb::Spouse,
                    RelationVerb::Partner,
                    RelationVerb::ExPartner,
                    RelationVerb::ExSpouse,
                ],
                ignored_ids: vec![],
                sexuality: vec![],
            },
        );
    }

    fn invert_gender(gender: &Gender) -> Gender {
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

    fn determine_partner_gender(mind: &Mind) -> Gender {
        if mind.sexuality.eq(&Sexuality::Hetrosexual) {
            return invert_gender(&mind.gender);
        } else if mind.sexuality.eq(&Sexuality::Homosexual) {
            return mind.gender.clone();
        } else {
            return invert_gender(&Gender::Ambiguous);
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

    fn get_partner_verb() -> RelationVerb {
        let mut rng = rand::thread_rng();
        let married = rng.gen::<f32>() < PARTNER_MARRIAGE_RATE;
        let split = rng.gen::<f32>() < PARTNER_SPLIT_RATE;
        let verb: RelationVerb;
        if married {
            if split {
                verb = RelationVerb::ExSpouse;
            } else {
                verb = RelationVerb::Spouse;
            }
        } else {
            if split {
                verb = RelationVerb::ExPartner
            } else {
                verb = RelationVerb::Partner;
            }
        }
        return verb;
    }

    pub fn link_partners<'a>(city: &'a mut City) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let ids: Vec<Uuid> = city.citizens.iter().map(|c| c.id).collect();
        let mut relations_to_add: Vec<(Uuid, Uuid)> = Vec::new();
        for mind_id in ids {
            city.citizens.shuffle(&mut rng);
            let mind = city.citizens.iter().find(|c| c.id.eq(&mind_id)).unwrap();
            let possible_partner_id = find_partner_id(&mind, &city.citizens, &mind.id);
            if possible_partner_id.is_some() {
                relations_to_add.push((mind.id.clone(), possible_partner_id.unwrap()));
            }
        }
        for (id_1, id_2) in relations_to_add {
            let citizens = city.citizens.iter_mut();
            let mut mind_1: Option<&mut Mind> = None;
            let mut mind_2: Option<&mut Mind> = None;
            for mind in citizens {
                if mind.id.eq(&id_1) {
                    mind_1 = Some(mind);
                } else if mind.id.eq(&id_2) {
                    mind_2 = Some(mind);
                }
            }
            if mind_1.is_some() && mind_2.is_some() {
                let verb = get_partner_verb();
                mind_1.unwrap().relations.push((verb.clone(), id_2.clone()));
                mind_2.unwrap().relations.push((verb.clone(), id_1.clone()));
            } else {
                println!("Mind Lookup Failed");
            }
        }
        return city;
    }

    pub fn link_parents<'a>(city: &'a mut City) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let ids: Vec<Uuid> = city.citizens.iter().map(|c| c.id).collect();
        for mind_id in ids {
            city.citizens.shuffle(&mut rng);
            let cl = city.clone();
            let mut citizens = city.citizens.iter_mut();
            let mind = citizens.find(|c| c.id.eq(&mind_id)).unwrap();
            let possible_parent_id = find_parent_id(&mind, &cl);
            if possible_parent_id.is_some() {
                let parent_id = possible_parent_id.unwrap();
                mind.relations.push((RelationVerb::Parent, parent_id));
                let possible_parent = citizens.find(|c| c.id.eq(&parent_id));
                if possible_parent.is_some() {
                    let parent = possible_parent.unwrap();
                    parent
                        .relations
                        .push((RelationVerb::Child, mind.id.clone()));
                    let parent_2_id = parent.relations.iter().find(|(v, _id)| {
                        v.eq(&RelationVerb::Partner)
                            || v.eq(&&RelationVerb::ExPartner)
                            || v.eq(&RelationVerb::Spouse)
                            || v.eq(&RelationVerb::ExSpouse)
                    });
                    if parent_2_id.is_some() {
                        mind.relations
                            .push((RelationVerb::Parent, parent_2_id.unwrap().1));
                        let possible_parent_2 = citizens.find(|c| c.id.eq(&parent_2_id.unwrap().1));
                        if possible_parent_2.is_some() {
                            let parent_2 = possible_parent_2.unwrap();
                            parent_2
                                .relations
                                .push((RelationVerb::Child, mind.id.clone()));
                        }
                    }
                }
            }
        }
        return city;
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
}
