pub mod building {
    use html_builder::*;
    use rand::Rng;
    use std::fmt::Write as fmtWrite;
    use uuid::Uuid;

    use crate::{
        city::city::*,
        city::{
            locations::locations::{gen_location, Location},
            population::mind::mind::*,
        },
        culture::culture::{random_culture, CultureConfig},
        language::language::*,
        templater::templater::*,
        utils::utils::random_pick,
    };
    #[derive(PartialEq, Debug, Clone)]
    pub enum FloorAreaType {
        Apartment,  // anywhere bar ground floor
        Commercial, // ground floor only
        Utilities,  // must have one somewhere
        Facilities, // may have one
        Security,   // must have one somewhere
        Lobby,      // must have one, must be on ground floor
    }
    #[derive(PartialEq, Debug, Clone)]
    pub enum FloorType {
        Residential,
        Commercial,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct BuildingFloorArea {
        pub id: Uuid,
        pub name: String,
        pub area_type: FloorAreaType,
        pub owning_institution: Option<Uuid>,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct BuildingFloor {
        pub id: Uuid,
        pub level: i32,
        pub floor_type: FloorType,
        pub areas: Vec<BuildingFloorArea>,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct Building {
        pub id: Uuid,
        pub name: String,
        pub floors: Vec<BuildingFloor>,
        pub location_id: Option<Uuid>,
    }

    pub fn print_building(building: &Building, city: &City) -> String {
        let mut output: String = String::new();
        output.push_str(&format!("  {}:\n", building.name));
        for floor in &building.floors {
            if floor.level > 0 {
                output.push_str(&format!("      Floor {}\n", floor.level));
            } else if floor.level.eq(&-1) {
                output.push_str(&format!("      Basement\n"));
            } else {
                output.push_str(&format!("      Ground Floor\n"));
            }
            for area in &floor.areas {
                let inst = city.institutions.iter().find(|i| {
                    area.owning_institution.is_some() && i.id.eq(&area.owning_institution.unwrap())
                });
                if inst.is_some() {
                    output.push_str(&format!(
                        "          {}: {}\n",
                        area.name,
                        inst.unwrap().name
                    ));
                } else {
                    let residents: Vec<&Mind> = city
                        .citizens
                        .values()
                        .filter(|m| m.residence.is_some() && m.residence.unwrap().eq(&area.id))
                        .collect();
                    if residents.len().eq(&0) {
                        output.push_str(&format!("          {}: Empty\n", area.name));
                    } else {
                        let mut names: Vec<String> = Vec::new();
                        for resident in residents {
                            names.push(format!("{} {}", resident.first_name, resident.last_name));
                        }
                        output.push_str(&format!(
                            "          {}: {}\n",
                            area.name,
                            names.join(", ")
                        ));
                    }
                }
            }
        }
        return output;
    }

    pub fn print_building_html<'a>(
        node: &'a mut Node<'a>,
        building: &Building,
        city: &City,
    ) -> &'a mut Node<'a> {
        let mut base = node.div().attr(&format!("id='{}'", building.id));
        writeln!(base.h5(), "{}", building.name).unwrap();
        let mut floors = base.ul();
        for floor in &building.floors {
            let mut f = floors.li();
            if floor.level > 0 {
                writeln!(f.h6(), "Floor {}", floor.level).unwrap();
            } else if floor.level.eq(&-1) {
                writeln!(f.h6(), "Basement").unwrap();
            } else {
                writeln!(f.h6(), "Ground Floor").unwrap();
            }

            for area in &floor.areas {
                let inst = city.institutions.iter().find(|i| {
                    area.owning_institution.is_some() && i.id.eq(&area.owning_institution.unwrap())
                });
                let residents: Vec<&Mind> = city
                    .citizens
                    .values()
                    .filter(|m| m.residence.is_some() && m.residence.unwrap().eq(&area.id))
                    .collect();
                let mut a = f.li().attr(&format!("id='{}'", area.id));
                writeln!(a, "{}: ", area.name).unwrap();
                if inst.is_some() {
                    writeln!(
                        a.a().attr(&format!("href='#{}'", inst.unwrap().id)),
                        "{}",
                        inst.unwrap().name
                    )
                    .unwrap();
                } else if residents.len() > 0 {
                    let mut first = true;
                    for resident in residents {
                        if !first {
                            writeln!(a, ", ").unwrap();
                        }
                        writeln!(
                            a.a().attr(&format!("href='#{}'", resident.id)),
                            "{} {}",
                            resident.first_name,
                            resident.last_name
                        )
                        .unwrap();
                        first = false;
                    }
                } else {
                    writeln!(a, " None").unwrap();
                }
            }
        }
        return node;
    }

    pub fn building_area_is_owned<'a>(area: &'a BuildingFloorArea, city: &'a City) -> bool {
        return city
            .citizens
            .values()
            .any(|c| c.residence.is_some() && c.residence.unwrap().eq(&area.id));
    }

    fn new_floor(level: i32, floor_type: FloorType, culture: &CultureConfig) -> BuildingFloor {
        let mut rng = rand::thread_rng();
        let mut areas: Vec<BuildingFloorArea> = Vec::new();
        let footprint = culture.avg_building_footprint / 2;
        let area_count = footprint + (rng.gen::<f32>() * footprint as f32) as i32;
        for i in 0..=area_count {
            if level == 0 && i == 0 {
                areas.push(BuildingFloorArea {
                    id: Uuid::new_v4(),
                    name: format!("{}{:0>2}", level, i + 1),
                    area_type: FloorAreaType::Lobby,
                    owning_institution: None,
                });
            } else if level.eq(&(-1)) && i == 0 {
                areas.push(BuildingFloorArea {
                    id: Uuid::new_v4(),
                    name: format!("{}{:0>2}", level, i + 1),
                    area_type: FloorAreaType::Utilities,
                    owning_institution: None,
                });
            } else {
                if floor_type.eq(&FloorType::Residential) {
                    areas.push(BuildingFloorArea {
                        id: Uuid::new_v4(),
                        name: format!("{}{:0>2}", level, i + 1),
                        area_type: FloorAreaType::Apartment,
                        owning_institution: None,
                    });
                } else {
                    areas.push(BuildingFloorArea {
                        id: Uuid::new_v4(),
                        name: format!("{}{:0>2}", level, i + 1),
                        area_type: FloorAreaType::Commercial,
                        owning_institution: None,
                    });
                }
            }
        }
        return BuildingFloor {
            id: Uuid::new_v4(),
            level,
            floor_type,
            areas,
        };
    }

    pub fn new_building_no_loc(
        dict: &Vec<Word>,
        culture: &CultureConfig,
        residential: bool,
    ) -> Building {
        return new_building(dict, None, culture, residential);
    }

    pub fn new_building(
        dict: &Vec<Word>,
        location_id: Option<Uuid>,
        culture: &CultureConfig,
        residential: bool,
    ) -> Building {
        let name_templates = vec![
            "{{{{Adjective(Position, Quality, Age, Colour)}} {{Noun(LastName)}} {{Noun(BuildingTitle)}}",
            "{{Noun(LastName)}} {{Noun(BuildingTitle)}}",
            "{{{{Adjective(Position, Quality, Age, Colour)}} {{Noun(BuildingTitle)}}",
        ];
        let mut rng = rand::thread_rng();
        let mut floors: Vec<BuildingFloor> = Vec::new();
        let floor_count =
            ((rng.gen::<f32>() * ((culture.avg_building_floors * 2) + 1) as f32) as i32).max(2);
        let commercial_floor_count = if residential {
            0
        } else {
            ((floor_count as f32 / 2.0) * (rng.gen::<f32>()))
                .floor()
                .max(1.0) as i32
        };
        let has_basement = rng.gen::<f32>() > 0.5;
        for i in (if has_basement { -1 } else { 0 })..=floor_count {
            let floor_type = if (i) < commercial_floor_count && i >= 0 {
                FloorType::Commercial
            } else {
                FloorType::Residential
            };
            floors.push(new_floor(i, floor_type, culture));
        }
        return Building {
            id: Uuid::new_v4(),
            name: render_template_2(random_pick(&name_templates), &dict, &culture.era),
            floors,
            location_id,
        };
    }

    fn find_free_area<'a>(city: &'a mut City) -> Option<&'a mut Location> {
        return city.areas.iter_mut().find(|a| {
            city.buildings
                .iter_mut()
                .filter(|b| b.location_id.is_some() && b.location_id.unwrap().eq(&a.id))
                .count()
                < a.size
        });
    }

    pub fn add_building_to_city<'a>(
        city: &'a mut City,
        dict: &Vec<Word>,
        residential: bool,
    ) -> &'a mut City {
        let mut free_location = find_free_area(city);
        if free_location.is_none() {
            city.areas.push(gen_location(&dict, &city.culture.era));
            free_location = find_free_area(city);
        }
        let new_building = new_building(
            &dict,
            Some(free_location.unwrap().id.clone()),
            &city.culture,
            residential,
        );
        city.buildings.push(new_building);
        return city;
    }

    fn count_available_apartments(city: &City) -> usize {
        return city
            .buildings
            .iter()
            .flat_map(|b| {
                b.floors
                    .iter()
                    .filter(|f| f.floor_type.eq(&FloorType::Residential))
                    .flat_map(|f| f.areas.clone())
            })
            .count();
    }

    pub fn add_buildings_per_year<'a>(city: &'a mut City, dict: &Vec<Word>) -> &'a mut City {
        let living_citizen_count = city.citizens.values().filter(|c| c.alive).count();
        let acceptable_homeless_count = living_citizen_count / 100;
        let mut free_apartment_count = count_available_apartments(&city);
        while free_apartment_count < (living_citizen_count - acceptable_homeless_count) {
            add_building_to_city(city, &dict, true);
            free_apartment_count = count_available_apartments(&city);
        }
        return city;
    }

    #[test]
    fn test_new_building() {
        let dict = build_dictionary();
        println!(
            "{:#?}",
            new_building_no_loc(&dict, &random_culture(&dict, &None), false)
        );
    }
}
