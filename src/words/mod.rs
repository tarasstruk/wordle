use rand::seq::IteratorRandom;
use std::env;
use std::fs;
use std::include_str;

pub struct Words {
    list: Vec<String>,
}

enum DictionarySource {
    DiskFile(String),
    Default,
}

impl DictionarySource {
    fn new() -> Self {
        match env::args().skip(1).next() {
            Some(path) => Self::DiskFile(path),
            None => Self::Default,
        }
    }
}

impl Words {
    pub fn new() -> Self {
        let dict = Self::read_dictionary();
        let list = Self::parse_content(&dict);
        Words { list }
    }

    // it always returns a string or fails when the file cannot be read
    fn read_dictionary() -> String {
        match DictionarySource::new() {
            DictionarySource::DiskFile(path) => fs::read_to_string(path).unwrap(),
            DictionarySource::Default => {
                println!("Loading the default dictionary.");
                include_str!("words.txt").into()
            }
        }
    }

    fn parse_content(buf: &str) -> Vec<String> {
        buf.lines()
            .map(|line| line.trim())
            .map(String::from)
            .collect()
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
