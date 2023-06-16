pub mod plants {
    use strum_macros::{Display, EnumIter};

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum PlantType {
        #[default]
        Normal,
        Tree,
        Flower,
        Crop,
    }
}
