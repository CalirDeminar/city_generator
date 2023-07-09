pub mod partners {
    use rand::seq::SliceRandom;
    use std::{collections::HashMap, ops::Range};

    use rand::Rng;
    use uuid::Uuid;

    use crate::city::{
        city::City,
        population::{
            mind::{
                mind::*,
                relations::{
                    friends::friends::SOCIAL_RELATIONS,
                    parental_naming_formats::parental_naming_formats::get_new_couple_last_names,
                    relations::*,
                },
            },
            population::Population,
        },
    };

    const PARTNER_CHANCE_GENERAL: f32 = 0.3; // multiple annual chances
    const PARTNER_MARRIAGE_RATE: f32 = 0.075; // single anunal chance
    const PARTNER_SPLIT_RATE: f32 = 0.06; // single annual chance
    const MARRIAGE_SPLIT_RATE: f32 = 0.03; // single annual chance

    const MAX_RELATION_AGE_DIFF: u32 = 20;
    pub const TAKEN_VERBS: [RelationVerb; 2] = [RelationVerb::Partner, RelationVerb::Spouse];
    // const EX_VERBS: [RelationVerb; 2] = [RelationVerb::ExPartner, RelationVerb::ExSpouse];

    fn flatten_rel_map(input: &Vec<(Uuid, Uuid)>) -> Vec<Uuid> {
        return input
            .iter()
            .map(|(a, b)| vec![a, b])
            .flatten()
            .map(|id| id.clone())
            .collect();
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

    fn is_single(mind: &Mind) -> bool {
        return TAKEN_VERBS
            .iter()
            .all(|v| !mind.relations.iter().any(|(mv, _id)| mv.eq(&v)));
    }

    fn compatible_sexuality(input: &Sexuality) -> Vec<Sexuality> {
        return match input {
            &Sexuality::Homosexual => vec![Sexuality::Homosexual, Sexuality::Bisexual],
            &Sexuality::Bisexual => vec![Sexuality::Homosexual, Sexuality::Bisexual],
            &Sexuality::Hetrosexual => vec![Sexuality::Hetrosexual],
            _ => Vec::new(),
        };
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

    fn determine_age_range(mind: &Mind, max_age_gap: u32) -> Range<u32> {
        let underflowing_min_age = (mind.age as i32 - max_age_gap as i32) < ADULT_AGE_FROM as i32;
        let min_age = if underflowing_min_age {
            ADULT_AGE_FROM as u32
        } else {
            mind.age - max_age_gap
        };
        return min_age..(mind.age + max_age_gap);
    }

    fn search_for_partner<'a>(
        population: &'a Population,
        target_gender: &Gender,
        age_range: Range<u32>,
        compatible_sexualities: Vec<Sexuality>,
        to_ignore: &Vec<Uuid>,
    ) -> Option<&'a Mind> {
        let mut rng = rand::thread_rng();
        let mut filtered: Vec<&Mind> = population
            .values()
            .filter(|c| is_single(&c))
            .filter(|c| c.gender.eq(&target_gender))
            .filter(|c| age_range.contains(&c.age))
            .filter(|c| {
                return !to_ignore.iter().any(|id| id.eq(&c.id));
            })
            .filter(|c| compatible_sexualities.contains(&c.sexuality))
            .collect();
        filtered.shuffle(&mut rng);
        if rng.gen::<f32>() > PARTNER_CHANCE_GENERAL {
            return None;
        }
        for mind in filtered {
            return Some(mind);
        }
        return None;
    }

    fn find_partner_id(
        mind: &Mind,
        population: &Population,
        to_ignore: &Vec<Uuid>,
    ) -> Option<Uuid> {
        let mut rng = rand::thread_rng();
        let target_gender = determine_partner_gender(&mind);
        let range_roll = rng.gen::<f32>();
        for i in 0..MAX_RELATION_AGE_DIFF {
            let age_range = determine_age_range(&mind, (range_roll * i as f32) as u32);
            let possible_partner = search_for_partner(
                population,
                &target_gender,
                age_range,
                compatible_sexuality(&mind.sexuality),
                to_ignore,
            );
            if possible_partner.is_some() {
                return Some(possible_partner.unwrap().id);
            }
        }
        return None;
    }

    pub fn link_partners<'a>(city: &'a mut City) -> &'a mut City {
        let citizen_ids: Vec<Uuid> = city
            .citizens
            .values()
            .filter(|c| c.alive)
            .map(|c| c.id)
            .collect();

        let mut relations_to_add: Vec<(Uuid, Uuid)> = Vec::new();

        for mind_id in citizen_ids {
            if !flatten_rel_map(&relations_to_add).contains(&mind_id) {
                // city.citizens.shuffle(&mut rng);

                let mind = city.citizens.get(&mind_id).unwrap();

                let mut taken_list = flatten_rel_map(&relations_to_add);
                taken_list.push(mind.id.clone());
                let possible_partner_id = find_partner_id(&mind, &city.citizens, &taken_list);
                if possible_partner_id.is_some() {
                    let root_repeating = flatten_rel_map(&relations_to_add)
                        .iter()
                        .any(|c| c.eq(&mind.id));
                    let parnet_repeating = flatten_rel_map(&relations_to_add)
                        .iter()
                        .any(|c| c.eq(&possible_partner_id.unwrap()));
                    if !root_repeating && !parnet_repeating {
                        relations_to_add
                            .push((mind.id.clone(), possible_partner_id.unwrap().clone()));
                    } else {
                        println!("Repeating Partner");
                    }
                }
            }
        }

        // let used_ids: Vec<Uuid> = Vec::new();
        // relations_to_add = relations_to_add.iter().collect();

        for (id_1, id_2) in relations_to_add {
            let citizens = city.citizens.values_mut().filter(|c| c.alive);
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
                let mind_1_mut = mind_1.unwrap();
                let mind_2_mut = mind_2.unwrap();

                mind_1_mut.relations.push((verb.clone(), id_2.clone()));
                mind_1_mut.activity_log.push(format!(
                    "Gained {} {} as a {}",
                    mind_2_mut.first_name, mind_2_mut.last_name, verb
                ));

                mind_2_mut.relations.push((verb.clone(), id_1.clone()));
                mind_2_mut.activity_log.push(format!(
                    "Gained {} {} as a {}",
                    mind_1_mut.first_name, mind_1_mut.last_name, verb
                ));
            } else {
                println!("Mind Lookup Failed");
            }
        }
        return city;
    }

    pub fn link_partners_by_year<'a>(city: &'a mut City) -> &'a mut City {
        let citizen_ids: Vec<Uuid> = city
            .citizens
            .values()
            .filter(|c| c.alive && is_single(c))
            .map(|c| c.id)
            .collect();

        let mut relations_to_add: Vec<(Uuid, Uuid)> = Vec::new();
        let mut taken_list = Vec::new();

        for mind_id in citizen_ids {
            if !flatten_rel_map(&relations_to_add).contains(&mind_id) {
                // city.citizens.shuffle(&mut rng);

                let mind = city.citizens.get(&mind_id).unwrap();

                let friend_ids: Vec<&Uuid> = mind
                    .relations
                    .iter()
                    .filter(|(v, _id)| SOCIAL_RELATIONS.contains(v))
                    .map(|(_v, id)| id)
                    .collect();

                let mut friends: HashMap<Uuid, Mind> = HashMap::new();
                for id in friend_ids {
                    friends.insert(id.clone(), city.citizens.get(&id).unwrap().clone());
                }

                let possible_partner_id = find_partner_id(
                    &mind,
                    &friends,
                    &vec![taken_list.clone(), vec![mind.id.clone()]].concat(),
                );
                if possible_partner_id.is_some() {
                    let root_repeating = taken_list.iter().any(|c| c.eq(&mind.id));
                    let parnet_repeating = taken_list
                        .iter()
                        .any(|c| c.eq(&possible_partner_id.unwrap()));
                    if !root_repeating && !parnet_repeating {
                        relations_to_add
                            .push((mind.id.clone(), possible_partner_id.unwrap().clone()));

                        taken_list.push(mind.id.clone());
                        taken_list.push(possible_partner_id.unwrap().clone());
                    } else {
                        println!("Repeating Partner");
                    }
                }
            }
        }

        for (id_1, id_2) in relations_to_add {
            let m1 = city.citizens.get(&id_1).unwrap().clone();
            let m2 = city.citizens.get(&id_2).unwrap().clone();
            let mind_1 = city.citizens.get_mut(&id_1).unwrap();
            mind_1.relations.push((RelationVerb::Partner, id_2.clone()));
            mind_1.activity_log.push(format!(
                "Gained {} {} as a Partner in year {}",
                m2.first_name, m2.last_name, city.year
            ));
            drop(mind_1);
            let mind_2 = city.citizens.get_mut(&id_2).unwrap();
            mind_2.relations.push((RelationVerb::Partner, id_1.clone()));
            mind_2.activity_log.push(format!(
                "Gained {} {} as a Partner in year {}",
                m1.first_name, m1.last_name, city.year
            ));
            drop(mind_2);
        }
        return city;
    }

    pub fn update_partners_by_year<'a>(city: &'a mut City) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let citizen_ids: Vec<Uuid> = city
            .citizens
            .values()
            .filter(|c| c.alive)
            .map(|c| c.id)
            .collect();
        for id in citizen_ids {
            let mut citizens = city.citizens.values_mut().filter(|c| c.alive);
            let mind = citizens.find(|c| c.id.eq(&id)).unwrap();
            if !is_single(mind) {
                let relations_ref = mind.relations.clone();
                let (verb, partner_id) = relations_ref
                    .iter()
                    .find(|(v, _rid)| TAKEN_VERBS.contains(&v))
                    .unwrap();
                let p = citizens.find(|m| m.id.eq(&partner_id));
                if p.is_some() {
                    let partner = p.unwrap();
                    match verb {
                        RelationVerb::Partner => {
                            if rng.gen::<f32>() < PARTNER_SPLIT_RATE {
                                mind.relations
                                    .retain(|(v, id)| !(v.eq(&verb) && id.eq(&partner_id)));
                                mind.relations
                                    .push((RelationVerb::ExPartner, partner_id.clone()));
                                mind.activity_log.push(format!(
                                    "Broke up with Partner {} {} in year {}",
                                    partner.first_name, partner.last_name, city.year
                                ));

                                partner
                                    .relations
                                    .retain(|(v, id)| !(v.eq(&verb) && id.eq(&mind.id)));
                                partner
                                    .relations
                                    .push((RelationVerb::ExPartner, mind.id.clone()));
                                partner.activity_log.push(format!(
                                    "Broke up with Partner {} {} in year {}",
                                    mind.first_name, mind.last_name, city.year
                                ));
                            } else if rng.gen::<f32>() < PARTNER_MARRIAGE_RATE {
                                let (mind_last_name, partner_last_name) =
                                    get_new_couple_last_names(&mind, &partner, &city.culture);
                                mind.last_name = mind_last_name;
                                partner.last_name = partner_last_name;
                                mind.relations
                                    .retain(|(v, id)| !(v.eq(&verb) && id.eq(&partner_id)));
                                mind.relations
                                    .push((RelationVerb::Spouse, partner_id.clone()));
                                mind.activity_log.push(format!(
                                    "Married Partner {} {} in year {}",
                                    partner.first_name, partner.last_name, city.year
                                ));

                                partner
                                    .relations
                                    .retain(|(v, id)| !(v.eq(&verb) && id.eq(&mind.id)));
                                partner
                                    .relations
                                    .push((RelationVerb::Spouse, mind.id.clone()));
                                partner.activity_log.push(format!(
                                    "Married Partner {} {} in year {}",
                                    mind.first_name, mind.last_name, city.year
                                ));
                            }
                        }
                        RelationVerb::Spouse => {
                            if rng.gen::<f32>() < MARRIAGE_SPLIT_RATE {
                                let mind_left = rng.gen::<f32>() < 0.5;
                                mind.relations
                                    .retain(|(v, id)| !(v.eq(&verb) && id.eq(&partner_id)));
                                mind.relations
                                    .push((RelationVerb::ExSpouse, partner_id.clone()));
                                mind.activity_log.push(format!(
                                    "Broke up with Spouse {} {} in year {}",
                                    partner.first_name, partner.last_name, city.year
                                ));
                                if mind_left {
                                    mind.residence = None;
                                } else {
                                    partner.residence = None;
                                }

                                partner
                                    .relations
                                    .retain(|(v, id)| !(v.eq(&verb) && id.eq(&mind.id)));
                                partner
                                    .relations
                                    .push((RelationVerb::ExSpouse, mind.id.clone()));
                                partner.activity_log.push(format!(
                                    "Broke up with Spouse {} {} in year {}",
                                    mind.first_name, mind.last_name, city.year
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        return city;
    }
}
