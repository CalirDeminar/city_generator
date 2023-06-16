pub mod materials {
    use strum_macros::{Display, EnumIter};

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum MaterialState {
        #[default]
        Solid,
        Liquid,
        Gas,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum SolidMaterialForm {
        #[default]
        Solid,
        Cloth,
        Thread,
        Mail,
        Sheet,
        Powder,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum MaterialTag {
        Flamable,
        Metal,
        Sharpenable,
        #[default]
        None,
    }
}
