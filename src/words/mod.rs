use rand::seq::IteratorRandom;
use std::fs;

pub struct Words {
    list: Vec<String>,
}

impl Words {
    pub fn new() -> Self {
        let buf = fs::read_to_string("words.txt").unwrap();
        let list: Vec<String> = buf
            .lines()
            .map(|line| line.trim())
            .map(String::from)
            .collect();
        Words { list }
    }

    pub fn sample(&mut self) -> Option<&String> {
        let mut rng = rand::thread_rng();
        self.list.iter().choose(&mut rng)
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
