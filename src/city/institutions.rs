pub mod institutions {
    use rand::seq::SliceRandom;
    use rand::Rng;
    use uuid::Uuid;

    use crate::city::building::building::{Building, BuildingFloor, BuildingFloorArea};
    use crate::city::city::{add_institution_to_city, City};
    use crate::city::locations::locations::Location;
    use crate::city::population::mind::mind::Mind;
    use crate::city::population::mind::relations::relations::ADULT_AGE_FROM;
    use crate::language::language::{build_dictionary, Era, Word};
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
        AdministrationService,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum PublicService {
        Library,
        School,
        University,
        Court,
        CityHall,
        Prison,
        PoliceStation,
        Hospital,
    }

    // #[derive(PartialEq, Debug, Clone)]
    // pub enum RawMaterial {
    //     Meat(String),
    //     Crop(String),
    //     Wood(String),
    //     Ore(String),
    //     Stone(String),
    //     Thread(String),
    // }

    // #[derive(PartialEq, Debug, Clone)]
    // pub enum Material {
    //     Raw(RawMaterial),
    //     Food(String),
    //     Drink(String),
    //     Metal(String),
    //     Cloth(String),
    //     Other,
    // }

    // #[derive(PartialEq, Debug, Clone)]
    // pub enum ServiceType {
    //     Food,
    //     Drink,
    //     FoodAndDrink,
    //     Entertainment,
    //     Administration,
    // }

    // #[derive(PartialEq, Debug, Clone)]
    // pub enum InstituteEconomyType {
    //     LocalProducer,                        // Creates products from the area
    //     LocalManufacturer,                    // Manufactures goods from products
    //     LocalService(ServiceType), // Provides a service to the city consuming minimal resources
    //     LocalServiceAndConsumer(ServiceType), // Provides a service to the city consuming resources
    //     LocalResller,              // Resells local products from producers and manufacturers
    //     ImportReseller,            // Imports products and goods into the city
    // }

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
        pub size: usize,
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

    const PUBLIC_INSTITUTE_BASE_SIZE: usize = 20;
    const PRIVATE_INSTITUTE_BASE_SIZE: usize = 10;

    const RANDOM_SACKING_RATE: f32 = 0.1;
    const STARTUP_RATE: f32 = 0.01;

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

    pub fn generate_public_institutions(dict: &Vec<Word>, era: &Option<Era>) -> Vec<Institution> {
        let mut rng = rand::thread_rng();
        let mut output: Vec<Institution> = Vec::new();
        for i in PUBLIC_INSTITUTES {
            output.push(Institution {
                id: Uuid::new_v4(),
                name: format!(
                    "{} {}",
                    render_template_2("{{Noun(HistoricalFigure)}}", &dict, era),
                    label_insitute_type(&i)
                ),
                public: true,
                institute_type: i,
                size: (rng.gen::<f32>() * PUBLIC_INSTITUTE_BASE_SIZE as f32) as usize,
            });
        }
        return output;
    }

    pub fn generate_restaurant(dict: &Vec<Word>, era: &Option<Era>) -> Institution {
        let mut rng = rand::thread_rng();
        let templates = vec![
            "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(LastName)}} {{Noun(RetailerFood)}}",
             "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(HistoricalFigure)}}'s {{Noun(RetailerFood)}}",
             "{{Noun(LastName)}} {{Noun(RetailerFood)}}",
             "{{Noun(LastName)}}'s {{Noun(RetailerFood)}}",
             "{{Noun(HistoricalFigure)}}'s {{Noun(RetailerFood)}}",
        ];
        let name = render_template_2(random_pick(&templates), &dict, era);
        return Institution {
            id: Uuid::new_v4(),
            name,
            public: false,
            institute_type: InstituteType::FoodService,
            size: (rng.gen::<f32>() * PRIVATE_INSTITUTE_BASE_SIZE as f32) as usize,
        };
    }

    pub fn generate_specialist_retailer(dict: &Vec<Word>, era: &Option<Era>) -> Institution {
        let mut rng = rand::thread_rng();
        let templates = vec![
            "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(LastName)}} {{Noun(RetailerSpecialist)}}",
            "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(LastName)}}'s {{Noun(RetailerSpecialist)}}",
            "{{Noun(LastName)}} {{Noun(RetailerSpecialist)}}",
            "{{Noun(LastName)}}'s {{Noun(RetailerSpecialist)}}",
        ];
        let name = render_template_2(random_pick(&templates), &dict, era);
        return Institution {
            id: Uuid::new_v4(),
            name,
            public: false,
            institute_type: InstituteType::SpecialistRetail,
            size: (rng.gen::<f32>() * PRIVATE_INSTITUTE_BASE_SIZE as f32) as usize,
        };
    }

    pub fn generate_general_retailer(dict: &Vec<Word>, era: &Option<Era>) -> Institution {
        let mut rng = rand::thread_rng();
        let templates = vec![
            "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(LastName}} {{Noun(GeneralRetailerName)}}",
            "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(LastName}}'s {{Noun(GeneralRetailerName)}}",
            "{{Noun(LastName}} {{Noun(GeneralRetailerName)}}",
            "{{Noun(LastName}}'s {{Noun(GeneralRetailerName)}}",
        ];
        let name = render_template_2(random_pick(&templates), &dict, era);
        return Institution {
            id: Uuid::new_v4(),
            name,
            public: false,
            institute_type: InstituteType::GeneralRetail,
            size: (rng.gen::<f32>() * PRIVATE_INSTITUTE_BASE_SIZE as f32) as usize,
        };
    }

    pub fn generate_population_institution(dict: &Vec<Word>, era: &Option<Era>) -> Institution {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        if roll > 0.4 {
            return generate_restaurant(&dict, era);
        } else if roll > 0.2 {
            return generate_specialist_retailer(&dict, era);
        } else {
            return generate_general_retailer(&dict, era);
        }
    }

    pub fn generate_population_institutions(size: usize, era: &Option<Era>) -> Vec<Institution> {
        let language_dict = build_dictionary();
        let mut output: Vec<Institution> = Vec::new();
        for i in generate_public_institutions(&language_dict, era) {
            output.push(i);
        }
        for _i in 0..((size as i32 - output.len() as i32).max(1)) {
            output.push(generate_population_institution(&language_dict, era));
        }
        return output;
    }

    pub fn random_sackings_per_year<'a>(city: &'a mut City) -> &'a mut City {
        let mut rng = rand::thread_rng();
        let employed = city
            .citizens
            .values_mut()
            .filter(|c| c.alive && c.employer.is_some());
        for mind in employed {
            if rng.gen::<f32>() < RANDOM_SACKING_RATE {
                mind.employer = None;
            }
        }
        return city;
    }

    pub fn assign_employment_per_year<'a>(city: &'a mut City) -> &'a mut City {
        let isntitutions_ref = city.institutions.clone();
        let citizens_ref = city.citizens.clone();
        let (employed, unemployed) = citizens_ref
            .values()
            .filter(|c| c.alive && c.age > ADULT_AGE_FROM)
            .fold(
                (vec![], vec![]),
                |(employed, unemployed): (Vec<&Mind>, Vec<&Mind>), c| {
                    if c.employer.is_some() {
                        let employed = vec![employed, vec![c]].concat();
                        return (employed, unemployed);
                    } else {
                        let unemployed = vec![unemployed, vec![c]].concat();
                        return (employed, unemployed);
                    }
                },
            );
        let mut under_strength_institutions: Vec<(&Institution, usize)> = isntitutions_ref
            .iter()
            .map(|i| {
                let employee_count = employed
                    .iter()
                    .filter(|c| c.employer.is_some() && c.employer.unwrap().eq(&i.id))
                    .count();
                return (i, employee_count);
            })
            .filter(|(i, c)| c < &i.size)
            .collect();
        for mind in unemployed {
            let possible_target = under_strength_institutions.pop();
            if possible_target.is_some() {
                let (inst, emp_count) = possible_target.unwrap();

                let mind_mut = city.citizens.get_mut(&mind.id).unwrap();
                mind_mut.employer = Some(inst.id.clone());
                drop(mind_mut);

                if emp_count + 1 < inst.size {
                    under_strength_institutions.push((inst, emp_count + 1));
                }
            }
            under_strength_institutions.shuffle(&mut rand::thread_rng());
        }
        return city;
    }

    pub fn create_startups_per_year<'a>(city: &'a mut City, dict: &Vec<Word>) -> &'a mut City {
        // let mut city = city;
        let mut rng = rand::thread_rng();
        let citizen_ref = city.citizens.clone();
        let unemployed = citizen_ref
            .values()
            .filter(|c| c.alive && c.age > ADULT_AGE_FROM && c.employer.is_none());
        for m in unemployed {
            if rng.gen::<f32>() < STARTUP_RATE {
                let new_inst = generate_population_institution(&dict, &city.culture.era);
                let mind = city.citizens.get_mut(&m.id).unwrap();
                mind.employer = Some(new_inst.id.clone());
                drop(mind);
                add_institution_to_city(city, new_inst, &dict);
            }
        }
        return city;
    }

    // #[test]
    // fn generate_population_institutions_test() {
    //     let name_dict = gen_name_dict();
    //     println!("{:#?}", generate_population_institutions(&name_dict));
    // }

    #[test]
    fn gen_institutions_test() {
        println!("{:#?}", generate_population_institutions(20, &None));
    }
}
