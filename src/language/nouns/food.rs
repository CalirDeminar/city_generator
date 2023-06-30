pub mod food {

    #[derive(PartialEq, Debug, Clone)]
    pub enum FoodConditionTags {
        Food,
        BrewableWine,
        BrewableBeer,
        BrewableCider,
        BrewableMead,
        BrewableAle,
        BrewableRum,
        BrewableWhiskey,
        Fruit,
        Grain,
        Leaf,
    }
}
