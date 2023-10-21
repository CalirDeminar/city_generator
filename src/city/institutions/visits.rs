pub mod visits {
    use crate::city::{
        city::City, institutions::institutions::InstituteType, population::mind::mind::Mind,
    };
    use rand::Rng;
    use uuid::Uuid;
    // use rand::seq::SliceRandom;

    // const HABIT_FRACTION: f32 = 0.1;

    const SOCIAL_HABIT_LIMIT: usize = 5;
    const SHOPPING_HABIT_LIMIT: usize = 10;

    #[derive(PartialEq)]
    pub enum VisitType {
        Shopping,
        Social,
    }

    pub fn get_habitual_institutions<'a>(
        mind: &'a Mind,
        visit_type: &'a VisitType,
    ) -> (Vec<&'a Uuid>, f32) {
        let total_visits: usize = if visit_type.eq(&VisitType::Shopping) {
            &mind.institution_shopping_visits
        } else {
            &mind.institution_social_visits
        }
        .values()
        .sum();

        if total_visits.eq(&0) {
            return (Vec::new(), 0.0);
        }

        let mut visits: Vec<(&Uuid, &usize)> = if visit_type.eq(&VisitType::Shopping) {
            &mind.institution_shopping_visits
        } else {
            &mind.institution_social_visits
        }
        .iter()
        .collect();
        let max_limit = if visit_type.eq(&VisitType::Shopping) {
            SHOPPING_HABIT_LIMIT
        } else {
            SOCIAL_HABIT_LIMIT
        };
        let limit = if visits.len() > max_limit {
            max_limit
        } else {
            visits.len()
        };

        visits.sort_by(|a, b| a.1.cmp(&b.1));
        let top_ten: Vec<&Uuid> = visits[0..limit].iter().map(|(id, _c)| *id).collect();
        let top_ten_sum = visits[0..limit].iter().fold(0, |acc, i| i.1 + acc);
        return (top_ten, (top_ten_sum as f32 / total_visits as f32));
    }

    fn calculate_annual_visits_for_mind<'a>(
        city: &'a mut City,
        mind_id: &Uuid,
        institutions: &Vec<&Uuid>,
        visit_type: &VisitType,
    ) {
        let mut rng = rand::thread_rng();

        let mind = city.citizens.get_mut(mind_id).unwrap();
        if !mind.alive {
            return;
        }
        let mind_clone = mind.clone();

        let inst_keys: &Vec<&Uuid> = institutions;
        let (habitual_keys, habit_scale) = get_habitual_institutions(&mind_clone, visit_type);

        if inst_keys.len() < 5 {
            return;
        }

        let visit_count = (rng.gen::<f32>() * 365.0).round() as usize;
        // TODO - this powf value be related to some mind personality value "amount somebody sticks to a habit"
        let habit_visit_odds = habit_scale.powf(0.5);
        for _i in 0..visit_count {
            let inst_key = if rng.gen::<f32>() < habit_visit_odds && habitual_keys.len() > 0 {
                habitual_keys[(rng.gen::<f32>() * (habitual_keys.len() as f32)) as usize]
            } else {
                inst_keys[(rng.gen::<f32>() * (inst_keys.len() as f32)) as usize]
            };
            let inst = city.institutions.get_mut(inst_key).unwrap();

            let count_target = if visit_type.eq(&VisitType::Shopping) {
                mind.institution_shopping_visits.clone()
            } else {
                mind.institution_social_visits.clone()
            };

            let old_count = if count_target.contains_key(&inst.id) {
                count_target.get(&inst.id).unwrap().clone()
            } else {
                0
            };
            drop(count_target);

            if visit_type.eq(&VisitType::Shopping) {
                mind.institution_shopping_visits
                    .insert(inst.id.clone(), old_count + 1);
            } else {
                mind.institution_social_visits
                    .insert(inst.id.clone(), old_count + 1);
            }
            inst.annual_visits += 1;

            drop(inst);
        }
        drop(mind);
    }

    pub fn run_citizen_shopping<'a>(city: &'a mut City) -> &'a mut City {
        let citizens = city.citizens.clone();
        let insts = city.institutions.clone();
        let shopping_institutions: Vec<&Uuid> = insts
            .values()
            .clone()
            .filter(|i| {
                vec![
                    InstituteType::GeneralRetail,
                    InstituteType::SpecialistFoodService,
                    InstituteType::SpecialistRetail,
                ]
                .contains(&i.institute_type)
            })
            .map(|inst| &inst.id)
            .collect();
        for mind_id in citizens.keys() {
            calculate_annual_visits_for_mind(
                city,
                mind_id,
                &shopping_institutions,
                &VisitType::Shopping,
            );
        }
        return city;
    }

    pub fn run_citizen_social<'a>(city: &'a mut City) -> &'a mut City {
        let citizens = city.citizens.clone();
        let insts = city.institutions.clone();
        let shopping_institutions: Vec<&Uuid> = insts
            .values()
            .clone()
            .filter(|i| {
                vec![
                    InstituteType::FoodService,
                    InstituteType::EntertainmentVenue,
                ]
                .contains(&i.institute_type)
            })
            .map(|inst| &inst.id)
            .collect();
        for mind_id in citizens.keys() {
            calculate_annual_visits_for_mind(
                city,
                mind_id,
                &shopping_institutions,
                &VisitType::Social,
            );
        }
        return city;
    }
}
