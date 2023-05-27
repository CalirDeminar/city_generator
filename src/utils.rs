pub mod utils {
    use rand::seq::SliceRandom;
    pub fn random_pick<T: Clone>(input: &Vec<T>) -> T {
        let mut rng = rand::thread_rng();
        let mut i = input.clone();
        i.shuffle(&mut rng);
        return i.pop().unwrap();
    }
}