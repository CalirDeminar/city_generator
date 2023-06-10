pub mod locations {
    // {descriptor?} {name?} {large_natural_feature} {smaller_feature}
    //  Hampton   River                 Valley
    use html_builder::*;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use rand_distr::{Distribution, Normal};
    use std::fmt::Write as fmtWrite;
    use uuid::Uuid;

    use crate::city::building::building::{print_building, Building};
    use crate::city::city::City;
    use crate::city::institutions::institutions::Institution;
    use crate::names::names::{gen_name_dict, NameDictionary};
    use crate::templater::templater::render_template;
    use crate::utils::utils::random_pick;

    const LOCATION_MEAN_INSTITUTIONS: f32 = 10.0;

    #[derive(PartialEq, Debug, Clone)]
    pub struct Location {
        pub id: Uuid,
        pub name: String,
        pub size: usize,
    }

    pub fn print_location(location: &Location, city: &City) -> String {
        let mut output: String = String::new();
        output.push_str("==Location=\n");
        output.push_str(&format!("Name: {}\n", location.name));
        output.push_str("Buildings: \n");
        let buildings: Vec<&Building> = city
            .buildings
            .iter()
            .filter(|b| b.location_id.is_some() && b.location_id.unwrap().eq(&location.id))
            .collect();
        for building in &buildings {
            output.push_str(&print_building(building, city));
        }
        output.push_str("===========\n");
        return output;
    }

    pub fn print_location_html<'a>(
        node: &'a mut Node<'a>,
        location: &Location,
        city: &City,
    ) -> &'a mut Node<'a> {
        let buildings: Vec<&Building> = city
            .buildings
            .iter()
            .filter(|b| b.location_id.is_some() && b.location_id.unwrap().eq(&location.id))
            .collect();

        let mut list_element = node.div().attr(&format!("id='{}'", location.id));
        writeln!(list_element.h3(), "Location: {}", location.name).unwrap();
        writeln!(list_element.p(), "Institutions: ").unwrap();
        let mut inst_list = list_element.ul();
        for building in buildings {
            let mut element = inst_list.li();
            writeln!(
                element.a().attr(&format!("href='#{}'", building.id)),
                "{}",
                building.name
            )
            .unwrap();
        }
        return node;
    }

    pub fn gen_location_name(name_dict: &NameDictionary, long: bool) -> String {
        let long_templates = vec![
            "{{LocationDescriptor}}{{LastName}}{{LocationMajorFeature}}{{LocationMinorFeature}}",
            "{{LastName}}{{LocationMajorFeature}}{{LocationMinorFeature}}",
            "{{LocationDescriptor}}{{LastName}}{{LocationMinorFeature}}",
            "{{LocationDescriptor}}{{LastName}}{{LocationMajorFeature}}",
        ];
        let short_templates = vec![
            "{{LocationDescriptor}}{{LastName}}",
            "{{LastName}}{{LocationMinorFeature}}",
            "{{LocationDescriptor}}{{LocationMinorFeature}}",
        ];
        if long {
            return render_template(random_pick(&long_templates), &name_dict.total_list);
        }
        return render_template(random_pick(&short_templates), &name_dict.total_list);
    }

    pub fn gen_location(name_dict: &NameDictionary) -> Location {
        let mut rng = rand::thread_rng();
        return Location {
            id: Uuid::new_v4(),
            name: gen_location_name(&name_dict, false),
            size: ((rng.gen::<f32>() * 10.0) as i32).max(1) as usize,
        };
    }

    fn get_institute_count_for_area() -> usize {
        return (Normal::new(LOCATION_MEAN_INSTITUTIONS, LOCATION_MEAN_INSTITUTIONS / 2.0)
            .unwrap()
            .sample(&mut rand::thread_rng()) as usize)
            .max(1);
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
