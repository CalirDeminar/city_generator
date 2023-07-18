pub mod plants {
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter}; // 0.17.1

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum PlantType {
        Plant,
        #[default]
        PlantTypeNormal,
        PlantTypeTree,
        PlantTypeFlower,
        PlantTypeCrop,
        PlantTypeFruit,
        PlantTypeGrain,
    }

    pub fn plant_tags() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for tag in PlantType::iter() {
            output.push(tag.to_string());
        }

        return output;
    }
}
