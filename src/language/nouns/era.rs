pub mod eras {
    use crate::language::language::Era;
    use strum::IntoEnumIterator;

    pub fn era_tags() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for tag in Era::iter() {
            output.push(tag.to_string());
        }
        return output;
    }
}
