pub mod friends;
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
        LatePartner,
        Spouse,
        ExSpouse,
        LateSpouse,
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
        let match_relation = mind.relations.iter().find(|(r, _id)| r.eq(&relation));
        if match_relation.is_some() {
            let (_verb, id) = match_relation.unwrap();
            return population.get(id);
        }
        return None;
    }

    pub fn find_relation_minor<'a>(
        mind: &Mind,
        relation: RelationVerb,
        population: &'a Population,
    ) -> Option<&'a Mind> {
        let match_relation: Vec<&(RelationVerb, Uuid)> = mind
            .relations
            .iter()
            .filter(|(r, _id)| r.eq(&relation))
            .collect();
        for (_relation, id) in match_relation {
            let rel = population.get(id);
            if rel.is_some() {
                let relation = rel.unwrap();
                if relation.age < ADULT_AGE_FROM {
                    return Some(relation);
                }
            }
        }
        return None;
    }

    pub fn link_colleagues<'a>(city: &'a mut City) -> &'a mut City {
        let ref_citizens = city.citizens.clone();
        let ref_ids = ref_citizens.keys();
        for m_id in ref_ids {
            let mut mind = city.citizens.get_mut(m_id).unwrap();
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
}
