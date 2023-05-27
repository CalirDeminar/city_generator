pub mod templater {
    use crate::names::names::*;
    use strum::IntoEnumIterator; // 0.17.1

    const EXAMPLE_TEMPLATE: &str = "{{LocationDesciptor}}{{LastName}}{{InstitutionFoodServiceSuffix}}";

    fn string_match_name_tag(token: &str) -> Option<NameTag> {
        for tag in NameTag::iter() {
            let matcher = format!("{}", tag);
            if matcher.eq(token.trim()) {
                return Some(tag);
            }
        }
        return None;
    }

    fn render_template(template: &str, name_dict: Vec<NameDefinition>) -> String {

    }
}