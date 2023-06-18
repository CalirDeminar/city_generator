pub mod geography {
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter}; // 0.17.1

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum GeographyFeatureSize {
        GeographyFeatureSizeBiome,
        GeographyFeatureSizeAreaFeature,
        #[default]
        GeographyFeatureSizeLocalFeature,
    }

    pub fn geography_tags() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        output.push(String::from("GeographicFeature"));
        for tag in GeographyFeatureSize::iter() {
            output.push(tag.to_string());
        }
        return output;
    }
}
