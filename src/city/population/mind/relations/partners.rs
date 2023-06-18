pub mod partners {
    use rand::seq::SliceRandom;
    use std::ops::Range;

    use rand::Rng;
    use uuid::Uuid;

    use crate::city::{
        city::City,
        population::{
            mind::{mind::*, relations::relations::*},
            population::Population,
        },
    };

    const PARTNER_CHANCE_GENERAL: f32 = 0.8;
    const PARTNER_MARRIAGE_RATE: f32 = 0.5;
    const PARTNER_SPLIT_RATE: f32 = 0.2;
    const MAX_RELATION_AGE_DIFF: u32 = 20;

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
            .iter()
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
        let mut rng = rand::thread_rng();
        let citizen_ids: Vec<Uuid> = city.citizens.iter().map(|c| c.id).collect();

        let mut relations_to_add: Vec<(Uuid, Uuid)> = Vec::new();

        for mind_id in citizen_ids {
            if !flatten_rel_map(&relations_to_add).contains(&mind_id) {
                city.citizens.shuffle(&mut rng);

                let mind = city.citizens.iter().find(|c| c.id.eq(&mind_id)).unwrap();

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
}
