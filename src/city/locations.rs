pub mod locations {
    // {descriptor?} {name?} {large_natural_feature} {smaller_feature}
    //  Hampton   River                 Valley
    use rand::Rng;
    use rand::seq::SliceRandom;
    use rand_distr::{Normal, Distribution};
    use uuid::Uuid;

    use crate::city::city::City;
    use crate::city::institutions::institutions::Institution;
    use crate::names::names::{NameDictionary, random_name_definition, NameTag, gen_name_dict};

    const LOCATION_MEAN_INSTITUTIONS: f32 = 10.0;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Location {
        pub id: Uuid,
        pub name: String,
        pub institutions: Vec<Uuid>
    }


    pub fn print_location(location: &Location, city: &City) -> String {
        let mut output: String = String::new();
        output.push_str("==Location=\n");
        output.push_str(&format!("Name: {}\n", location.name));
        output.push_str("Institutions: \n");
        for inst_id in &location.institutions {
            let inst = city.institutions.iter().find(|i| i.id.eq(inst_id)).unwrap();
            output.push_str(&format!("  {}\n", &inst.name));
        }
        output.push_str("===========\n");
        return output;
    }


    pub fn gen_location_name(name_dict: &NameDictionary, long: bool) -> String {
        let mut rng = rand::thread_rng();
        let has_name = rng.gen::<f32>() < 0.5;
        let has_descriptor = !has_name || rng.gen::<f32>() < 0.5;
        let has_minor_feature = long && rng.gen::<f32>() < 0.5;
        let has_major_feature = !has_minor_feature || rng.gen::<f32>() < 0.5;
        let mut output = String::new();
        if has_descriptor {
            let name = random_name_definition(&name_dict.location_descriptors);
            if name.tags.contains(&NameTag::Suffixable) {
                output.push_str(&format!("{}", name.name.to_lowercase()));
            } else {
                output.push_str(&format!(" {}", name.name));
            }
        }
        if has_name {
            let name = random_name_definition(&name_dict.last_names);
            if name.tags.contains(&NameTag::Suffixable) {
                output.push_str(&format!("{}", name.name.to_lowercase()));
            } else {
                output.push_str(&format!(" {}", name.name));
            }
        }
        if has_major_feature {
            let name = random_name_definition(&name_dict.major_features);
            if name.tags.contains(&NameTag::Suffixable) {
                output.push_str(&format!("{}", name.name.to_lowercase()));
            } else {
                output.push_str(&format!(" {}", name.name));
            }
        }

        if has_minor_feature {
            let name = random_name_definition(&name_dict.minor_features);
            if name.tags.contains(&NameTag::Suffixable) {
                output.push_str(&format!("{}", name.name.to_lowercase()));
            } else {
                output.push_str(&format!(" {}", name.name));
            }
        }

        return String::from(output.trim());
    }

    pub fn gen_location(name_dict: &NameDictionary) -> Location {
        return Location {
            id: Uuid::new_v4(),
            name: gen_location_name(&name_dict, true),
            institutions: Vec::new(),
        }
    }


    fn get_institute_count_for_area() -> usize {
        return (Normal::new(LOCATION_MEAN_INSTITUTIONS, LOCATION_MEAN_INSTITUTIONS / 2.0).unwrap().sample(&mut rand::thread_rng()) as usize).max(1);
    }
    pub fn gen_locations_from_institutions(name_dict: &NameDictionary, insts: &Vec<Institution>) -> Vec<Location> {
        let mut rng = rand::thread_rng();

        let mut output: Vec<Location> = Vec::new();
        let mut curr = gen_location(&name_dict);
        let mut remaining = get_institute_count_for_area();

        let mut institutions = insts.clone();
        institutions.shuffle(&mut rng);

        for inst in institutions {
            if remaining <= 0 {
                remaining = get_institute_count_for_area();
                output.push(curr.clone());
                curr = gen_location(&name_dict);
            }
            curr.institutions.push(inst.id.clone());
            remaining = remaining - 1;
        }
        output.push(curr);
        return output;
    }

    #[test]
    fn test_gen_location_name() {
        let name_dict = gen_name_dict();
        for _i in 0..10 {
            println!("{}", gen_location_name(&name_dict, false));
        }
    }

    // valid location names
    // descriptor name major minor
    // descriptor major minor
    // name major minor
    // descriptor name minor
}