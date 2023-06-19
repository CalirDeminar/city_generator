pub mod parser {
    use regex::Regex;
    use std::fs::{self, File};
    use std::io::{self, BufRead, Write};

    pub type ParserOutput = Vec<(String, Vec<String>)>;

    pub fn parse_file(filename: String) -> ParserOutput {
        let mut output: ParserOutput = Vec::new();
        let full_filename = &format!("./static_data/{}", filename);

        let file = File::open(full_filename).expect(&format!("Cannot open: {}", &filename));
        let lines = io::BufReader::new(file).lines();

        for l in lines {
            let mut subject: String = String::new();
            let mut tags: Vec<String> = Vec::new();
            if l.is_ok() {
                let line_value = l.unwrap();
                let line =
                    Regex::replace_all(&Regex::new(r"\/\/[a-zA-Z ]*$").unwrap(), &line_value, "");
                let splits = line.split(",");
                for (i, entry) in splits.enumerate() {
                    if i == 0 {
                        subject = String::from(entry.trim());
                    } else {
                        tags.push(String::from(entry.trim()));
                    }
                }
                if subject.len() > 0 {
                    output.push((subject, tags));
                }
            }
        }

        return output;
    }

    type DataFileGroup = (String, Vec<String>);

    fn format_file(filename: String) {
        let full_filename = &format!("./static_data/{}", filename);

        let read_file = File::open(full_filename).expect(&format!("Cannot open: {}", &filename));
        let mut groups: Vec<DataFileGroup> = Vec::new();
        let mut working_group: DataFileGroup = (String::new(), Vec::new());
        let lines = io::BufReader::new(read_file).lines();
        for l in lines {
            if l.is_ok() {
                let line = l.unwrap().clone();
                if line.contains("//") {
                    working_group.1.sort();
                    groups.push(working_group.clone());
                    working_group = (String::from(line), Vec::new());
                } else {
                    working_group.1.push(line.clone());
                }
            }
        }
        working_group.1.sort();
        groups.push(working_group);
        let mut output: String = String::new();
        for group in groups {
            if group.0.len() > 0 {
                output.push_str(&format!("{}\n", group.0));
            }
            for i in group.1 {
                output.push_str(&format!("{}\n", i));
            }
        }
        output.trim();
        let mut write_file = File::create(&format!("./static_data/{}", filename)).unwrap();
        write_file
            .write_all(output.into_bytes().as_slice())
            .unwrap();
    }

    fn format_folder(path: String) {
        let paths = fs::read_dir(path).unwrap();
        for p in paths {
            let path = p.unwrap();
            let filename = path.file_name();
            println!("Metadata {:?}", path.metadata());
            if path.metadata().unwrap().is_file() {
                println!("{}", filename.to_str().unwrap());
                format_file(String::from(filename.to_str().unwrap()));
            } else {
            }
        }
    }

    #[test]
    fn format_data_files() {
        format_folder(String::from("./static_data"))
    }
}
