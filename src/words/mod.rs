use rand::{rngs::ThreadRng, seq::IteratorRandom};
use std::fs;

pub struct Words {
    list: Vec<String>,
    rng: ThreadRng,
}

impl Words {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        let buf = fs::read_to_string("words.txt").unwrap();
        let list: Vec<String> = buf
            .lines()
            .map(|line| line.trim())
            .map(String::from)
            .collect();
        Words { list, rng }
    }

    pub fn sample(&mut self) -> Option<String> {
        self.list.iter().choose(&mut self.rng).map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_words() {
        let words = Words::new();
        assert_eq!(
            words.list[0..=1],
            vec!["vero".to_string(), "true".to_string()]
        );
    }

    #[test]
    fn sample_returns_a_random_word() {
        let mut words = Words::new();
        assert!(words.sample().is_some())
    }
}
