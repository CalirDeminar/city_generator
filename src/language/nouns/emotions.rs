pub mod emotions {

    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter}; // 0.17.1

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy, Default)]
    pub enum EmotionGroups {
        EmotionGroupLove,
        EmotionGroupJoy,
        EmotionGroupSurprise,
        EmotionGroupAnger,
        EmotionGroupSadness,
        EmotionGroupFear,
        #[default]
        Other,
    }

    pub fn emotion_group_tags() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        output.push(String::from("Emotion"));
        for tag in EmotionGroups::iter() {
            output.push(tag.to_string());
        }
        return output;
    }
}
