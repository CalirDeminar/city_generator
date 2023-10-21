pub mod visits {
    use crate::city::{
        city::City, institutions::institutions::InstituteType, population::mind::mind::Mind,
    };
    use rand::Rng;
    use uuid::Uuid;
    // use rand::seq::SliceRandom;

    // const HABIT_FRACTION: f32 = 0.1;

    pub fn get_habitual_institutions(mind: &Mind) -> (Vec<&Uuid>, f32) {
        let total_visits: usize = mind.institution_visits.values().sum();

        if total_visits.eq(&0) {
            return (Vec::new(), 0.0);
        }

        let mut visits: Vec<(&Uuid, &usize)> = mind.institution_visits.iter().collect();
        let limit = if visits.len() > 10 { 10 } else { visits.len() };

        visits.sort_by(|a, b| a.1.cmp(&b.1));
        let top_ten: Vec<&Uuid> = visits[0..limit].iter().map(|(id, _c)| *id).collect();
        let top_ten_sum = visits[0..limit].iter().fold(0, |acc, i| i.1 + acc);
        return (top_ten, (top_ten_sum as f32 / total_visits as f32));
    }

    fn calculate_annual_visits_for_mind<'a>(
        city: &'a mut City,
        mind_id: &Uuid,
        institutions: &Vec<&Uuid>,
    ) {
        let mut rng = rand::thread_rng();

        let mind = city.citizens.get_mut(mind_id).unwrap();
        if !mind.alive {
            return;
        }
        let mind_clone = mind.clone();

        let inst_keys: &Vec<&Uuid> = institutions;
        let (habitual_keys, habit_scale) = get_habitual_institutions(&mind_clone);

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

            let old_count = if mind.institution_visits.contains_key(&inst.id) {
                mind.institution_visits.get(&inst.id).unwrap()
            } else {
                &0
            };

            mind.institution_visits
                .insert(inst.id.clone(), old_count + 1);
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
                    InstituteType::FoodService,
                    InstituteType::GeneralRetail,
                    InstituteType::SpecialistRetail,
                    InstituteType::EntertainmentVenue,
                ]
                .contains(&i.institute_type)
            })
            .map(|inst| &inst.id)
            .collect();
        for mind_id in citizens.keys() {
            calculate_annual_visits_for_mind(city, mind_id, &shopping_institutions);
        }
        return city;
    }
}
