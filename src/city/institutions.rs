pub mod institutions {
    use rand::Rng;
    use uuid::Uuid;

    use crate::city::building::building::{Building, BuildingFloor, BuildingFloorArea};
    use crate::city::city::City;
    use crate::city::locations::locations::Location;
    use crate::names::names::*;
    use crate::templater::templater::*;
    use crate::utils::utils::random_pick;

    #[derive(PartialEq, Debug, Clone)]
    pub enum InstituteType {
        // Public Infra
        PowerStation,
        WaterTreatmentWorks,
        SewageWorks,
        Library,
        School,
        University,
        Court,
        CityHall,
        Prison,
        PoliceStation,
        Hospital,
        // Corporate Infra
        FoodService,             // Restarants, Bars, Pubs
        GeneralRetail,           // Most "general" shops, cornerShops, supermarkets, etc
        SpecialistRetail,        // Specialist Retailers, jewelers, tailors, mechanics
        EntertainmentVenue,      // Thearters, cinemas, nightclubs
        IndustrialManufacturers, // Goods manufacturers
        SpecialistService,       // "Office" businesses
        Publishers,
    }

    pub const LARGE_SCALE_INSTITUTE_TYPES: [InstituteType; 4] = [
        InstituteType::PowerStation,
        InstituteType::WaterTreatmentWorks,
        InstituteType::SewageWorks,
        InstituteType::IndustrialManufacturers,
    ];

    #[derive(PartialEq, Debug, Clone)]
    pub struct Institution {
        pub id: Uuid,
        pub name: String,
        pub public: bool,
        pub institute_type: InstituteType,
    }

    const PUBLIC_INSTITUTES: [InstituteType; 11] = [
        InstituteType::PowerStation,
        InstituteType::WaterTreatmentWorks,
        InstituteType::SewageWorks,
        InstituteType::Library,
        InstituteType::School,
        InstituteType::University,
        InstituteType::Court,
        InstituteType::CityHall,
        InstituteType::Prison,
        InstituteType::PoliceStation,
        InstituteType::Hospital,
    ];

    fn label_insitute_type(i: &InstituteType) -> String {
        return String::from(match i {
            InstituteType::PowerStation => "Power Station",
            InstituteType::WaterTreatmentWorks => "Water Treatment Works",
            InstituteType::SewageWorks => "Sewage Works",
            InstituteType::CityHall => "City Hall",
            InstituteType::PoliceStation => "Police Station",
            _ => {
                let r = format!("{:?}", i);
                return r;
            }
        });
    }

    pub fn find_institution_building<'a>(
        institution: &Institution,
        city: &'a City,
    ) -> Option<&'a Building> {
        return city.buildings.iter().find(|b| {
            b.floors.iter().any(|f| {
                f.areas.iter().any(|a| {
                    a.owning_institution.is_some()
                        && a.owning_institution.unwrap().eq(&institution.id)
                })
            })
        });
    }

    pub fn find_institution_address<'a>(
        institution: &Institution,
        city: &'a City,
    ) -> (
        &'a Building,
        &'a BuildingFloor,
        &'a BuildingFloorArea,
        &'a Location,
    ) {
        let building = city
            .buildings
            .iter()
            .find(|b| {
                b.floors.iter().any(|f| {
                    f.areas.iter().any(|a| {
                        a.owning_institution.is_some()
                            && a.owning_institution.unwrap().eq(&institution.id)
                    })
                })
            })
            .unwrap();
        let floor = building
            .floors
            .iter()
            .find(|f| {
                f.areas.iter().any(|a| {
                    a.owning_institution.is_some()
                        && a.owning_institution.unwrap().eq(&institution.id)
                })
            })
            .unwrap();
        let area = floor
            .areas
            .iter()
            .find(|a| {
                a.owning_institution.is_some() && a.owning_institution.unwrap().eq(&institution.id)
            })
            .unwrap();
        let location = city
            .areas
            .iter()
            .find(|a| a.id.eq(&building.location_id.unwrap()))
            .unwrap();
        return (building, floor, area, location);
    }

    pub fn generate_public_institutions(name_dict: &NameDictionary) -> Vec<Institution> {
        let mut output: Vec<Institution> = Vec::new();
        for i in PUBLIC_INSTITUTES {
            output.push(Institution {
                id: Uuid::new_v4(),
                name: format!(
                    "{} {}",
                    random_pick(&name_dict.last_names).name,
                    label_insitute_type(&i)
                ),
                public: true,
                institute_type: i,
            });
        }
        return output;
    }

    pub fn generate_restaurant(name_dict: &NameDictionary) -> Institution {
        let templates = vec![
            "{{LocationDescriptor}}{{LastName}}{{InstitutionFoodServiceSuffix}}",
            "{{LastName}}{{InstitutionFoodServiceSuffix}}",
        ];
        let name = render_template(random_pick(&templates), &name_dict.total_list);
        return Institution {
            id: Uuid::new_v4(),
            name,
            public: false,
            institute_type: InstituteType::FoodService,
        };
    }

    pub fn generate_specialist_retailer(name_dict: &NameDictionary) -> Institution {
        let templates = vec![
            "{{LocationDescriptor}}{{LastName}}{{InstitutionRetailSpecificSuffix}}",
            "{{LastName}}{{InstitutionRetailSpecificSuffix}}",
        ];
        let name = render_template(random_pick(&templates), &name_dict.total_list);
        return Institution {
            id: Uuid::new_v4(),
            name,
            public: false,
            institute_type: InstituteType::SpecialistRetail,
        };
    }

    pub fn generate_general_retailer(name_dict: &NameDictionary) -> Institution {
        let templates = vec![
            "{{LocationDescriptor}}{{LastName}}{{InstitutionRetailGeneralSuffix}}",
            "{{LastName}}{{InstitutionRetailGeneralSuffix}}",
        ];
        let name = render_template(random_pick(&templates), &name_dict.total_list);
        return Institution {
            id: Uuid::new_v4(),
            name,
            public: false,
            institute_type: InstituteType::GeneralRetail,
        };
    }

    pub fn generate_population_institution(name_dict: &NameDictionary) -> Institution {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        if roll > 0.4 {
            return generate_restaurant(&name_dict);
        } else if roll > 0.2 {
            return generate_specialist_retailer(&name_dict);
        } else {
            return generate_general_retailer(&name_dict);
        }
    }

    pub fn generate_population_institutions(size: usize) -> Vec<Institution> {
        let name_dict = gen_name_dict();
        let mut output: Vec<Institution> = Vec::new();
        for i in generate_public_institutions(&name_dict) {
            output.push(i);
        }
        for _i in 0..((size as i32 - output.len() as i32).max(1)) {
            output.push(generate_population_institution(&name_dict));
        }
        return output;
    }

    // #[test]
    // fn generate_population_institutions_test() {
    //     let name_dict = gen_name_dict();
    //     println!("{:#?}", generate_population_institutions(&name_dict));
    // }

    #[test]
    fn gen_institutions_test() {
        println!("{:#?}", generate_population_institutions(20));
    }
}
