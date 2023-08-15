#[derive(Debug, PartialEq, Eq)]
pub enum GuessChar {
    Hit(char),
    Miss(char),
    Move(char),
}

#[derive(Debug)]
pub struct Verifier {
    input: Vec<GuessChar>,
    secret: Vec<char>,
}

impl Verifier {
    pub fn new(secret: &str) -> Self {
        Verifier {
            input: Vec::new(),
            secret: secret.chars().collect(),
        }
    }

    pub fn hint(&self) -> String {
        let mut output = String::new();
        for item in self.input.iter() {
            match item {
                GuessChar::Hit(_) => output.push('+'),
                GuessChar::Miss(_) => output.push('-'),
                GuessChar::Move(_) => output.push('?'),
            }
        }
        output
    }

    fn parse_input(input: &str) -> Vec<GuessChar> {
        input.trim().chars().map(GuessChar::Miss).collect()
    }

    pub fn verify(&mut self, input: &str) -> bool {
        self.input = Self::parse_input(input);

        for (gidx, guess) in self.input.iter_mut().enumerate() {
            for (sidx, secret) in self.secret.iter().enumerate() {
                // println!("guess: {:?}, gid: {}, sid: {}", guess, &gidx, &sidx);
                if let GuessChar::Miss(ch) = guess {
                    if ch == secret && gidx == sidx {
                        *guess = GuessChar::Hit(*ch)
                    }
                }
            }
            for (sidx, secret) in self.secret.iter().enumerate() {
                // println!("guess: {:?}, gid: {}, sid: {}", guess, &gidx, &sidx);
                if let GuessChar::Miss(ch) = guess {
                    if ch == secret && gidx != sidx {
                        *guess = GuessChar::Move(*ch)
                    }
                }
            }
        }
        self.input.iter().all(|item| match item {
            GuessChar::Hit(_) => true,
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use GuessChar::*;

    #[test]
    fn new_saves_secret() {
        let verifier = Verifier::new("abcd");
        assert_eq!(verifier.secret, vec!['a', 'b', 'c', 'd'])
    }

    #[test]
    fn verify_verifies() {
        let mut verifier = Verifier::new("baba");
        assert!(!verifier.verify("buba"));
        assert_eq!(
            verifier.input,
            vec![Hit('b'), Miss('u'), Hit('b'), Hit('a')]
        )
    }

    #[test]
    fn verify_variant_1() {
        let mut verifier = Verifier::new("beco");
        assert!(!verifier.verify("ecoo"));
        assert_eq!(
            verifier.input,
            vec![Move('e'), Move('c'), Move('o'), Hit('o')]
        )
    }

    #[test]
    fn verify_variant_2() {
        let mut verifier = Verifier::new("mohn");
        assert!(!verifier.verify("moor"));
        assert_eq!(
            verifier.input,
            vec![Hit('m'), Hit('o'), Move('o'), Miss('r')]
        )
    }

    #[test]
    fn verify_variant_3() {
        let mut verifier = Verifier::new("tier");
        assert!(verifier.verify("tier"));
        assert_eq!(verifier.input, vec![Hit('t'), Hit('i'), Hit('e'), Hit('r')])
    }

    #[test]
    fn verify_hint() {
        let mut verifier = Verifier::new("tier");
        assert!(!verifier.verify("teor"));
        assert_eq!(verifier.hint(), "+?-+")
    }
}
