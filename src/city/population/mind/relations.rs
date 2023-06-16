pub mod relations {
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

    const PARTNER_CHANCE_GENERAL: f32 = 0.5;
    const PARTNER_MARRIAGE_RATE: f32 = 0.5;
    const PARTNER_SPLIT_RATE: f32 = 0.2;
    const HOMOSEXUALITY_CHANCE: f32 = 0.2;

    pub const ADULT_AGE_FROM: u32 = 18;

    const PARENT_PRESENCE_CHANCE: f32 = 0.3;

    const FRIEND_OUTGOING_MAX: f32 = 3.0;
    const FRIEND_MULTIPLIER_SAME_GENDER: f32 = 0.66;
    const FRIEND_MULTIPLER_DIFFERENT_GENDER: f32 = 0.33;
    const FRIEND_RATE: f32 = 0.01;

    fn gen_mind_with_gender_and_relation(
        name_dict: &NameDictionary,
        gender: &Gender,
        age: u32,
        relations: Vec<Relation>,
    ) -> Mind {
        let (first_name, last_name) = random_mind_name(&name_dict, &gender);
        return Mind {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            gender: gender.clone(),
            age,
            relations,
            employer: None,
            residence: None,
        };
    }

    fn mind_is_single(mind: &Mind) -> bool {
        return !mind
            .relations
            .iter()
            .any(|(v, _i)| v.eq(&RelationVerb::Partner) || v.eq(&RelationVerb::Spouse));
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

    fn find_id_for_relation(
        target_gender: &Gender,
        max_age: u32,
        min_age: u32,
        without_relations: Vec<RelationVerb>,
        with_relations: Vec<RelationVerb>,
        city: &City,
    ) -> Option<Uuid> {
        let mut rng = rand::thread_rng();

        let mut target_gender_population: Vec<&Mind> = city
            .citizens
            .iter()
            .filter(|c| {
                c.gender.eq(&target_gender)
                    && mind_without_these_relations(&c, &without_relations)
                    && mind_with_these_relations(&c, &with_relations)
                    && c.age > min_age
                    && c.age < max_age
            })
            .collect();
        target_gender_population.shuffle(&mut rng);
        let rtn = target_gender_population.first();
        if rtn.is_some() {
            return Some(rtn.unwrap().id);
        }
        return None;
    }

    fn find_partner_id(mind: &Mind, city: &City) -> Option<Uuid> {
        let mut rng = rand::thread_rng();
        let target_gender = gen_partner_gender(&mind.gender);
        let max_age_gap = (rng.gen::<f32>() * 10.0) as u32;
        return find_id_for_relation(
            &target_gender,
            mind.age + max_age_gap,
            mind.age - max_age_gap,
            vec![RelationVerb::Partner, RelationVerb::Spouse],
            vec![],
            &city,
        );
    }

    fn find_parent_id(mind: &Mind, city: &City) -> Option<Uuid> {
        let mut rng = rand::thread_rng();
        let target_gender = gen_partner_gender(&mind.gender);
        return find_id_for_relation(
            &target_gender,
            mind.age + 50,
            mind.age + 18,
            vec![],
            vec![
                RelationVerb::Spouse,
                RelationVerb::Partner,
                RelationVerb::ExPartner,
                RelationVerb::ExSpouse,
            ],
            &city,
        );
    }

    fn gen_partner_gender(input_gender: &Gender) -> Gender {
        let mut rng = rand::thread_rng();
        let partner_type_roll = rng.gen::<f32>();
        let parnet_gender;
        if partner_type_roll > HOMOSEXUALITY_CHANCE {
            parnet_gender = if input_gender.eq(&Gender::Male) {
                Gender::Female
            } else {
                Gender::Male
            };
        } else {
            parnet_gender = input_gender.clone();
        }
        return parnet_gender;
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

    pub fn add_partners_to_population(
        population: Vec<Mind>,
        name_dict: &NameDictionary,
    ) -> Vec<Mind> {
        // TODO - Ensure Last Name Consistency (Sometimes?)
        let mut rng = rand::thread_rng();
        let mut output: Vec<Mind> = Vec::new();
        for mind in population {
            let has_partner = rng.gen::<f32>() > PARTNER_CHANCE_GENERAL;
            if !has_partner {
                output.push(mind.clone());
                continue;
            } else {
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
                let partner_gender = gen_partner_gender(&mind.gender);
                let mut target = mind.clone();
                let distribution = Normal::new(mind.age.clone() as f32, 4.0).unwrap();
                let relation = gen_mind_with_gender_and_relation(
                    &name_dict,
                    &partner_gender,
                    distribution.sample(&mut rand::thread_rng()) as u32,
                    vec![(verb.clone(), target.id.clone())],
                );
                target.relations.push((verb.clone(), relation.id.clone()));
                assert!(relation.relations.iter().any(|(_, m)| m == &target.id));
                output.push(target);
                output.push(relation);
            }
        }
        return output;
    }

    pub fn add_parents_to_population(
        population: Vec<Mind>,
        name_dict: &NameDictionary,
    ) -> Vec<Mind> {
        let mut rng = rand::thread_rng();
        let mut output: Vec<Mind> = Vec::new();

        for mind in population {
            let parents_present = rng.gen::<f32>() > PARENT_PRESENCE_CHANCE;
            if !parents_present {
                output.push(mind);
            } else {
                let mut mind_m = mind.clone();
                let parent_age_distribution =
                    Normal::new(mind.age.clone() as f32 + 30.0, 5.0).unwrap();
                let mut parent_one = gen_mind_with_gender_and_relation(
                    &name_dict,
                    &Gender::Female,
                    parent_age_distribution.sample(&mut rand::thread_rng()) as u32,
                    vec![(RelationVerb::Child, mind.id.clone())],
                );
                parent_one.last_name = String::from(&mind_m.last_name);
                let mut parent_two = gen_mind_with_gender_and_relation(
                    &name_dict,
                    &Gender::Male,
                    parent_age_distribution.sample(&mut rand::thread_rng()) as u32,
                    vec![(RelationVerb::Child, mind.id.clone())],
                );
                parent_two.last_name = String::from(&parent_one.last_name);
                let parent_one_alive = parent_one.age
                    < Normal::new(65.0, 10.0)
                        .unwrap()
                        .sample(&mut rand::thread_rng()) as u32;
                if parent_one_alive {
                    mind_m
                        .relations
                        .push((RelationVerb::Parent, parent_one.id.clone()));
                    parent_two
                        .relations
                        .push((RelationVerb::Partner, parent_one.id.clone()));
                }
                let parent_two_alive = parent_two.age
                    < Normal::new(65.0, 10.0)
                        .unwrap()
                        .sample(&mut rand::thread_rng()) as u32;
                if parent_two_alive {
                    mind_m
                        .relations
                        .push((RelationVerb::Parent, parent_two.id.clone()));
                    parent_one
                        .relations
                        .push((RelationVerb::Partner, parent_two.id.clone()));
                }
                if parent_one_alive {
                    output.push(parent_one);
                }
                if parent_two_alive {
                    output.push(parent_two);
                }
                output.push(mind_m);
            }
        }
        return output;
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

    pub fn link_friends_within_population(population: Vec<Mind>) -> Vec<Mind> {
        let mut rng = rand::thread_rng();
        let mut population_ref = population.clone();
        let mut output: Vec<Mind> = Vec::new();
        // add outcoming friendships to the population
        for mind in population {
            let friend_count = (rng.gen::<f32>() * FRIEND_OUTGOING_MAX) as u32;
            let mut mind_m = mind.clone();
            for _i in 0..friend_count {
                let match_f = population_ref.iter().find(|m| match_friend(&mind_m, m));
                if match_f.is_some() {
                    mind_m
                        .relations
                        .push((RelationVerb::Friend, match_f.unwrap().id.clone()));
                }
                population_ref.shuffle(&mut rng);
            }
            output.push(mind_m);
        }
        // reflect those outgoing friendships back
        let output_ref = output.clone();
        for mind in output.iter_mut() {
            let incoming_friends: Vec<&Mind> = output_ref
                .iter()
                .filter(|m| {
                    m.relations
                        .iter()
                        .any(|(verb, id)| verb.eq(&RelationVerb::Friend) && id.eq(&mind.id))
                })
                .collect();
            for friend in incoming_friends {
                mind.relations
                    .push((RelationVerb::Friend, friend.id.clone()));
            }
        }
        return output;
    }

    pub fn link_colleagues(population: Population) -> Population {
        let ref_pop = population.clone();
        let mut output: Population = Vec::new();

        for m in population {
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

        return output;
    }
}
