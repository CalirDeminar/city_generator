pub mod emotions {

    use strum_macros::{Display, EnumIter};

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum EmotionGroups {
        Love,
        Joy,
        Surprise,
        Anger,
        Sadness,
        Fear,
        #[default]
        Other,
    }
}
