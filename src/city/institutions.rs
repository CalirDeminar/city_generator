pub mod food_institutions;
pub mod institutions {
    use rand::seq::SliceRandom;
    use rand::Rng;
    use uuid::Uuid;

    use crate::city::building::building::{Building, BuildingFloor, BuildingFloorArea};
    use crate::city::city::{add_institution_to_city, City};
    use crate::city::locations::locations::Location;
    use crate::city::population::mind::mind::{
        add_leaving_workplace_to_mind_log, add_new_workplace_to_mind_log,
        add_startup_creation_to_mind_log, Mind,
    };
    use crate::city::population::mind::relations::relations::ADULT_AGE_FROM;
    use crate::culture::culture::CultureConfig;
    use crate::language::language::*;
    use crate::templater::templater::*;
    use crate::utils::utils::random_pick;

    use super::food_institutions::food_institutions::{
        random_general_food_outlet, random_specialist_food_outlet,
    };

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
        FoodService,             // Cafes, Bars, Pubs
        SpecialistFoodService,   // Waffle Houses, Noodle Bars, etc
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
        pub serves: Vec<String>,
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
    pub const PRIVATE_INSTITUTE_BASE_SIZE: usize = 10;

    const RANDOM_SACKING_RATE: f32 = 0.1;
    const STARTUP_RATE: f32 = 0.01;

    fn label_insitute_type(i: &InstituteType, era: &Option<Era>) -> String {
        return String::from(match (i, era) {
            (InstituteType::PowerStation, Some(Era::Fantasy))
            | (InstituteType::PowerStation, Some(Era::Medieval)) => "Gallows",
            (InstituteType::PowerStation, _) => "Power Station",
            (InstituteType::WaterTreatmentWorks, Some(Era::Fantasy))
            | (InstituteType::WaterTreatmentWorks, Some(Era::Medieval)) => "Spring",
            (InstituteType::WaterTreatmentWorks, _) => "Water Treatment Works",
            (InstituteType::SewageWorks, Some(Era::Fantasy))
            | (InstituteType::SewageWorks, Some(Era::Medieval)) => "Latrines",
            (InstituteType::SewageWorks, _) => "Sewage Works",
            (InstituteType::CityHall, _) => "City Hall",
            (InstituteType::PoliceStation, Some(Era::Fantasy))
            | (InstituteType::PoliceStation, Some(Era::Medieval)) => "Guard House",
            (InstituteType::PoliceStation, _) => "Police Station",
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
                    render_template_2("{{Noun(HistoricalFigure)}}", &dict, &era),
                    label_insitute_type(&i, era)
                ),
                public: true,
                institute_type: i,
                size: (rng.gen::<f32>() * PUBLIC_INSTITUTE_BASE_SIZE as f32) as usize,
                serves: Vec::new(),
            });
        }
        return output;
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
            serves: Vec::new(),
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
            serves: Vec::new(),
        };
    }

    pub fn generate_admin(dict: &Vec<Word>, era: &Option<Era>) -> Institution {
        let mut rng = rand::thread_rng();
        let templates = vec![
            "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(LastName}} {{Noun(ServiceAdmin)}}",
            "{{Adjective(Position, Quality, Age, Colour)}} {{Noun(LastName}}'s {{Noun(ServiceAdmin)}}",
            "{{Noun(LastName}} {{Noun(ServiceAdmin)}}",
            "{{Noun(LastName}}'s {{Noun(ServiceAdmin)}}",
        ];
        let name = render_template_2(random_pick(&templates), &dict, era);
        return Institution {
            id: Uuid::new_v4(),
            name,
            public: false,
            institute_type: InstituteType::AdministrationService,
            size: (rng.gen::<f32>() * PRIVATE_INSTITUTE_BASE_SIZE as f32) as usize,
            serves: Vec::new(),
        };
    }

    pub fn generate_population_institution(
        dict: &Vec<Word>,
        culture: &Option<CultureConfig>,
    ) -> Institution {
        let mut rng = rand::thread_rng();
        let era = if culture.is_some() {
            culture.clone().unwrap().era
        } else {
            None
        };
        let roll: f32 = rng.gen();
        if roll > 0.875 {
            return random_specialist_food_outlet(&dict, &culture);
        } else if roll > 0.75 {
            return random_general_food_outlet(&dict, &culture);
        } else if roll > 0.5 {
            return generate_specialist_retailer(&dict, &era);
        } else if roll > 0.25 {
            return generate_general_retailer(&dict, &era);
        } else {
            return generate_admin(&dict, &era);
        }
    }

    pub fn generate_population_institutions(
        size: usize,
        culture: &Option<CultureConfig>,
    ) -> Vec<Institution> {
        let era = if culture.is_some() {
            culture.clone().unwrap().era
        } else {
            None
        };
        let language_dict = build_dictionary();
        let mut output: Vec<Institution> = Vec::new();
        for i in generate_public_institutions(&language_dict, &era) {
            output.push(i);
        }
        for _i in 0..((size as i32 - output.len() as i32).max(1)) {
            output.push(generate_population_institution(&language_dict, &culture));
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
                let employer = city
                    .institutions
                    .iter()
                    .find(|i| i.id.eq(&mind.employer.unwrap()));
                mind.employer = None;

                add_leaving_workplace_to_mind_log(mind, city.year, &employer.unwrap().name);
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
                add_new_workplace_to_mind_log(mind_mut, city.year, &inst.name);
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
                let new_inst = generate_population_institution(&dict, &Some(city.culture.clone()));
                let mind = city.citizens.get_mut(&m.id).unwrap();
                mind.employer = Some(new_inst.id.clone());
                add_startup_creation_to_mind_log(mind, city.year, &new_inst.name);
                drop(mind);
                add_institution_to_city(city, new_inst, &dict);
            }
        }
        return city;
    }
}
