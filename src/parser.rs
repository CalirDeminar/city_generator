pub mod parser {
    use regex::Regex;
    use std::fs::File;
    use std::io::{self, BufRead};

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
}
