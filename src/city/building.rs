pub mod building {
    use uuid::Uuid;
    use rand::Rng;

    use crate::{templater::templater::render_template, names::names::{NameDictionary, gen_name_dict}, utils::utils::random_pick};
    #[derive(PartialEq, Debug, Clone)]
    pub enum FloorAreaType {
        Apartment, // anywhere bar ground floor
        Commercial, // ground floor only
        Utilities, // must have one somewhere
        Facilities, // may have one
        Security, // must have one somewhere
        Lobby // must have one, must be on ground floor
    }
    #[derive(PartialEq, Debug, Clone)]
    pub enum FloorType {
        Residential,
        Commercial
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct BuildingFloorArea {
        pub id: Uuid,
        pub name: String,
        pub area_type: FloorAreaType,
        pub owning_institution: Option<Uuid>,
        pub owning_citizen: Option<Uuid>
        // pub rooms: Vec<BuildingFloorAreaRoom>
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct BuildingFloor {
        pub id: Uuid,
        pub level: i32,
        pub floor_type: FloorType,
        pub areas: Vec<BuildingFloorArea>
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct Building {
        pub id: Uuid,
        pub name: String,
        pub floors: Vec<BuildingFloor>
    }

    fn new_floor(level: i32, floor_type: FloorType) -> BuildingFloor {
        let mut rng = rand::thread_rng();
        let mut areas: Vec<BuildingFloorArea> = Vec::new();
        let area_count = 5 + (rng.gen::<f32>()*10.0) as i32;
        for i in 0..=area_count {
            if level==0 && i==0 {
                areas.push(BuildingFloorArea {
                    id: Uuid::new_v4(),
                    name: format!("{}", i),
                    area_type: FloorAreaType::Lobby,
                    owning_institution: None,
                    owning_citizen: None
                });
            } else if level==-1 && i==0 {
                areas.push(BuildingFloorArea {
                    id: Uuid::new_v4(),
                    name: format!("B{}", i),
                    area_type: FloorAreaType::Utilities,
                    owning_institution: None,
                    owning_citizen: None
                });
            } else {
                if floor_type.eq(&FloorType::Residential) {
                    areas.push(BuildingFloorArea {
                        id: Uuid::new_v4(),
                        name: format!("{}{:0>2}", level, i),
                        area_type: FloorAreaType::Apartment,
                        owning_institution: None,
                        owning_citizen: None
                    });
                } else {
                    areas.push(BuildingFloorArea {
                        id: Uuid::new_v4(),
                        name: format!("{}{}", level, i),
                        area_type: FloorAreaType::Commercial,
                        owning_institution: None,
                        owning_citizen: None
                    });
                }
            }
        }
        return BuildingFloor {
            id: Uuid::new_v4(),
            level,
            floor_type,
            areas
        }
    }

    pub fn new_building(name_dict: &NameDictionary) -> Building {
        let name_templates = vec![
            "{{LocationDescriptor}}{{LastName}}{{BuildingSuffix}}",
            "{{LastName}}{{BuildingSuffix}}",
            "{{LocationDescriptor}}{{BuildingSuffix}}"
        ];
        let mut rng = rand::thread_rng();
        let mut floors: Vec<BuildingFloor> = Vec::new();
        let floor_count = ((rng.gen::<f32>() * 12.0)as i32).max(2);
        let commercial_floor_count = ((floor_count as f32 / 2.0)*(rng.gen::<f32>())).floor().max(1.0) as i32;
        let has_basement = rng.gen::<f32>() > 0.5;
        for i in (if has_basement {-1} else {0})..=floor_count {
            let floor_type = if (i) < commercial_floor_count && i >=0 {FloorType::Commercial} else {FloorType::Residential};
            floors.push(new_floor(i, floor_type));
        }
        return Building {
            id: Uuid::new_v4(),
            name: render_template(random_pick(&name_templates), &name_dict.total_list),
            floors
        }
    }

    #[test]
    fn test_new_building() {
        let dict = gen_name_dict();
        println!("{:#?}", new_building(&dict));
    }
}