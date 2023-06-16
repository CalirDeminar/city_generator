pub mod creatures {
    use strum_macros::{Display, EnumIter};

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum CreatureSize {
        Tiny,
        Small,
        #[default]
        Normal,
        Large,
        Great,
    }
    // Creature Family
    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum CreatureFamily {
        Mammal,
        Bird,
        Reptile,
        Fish,
        #[default]
        Other,
    }
}
