pub mod friends;
pub mod parental_naming_formats;
pub mod parents;
pub mod partners;
pub mod relations {
    use crate::city::city::City;
    use crate::city::population::{mind::mind::*, population::Population};
    // use rand::seq::SliceRandom;
    use rand::Rng;
    use strum_macros::Display;
    use uuid::Uuid;

    #[derive(PartialEq, Debug, Clone, Display)]
    pub enum RelationVerb {
        // family
        Parent,
        Child,
        Partner,
        ExPartner,
        LatePartner,
        Spouse,
        ExSpouse,
        LateSpouse,
        Sibling,
        Grandparent,
        Grandchild,
        Cousin,
        Pibling, // Aunt/Uncle
        Nibling, // Neice/Nephew
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
        population: &'a Population,
    ) -> Option<&'a Mind> {
        let match_relations: Vec<&(RelationVerb, Uuid)> = mind
            .relations
            .iter()
            .filter(|(r, _id)| r.eq(&relation))
            .collect();
        for (_verb, id) in match_relations {
            let relation = population.get(id);
            if relation.is_some() && relation.unwrap().alive {
                return relation;
            }
        }
        return None;
    }

    pub fn find_relation_minor<'a>(
        mind: &Mind,
        relation: RelationVerb,
        population: &'a Population,
    ) -> Option<&'a Mind> {
        let match_relations: Vec<&(RelationVerb, Uuid)> = mind
            .relations
            .iter()
            .filter(|(r, _id)| r.eq(&relation))
            .collect();
        for (_verb, id) in match_relations {
            let relation = population.get(id);
            if relation.is_some()
                && relation.unwrap().alive
                && relation.unwrap().age < ADULT_AGE_FROM
            {
                return relation;
            }
        }
        return None;
    }

    pub fn link_colleagues<'a>(city: &'a mut City) -> &'a mut City {
        let ref_citizens = city.citizens.clone();
        let ref_ids = ref_citizens.keys();
        for m_id in ref_ids {
            let mind = city.citizens.get_mut(m_id).unwrap();
            mind.relations
                .retain(|(v, _id)| !v.eq(&RelationVerb::Colleague));
            if mind.employer.is_some() {
                let colleagues: Vec<&Mind> = ref_citizens
                    .values()
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
        }

        return city;
    }

    pub fn link_siblings<'a>(city: &'a mut City) -> &'a mut City {
        let ref_pop = city.citizens.clone();

        for m in ref_pop.values() {
            let parents: Vec<Mind> = m
                .relations
                .iter()
                .filter(|(v, _id)| v.eq(&RelationVerb::Parent))
                .map(|(_v, id)| ref_pop.get(id).unwrap().clone())
                .collect();
            let siblings: Vec<Uuid> = parents
                .iter()
                .flat_map(|m| m.relations.clone())
                .filter(|(verb, _id)| verb.eq(&RelationVerb::Child))
                .map(|(_v, id)| id)
                .collect();
            for sibling in siblings {
                let m = city.citizens.get_mut(&m.id).unwrap();
                if !m
                    .relations
                    .iter()
                    .any(|(v, rel)| v.eq(&RelationVerb::Sibling) && rel.eq(&sibling))
                {
                    m.relations.push((RelationVerb::Sibling, sibling));
                }
                drop(m);
            }
        }

        return city;
    }

    pub fn link_grandparents<'a>(city: &'a mut City) -> &'a mut City {
        let ref_pop = city.citizens.clone();
        let ref_ids = ref_pop.keys();
        for id in ref_ids {
            let mut citizens = city.citizens.values_mut();
            let m = citizens.find(|c| c.id.eq(id)).unwrap();

            let parents: Vec<&Mind> = m
                .relations
                .iter()
                .filter(|(v, _id)| v.eq(&RelationVerb::Parent))
                .map(|(_v, id)| ref_pop.get(id).unwrap())
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

    pub fn invert_relation(verb: &RelationVerb) -> Option<RelationVerb> {
        return match verb {
            RelationVerb::Parent => Some(RelationVerb::Child),
            RelationVerb::Child => Some(RelationVerb::Parent),
            RelationVerb::Sibling => Some(RelationVerb::Sibling),
            RelationVerb::Nibling => Some(RelationVerb::Pibling),
            RelationVerb::Pibling => Some(RelationVerb::Nibling),
            RelationVerb::Grandparent => Some(RelationVerb::Grandchild),
            RelationVerb::Grandchild => Some(RelationVerb::Grandparent),
            _ => None,
        };
    }

    pub fn link_family_at_birth<'a>(city: &'a mut City, child: &'a mut Mind) -> &'a mut City {
        let parents: Vec<Mind> = child
            .relations
            .iter()
            .filter(|(v, _id)| v.eq(&RelationVerb::Parent))
            .map(|(_v, id)| city.citizens.get(&id).unwrap().clone())
            .collect();
        for parent in parents {
            for (verb, id) in parent.relations.iter().filter(|(_v, id)| !id.eq(&child.id)) {
                match verb {
                    RelationVerb::Child => {
                        // Create Siblings
                        if !child
                            .relations
                            .contains(&(RelationVerb::Sibling, id.clone()))
                        {
                            let sibling = city.citizens.get_mut(&id).unwrap();
                            if sibling.alive {
                                child.relations.push((RelationVerb::Sibling, id.clone()));
                                sibling
                                    .relations
                                    .push((RelationVerb::Sibling, child.id.clone()));
                                sibling.activity_log.push(format!(
                                    "Gained {} {} as a Sibling in year {}",
                                    child.first_name, child.last_name, city.year
                                ));
                            }
                            drop(sibling);
                        }
                    }
                    RelationVerb::Sibling => {
                        // Create Pib/Nib-lings
                        if !child
                            .relations
                            .contains(&(RelationVerb::Pibling, id.clone()))
                        {
                            // direct
                            let pibling = city.citizens.get_mut(&id).unwrap();
                            if pibling.alive {
                                child.relations.push((RelationVerb::Pibling, id.clone()));
                                pibling
                                    .relations
                                    .push((RelationVerb::Nibling, child.id.clone()));
                                pibling.activity_log.push(format!(
                                    "Gained {} {} as a Nibling in year {}",
                                    child.first_name, child.last_name, city.year
                                ));
                            }
                            let pibling_relations = pibling.relations.clone();
                            drop(pibling);

                            // pibling spouse
                            let pibling_spouse_option = pibling_relations
                                .iter()
                                .find(|(v, _id)| v.eq(&RelationVerb::Spouse));

                            if pibling_spouse_option.is_some() {
                                let pibling_spouse = city
                                    .citizens
                                    .get_mut(&pibling_spouse_option.unwrap().1)
                                    .unwrap();
                                if pibling_spouse.alive {
                                    child
                                        .relations
                                        .push((RelationVerb::Pibling, pibling_spouse.id.clone()));
                                    pibling_spouse
                                        .relations
                                        .push((RelationVerb::Nibling, child.id.clone()));
                                    pibling_spouse.activity_log.push(format!(
                                        "Gained {} {} as a Nibling in year {}",
                                        child.first_name, child.last_name, city.year
                                    ));
                                }
                            }
                        }
                        // println!("Add Pibling");
                    }
                    RelationVerb::Parent => {
                        // Create Grandparent

                        let grandparent = city.citizens.get_mut(&id).unwrap();
                        if grandparent.alive {
                            child
                                .relations
                                .push((RelationVerb::Grandparent, id.clone()));
                            grandparent
                                .relations
                                .push((RelationVerb::Grandchild, child.id.clone()));
                            grandparent.activity_log.push(format!(
                                "Gained {} {} as a Grandchild in year {}",
                                child.first_name, child.last_name, city.year
                            ));
                        }
                        // println!("Add Grandparent");
                    }
                    RelationVerb::Nibling => {
                        // Create Cousin

                        let cousin = city.citizens.get_mut(&id).unwrap();
                        if cousin.alive {
                            if !child.relations.contains(&(RelationVerb::Cousin, *id)) {
                                child.relations.push((RelationVerb::Cousin, id.clone()));
                            }
                            if !cousin.relations.contains(&(RelationVerb::Cousin, child.id)) {
                                cousin
                                    .relations
                                    .push((RelationVerb::Cousin, child.id.clone()));
                                cousin.activity_log.push(format!(
                                    "Gained {} {} as a Cousin in year {}",
                                    child.first_name, child.last_name, city.year
                                ));
                            }
                        }
                        drop(cousin);
                    }
                    _ => {}
                }
            }
        }
        return city;
    }
}
