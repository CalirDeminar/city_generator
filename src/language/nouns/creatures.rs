pub mod creatures {
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter}; // 0.17.1

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy)]
    pub enum CreatureSize {
        CreatureSizeTiny,
        CreatureSizeSmall,
        CreatureSizeNormal,
        CreatureSizeLarge,
        CreatureSizeGreat,
    }
    // Creature Family
    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy)]
    pub enum CreatureFamily {
        CreatureFamilyMammal,
        CreatureFamilyBird,
        CreatureFamilyReptile,
        CreatureFamilyFish,
        CreatureFamilyInsect,
        CreatureFamilyCrustation,
        CreatureFamilyMollusk,
        CreatureFamilyOther,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy)]
    pub enum CreatureCategory {
        Creature,
        CreatureAnimal,
        CreatureSentient,
        CreatureBeast,
        CreatureMagical,
    }

    #[derive(PartialEq, Debug, Clone, EnumIter, Display, Copy)]
    pub enum CreatureDiet {
        CreatureDietCarnivorous,
        CreatureDietHerbivorous,
        CreatureDietOmnivorous,
        CreatureDietOther,
    }

    pub fn creature_tags() -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for tag in CreatureSize::iter() {
            output.push(tag.to_string());
        }
        for tag in CreatureFamily::iter() {
            output.push(tag.to_string())
        }

        return output;
    }
}
