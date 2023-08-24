pub mod locations {
    // {descriptor?} {name?} {large_natural_feature} {smaller_feature}
    //  Hampton   River                 Valley
    use html_builder::*;
    
    use rand::Rng;
    use std::fmt::Write as fmtWrite;
    use uuid::Uuid;

    use crate::city::building::building::{print_building, print_building_html, Building};
    use crate::city::city::City;
    
    use crate::language::language::*;
    use crate::templater::templater::*;
    use crate::utils::utils::random_pick;


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
        writeln!(list_element.h3(), "{}", location.name).unwrap();
        writeln!(list_element.h4(), "Buildings: ").unwrap();
        let mut building_list = list_element.ul();
        for building in buildings {
            let mut element = building_list.li();
            print_building_html(&mut element, &building, &city);
        }
        return node;
    }

    pub fn gen_location_name(dict: &Vec<Word>, long: bool, era: &Option<Era>) -> String {
        let long_templates = vec![
            "{{{{Adjective(Position, Quality, Age, Colour)}} {{Noun(!LastName, !HistoricalFigure))}} {{Noun(GeographyFeatureSizeAreaFeature)}} {{Noun(GeographyFeatureSizeLocalFeature)}}",
            "{{Noun(HistoricalFigure)}} {{Noun(GeographyFeatureSizeAreaFeature)}} {{Noun(GeographyFeatureSizeLocalFeature)}}",
            "{{{{Adjective(Position, Quality, Age, Colour)}} {{Noun(!LastName, !HistoricalFigure)}} {{Noun(GeographyFeatureSizeLocalFeature)}}",
            "{{{{Adjective(Position, Quality, Age, Colour)}} {{Noun(!LastName, !HistoricalFigure)}} {{Noun(GeographyFeatureSizeAreaFeature)}}",
        ];
        let short_templates = vec![
            "{{{{Adjective(Position, Quality, Age, Colour)}} {{Noun(!LastName, !HistoricalFigure)}}",
            "{{Noun(HistoricalFigure)}} {{Noun(GeographyFeatureSizeLocalFeature)}}",
            "{{{{Adjective(Position, Quality, Age, Colour)}} {{Noun(GeographyFeatureSizeLocalFeature)}}",
        ];
        if long {
            return render_template_2(random_pick(&long_templates), &dict, era);
        }
        return render_template_2(random_pick(&short_templates), &dict, era);
    }

    pub fn gen_location(dict: &Vec<Word>, era: &Option<Era>) -> Location {
        let mut rng = rand::thread_rng();
        return Location {
            id: Uuid::new_v4(),
            name: gen_location_name(&dict, false, era),
            size: ((rng.gen::<f32>() * 10.0) as i32).max(1) as usize,
        };
    }

    #[test]
    fn test_gen_location_name() {
        use crate::culture::culture::*;
        let dict = build_dictionary();
        let name_dict = build_culture_dictionary(&dict, &random_culture(&dict, &None));
        for _i in 0..10 {
            println!("{}", gen_location_name(&name_dict, true, &None));
        }
    }

    // valid location names
    // descriptor name major minor
    // descriptor major minor
    // name major minor
    // descriptor name minor
}
