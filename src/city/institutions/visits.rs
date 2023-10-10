pub mod visits {
    use rand_distr::num_traits::ToPrimitive;
    use uuid::Uuid;
    use crate::city::{city::City, population::mind::mind::Mind};
    use rand::Rng;
    // use rand::seq::SliceRandom;

    const HABIT_FRACTION: f32 = 0.1;

    pub fn get_habitual_institutions(mind: &Mind) -> (Vec<&Uuid>, usize) {
        let total_visits: usize = mind.institution_visits.values().sum();
        if total_visits.eq(&0){
            return (Vec::new(), 0);
        }
        return (mind.institution_visits
            .iter()
            .filter(|(_id, c)| ((c.to_f32().unwrap()) / (total_visits as f32)) > HABIT_FRACTION)
            .map(|(id, _c)| id).collect(), total_visits);
    }

    fn calculate_annual_visits_for_mind<'a>(city: &'a mut City, mind_id: &Uuid) {
        let mut rng = rand::thread_rng();

        let mind = city.citizens.get_mut(mind_id).unwrap();
        if !mind.alive {
            return;
        }
        let mind_clone = mind.clone();
        let city_inst_clone = city.institutions.clone();

        let inst_keys: Vec<&Uuid> = city_inst_clone.keys().into_iter().collect();
        let (habitual_keys, habit_scale) = get_habitual_institutions(&mind_clone);

        if inst_keys.len() < 5 {
            return;
        }

        let visit_count = (rng.gen::<f32>() * 365.0).round() as usize;
        let habit_visit_odds = if habit_scale > 0 { ((habit_scale as f32 / mind.age as f32) / visit_count as f32).powf(0.5) } else { 0.0 };
        println!("Habit Odds: {}", habit_visit_odds);
        for _i in 0..visit_count {
            let inst_key = if rng.gen::<f32>() < habit_visit_odds && habitual_keys.len() > 0 { 
                habitual_keys[((rng.gen::<f32>() * habitual_keys.len() as f32) as usize)] 
            } else {
                inst_keys[((rng.gen::<f32>() * inst_keys.len() as f32) as usize)]
            };
            let inst = city.institutions.get_mut(inst_key).unwrap();

            let old_count = if mind.institution_visits.contains_key(&inst.id) {mind.institution_visits.get(&inst.id).unwrap() } else { &0 };
            
            mind.institution_visits.insert(inst.id.clone(), old_count + 1);
            inst.annual_visits += 1;

            drop(inst);
        }
        drop(mind);
    }

    pub fn run_citizen_shopping<'a>(city: &'a mut City) -> &'a mut City {
        let citizens = city.citizens.clone();
        for mind_id in citizens.keys() {
            calculate_annual_visits_for_mind(city, mind_id);
        }
        return city;
    }
}