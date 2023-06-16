pub mod geography {
    use strum_macros::{Display, EnumIter};

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum FeatureSize {
        Biome,
        AreaFeature,
        #[default]
        LocalFeature,
    }
}
