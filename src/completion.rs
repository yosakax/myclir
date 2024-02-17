use rand;
use rand::Rng;

pub struct Completion {
    pub list: Vec<String>,
    pub preserved_words: Vec<String>,
}

impl Completion {
    pub fn new() -> Self {
        Self {
            list: vec![],
            preserved_words: vec![],
        }
    }

    pub fn update(&mut self) {
        self.generate_random_word();
    }

    fn generate_random_word(&mut self) {
        self.list.clear();

        let characters = "abcdefghijklmnopqrstuvwxyz";

        for _ in 0..5 {
            let mut random_word = String::new();
            for _ in 0..rand::thread_rng().gen_range(3..10) {
                // Generate a random index to select a character from the list
                let random_index = rand::thread_rng().gen_range(0..characters.len());

                // Get the random character from the list
                let random_character = characters.chars().nth(random_index).unwrap();
                random_word.push(random_character);
            }

            // Return the random character as a word
            self.list.push(random_word);
        }
    }
}
