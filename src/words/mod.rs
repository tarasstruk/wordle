use rand::seq::IteratorRandom;
use std::env;
use std::fs;
use std::include_str;

pub struct Words {
    list: Vec<String>,
}

impl Words {
    pub fn new() -> Self {
        let list = Self::read_dictionary_from_file()
            .or_else(|| Self::get_default_dictionary())
            .unwrap();
        Words { list }
    }

    fn read_dictionary_from_file() -> Option<Vec<String>> {
        // if the path to file is not provided we skip the rest
        let path = env::args().skip(1).next()?;
        match fs::read_to_string(path) {
            Ok(buf) => Some(Self::parse_content(&buf)),
            Err(err) => {
                println!("Eror reading file: {:?}", err);
                None
            }
        }
    }

    fn parse_content(buf: &str) -> Vec<String> {
        buf.lines()
            .map(|line| line.trim())
            .map(String::from)
            .collect()
    }

    fn get_default_dictionary() -> Option<Vec<String>> {
        let buf = include_str!("words.txt");
        Some(Self::parse_content(buf))
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
