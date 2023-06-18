pub mod materials {
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter}; // 0.17.1

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum MaterialState {
        #[default]
        MaterialStateSolid,
        MaterialStateLiquid,
        MaterialStateGas,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum SolidMaterialForm {
        #[default]
        SolidMaterialFormSolid,
        SolidMaterialFormCloth,
        SolidMaterialFormThread,
        SolidMaterialFormMail,
        SolidMaterialFormPlate,
        SolidMaterialFormPowder,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum MaterialTag {
        MaterialTagFlamable,
        MaterialTagMetal,
        MaterialTagSharpenable,
        #[default]
        MaterialTagNone,
    }

    pub fn material_tags() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        output.push(String::from("Material"));
        for tag in MaterialState::iter() {
            output.push(tag.to_string());
        }
        for tag in SolidMaterialForm::iter() {
            output.push(tag.to_string());
        }
        for tag in MaterialTag::iter() {
            output.push(tag.to_string());
        }
        return output;
    }
}
