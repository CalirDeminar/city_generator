pub mod locations {
    // {descriptor?} {name?} {large_natural_feature} {smaller_feature}
    //  Hampton   River                 Valley
    use rand::seq::SliceRandom;
    use rand_distr::{Normal, Distribution};
    use uuid::Uuid;

    use crate::city::city::City;
    use crate::city::institutions::institutions::Institution;
    use crate::names::names::{NameDictionary, gen_name_dict};
    use crate::templater::templater::render_template;
    use crate::utils::utils::random_pick;

    const LOCATION_MEAN_INSTITUTIONS: f32 = 10.0;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Location {
        pub id: Uuid,
        pub name: String,
    }


    pub fn print_location(location: &Location, city: &City) -> String {
        let mut output: String = String::new();
        output.push_str("==Location=\n");
        output.push_str(&format!("Name: {}\n", location.name));
        output.push_str("Institutions: \n");
        let institutions: Vec<&Institution> = city.institutions.iter().filter(|i| i.locationId.eq(&location.id)).collect();
        for inst in &institutions {
            output.push_str(&format!("  {}\n", &inst.name));
        }
        output.push_str("===========\n");
        return output;
    }


    pub fn gen_location_name(name_dict: &NameDictionary, long: bool) -> String {
        let long_templates = vec!["{{LocationDescriptor}}{{LastName}}{{LocationMajorFeature}}{{LocationMinorFeature}}"];
        let short_templates = vec![
            "{{LastName}}{{LocationMajorFeature}}{{LocationMinorFeature}}",
            "{{LocationDescriptor}}{{LastName}}{{LocationMinorFeature}}",
            "{{LocationDescriptor}}{{LastName}}{{LocationMajorFeature}}"
        ];
        if long {
            return render_template(random_pick(&long_templates), &name_dict.total_list);
        }
        return render_template(random_pick(&short_templates), &name_dict.total_list);
    }

    pub fn gen_location(name_dict: &NameDictionary) -> Location {
        return Location {
            id: Uuid::new_v4(),
            name: gen_location_name(&name_dict, true),
        }
    }


    fn get_institute_count_for_area() -> usize {
        return (Normal::new(LOCATION_MEAN_INSTITUTIONS, LOCATION_MEAN_INSTITUTIONS / 2.0).unwrap().sample(&mut rand::thread_rng()) as usize).max(1);
    }

    #[test]
    fn test_gen_location_name() {
        let name_dict = gen_name_dict();
        for _i in 0..10 {
            println!("{}", gen_location_name(&name_dict, true));
        }
    }

    // valid location names
    // descriptor name major minor
    // descriptor major minor
    // name major minor
    // descriptor name minor
}