pub mod html_exporter {
    use html_builder::*;
    use std::{fmt::Write as fmtWrite, fs::File, io::Write};
    use uuid::Uuid;

    use crate::city::{
        building::building::Building,
        city::City,
        institutions::institutions::find_institution_address,
        locations::locations::Location,
        population::mind::{
            mind::{find_address, get_name_from_id, Mind},
            relations::relations::RelationVerb,
        },
    };

    pub fn export_city_html(city: &City) {
        let living = city.citizens.values().filter(|c| c.alive);
        let dead = city.citizens.values().filter(|c| !c.alive);
        let mut document = Buffer::new();
        document.doctype();
        let mut html = document.html().attr("lang='en'");
        writeln!(html.head().title(), "City Name: {}", &city.name).unwrap();
        html.link().attr("rel='stylesheet' href='./style.css'");
        let mut body = html.body();
        writeln!(body.h1(), "{}", city.name).unwrap();
        writeln!(body.p(), "Population: {}", living.clone().count()).unwrap();
        writeln!(body.p(), "Dead: {}", dead.clone().count()).unwrap();
        writeln!(body.p(), "Area Count: {}", city.areas.len()).unwrap();
        writeln!(body.p(), "Building Count: {}", city.buildings.len()).unwrap();
        writeln!(body.h2(), "Locations:").unwrap();
        let mut loc_list = body.ul();
        for area in &city.areas {
            print_location_html(&mut loc_list.li(), &area, &city);
        }

        writeln!(body.h2(), "Citizens").unwrap();
        let mut citizen_list = body.ul();
        for m in living {
            print_mind_html(&mut citizen_list.li(), &m, &city);
        }

        let mut file = File::create("./export.html").unwrap();
        file.write_all(document.finish().into_bytes().as_slice())
            .unwrap();
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
                    if inst.unwrap().serves.len() > 0 {
                        writeln!(a.div(), "{}", "Serves: ").unwrap();
                        let mut m = a.ul();
                        for item in inst.unwrap().serves.clone() {
                            writeln!(m.li(), "{}", item).unwrap();
                        }
                    }
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

    pub fn print_mind_html<'a>(
        node: &'a mut Node<'a>,
        mind: &Mind,
        city: &City,
    ) -> &'a mut Node<'a> {
        let workplace = city
            .institutions
            .iter()
            .find(|i| mind.employer.is_some() && mind.employer.unwrap().eq(&i.id));

        let mut relations: Vec<(&RelationVerb, String, Uuid)> = mind
            .relations
            .iter()
            .map(|(verb, id)| (verb, get_name_from_id(&id, &city.citizens), id.clone()))
            .collect();
        relations.sort_by_key(|a| a.0.to_string());

        let mut list_element = node.div().attr(&format!("id='{}'", mind.id));
        writeln!(
            list_element.h3(),
            "Name: {} {}",
            &mind.first_name,
            &mind.last_name
        )
        .unwrap();
        writeln!(list_element.p(), "Gender: {}", &mind.gender).unwrap();
        writeln!(list_element.p(), "Age: {}", &mind.age).unwrap();
        let description = &mind.physical_description;
        writeln!(
            list_element.p(),
            "Description: has {}, {} {} hair and {} eyes. Is {} with a {} build.\n",
            description.hair_adjectives.first().unwrap(),
            description.hair_colour,
            description.hair_length,
            description.eye_colour,
            description.height_adjective,
            description.build_adjective
        )
        .unwrap();

        if workplace.is_some() {
            let (building, _floor, _area, location) =
                find_institution_address(&workplace.unwrap(), &city);
            let mut p = list_element.p();
            writeln!(p, "Employer: {} at", workplace.unwrap().name).unwrap();
            writeln!(
                p.a().attr(&format!("href='#{}'", building.id)),
                "{}",
                building.name
            )
            .unwrap();
            writeln!(p, " in ").unwrap();
            writeln!(
                p.a().attr(&format!("href='#{}'", location.id)),
                "{}",
                location.name
            )
            .unwrap();
        } else {
            writeln!(list_element.p(), "Employer: None").unwrap();
        }
        if mind.residence.is_some() {
            let (building, apartment, residential_location) = find_address(mind, city);
            let mut line = list_element.p();
            writeln!(line, "Lives at: ").unwrap();
            writeln!(
                line.a().attr(&format!("href='#{}'", apartment.id)),
                "{}",
                apartment.name
            )
            .unwrap();
            writeln!(line, " - ").unwrap();
            writeln!(
                line.a().attr(&format!("href='#{}'", building.id)),
                "{}",
                building.name
            )
            .unwrap();
            writeln!(line, " - ").unwrap();
            writeln!(
                line.a()
                    .attr(&format!("href='#{}'", residential_location.id)),
                "{}",
                residential_location.name
            )
            .unwrap();
        }

        if relations.len() < 1 {
            writeln!(list_element.p(), "Relations: None").unwrap();
        } else {
            writeln!(list_element.p(), "Relations:").unwrap();
            let mut relation_list = list_element.ul();
            for (verb, name, id) in relations {
                let mut list_el = relation_list.li();
                let mut list_el_para = list_el.p();
                writeln!(list_el_para, "{:?}:", verb).unwrap();
                writeln!(
                    list_el_para.a().attr(&format!("href='#{}'", id)),
                    "{}",
                    name
                )
                .unwrap();
            }
        }

        return node;
    }
}
