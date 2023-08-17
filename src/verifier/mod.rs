#[derive(Debug, PartialEq, Eq)]
pub enum GuessResult {
    Guessed,
    Missed { hint: String },
}

#[derive(Debug)]
pub struct Verifier {
    secret: String,
}

impl Verifier {
    pub fn new(secret: &str) -> Self {
        Verifier {
            secret: secret.to_owned(),
        }
    }

    pub fn verify(&mut self, input: &str) -> GuessResult {
        if input == self.secret {
            return GuessResult::Guessed;
        }

        let secret_chars = self.secret.chars().collect::<Vec<_>>();

        let hint: String = input
            .chars()
            .enumerate()
            .map(|(index, ch)| {
                if secret_chars.get(index) == Some(&ch) {
                    '+'
                } else if secret_chars.contains(&ch) {
                    '?'
                } else {
                    '-'
                }
            })
            .collect();
        GuessResult::Missed { hint }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_missed_variant_0() {
        let mut verifier = Verifier::new("baba");
        assert_eq!(
            verifier.verify("buba"),
            GuessResult::Missed {
                hint: "+-++".to_owned()
            }
        );
    }

    #[test]
    fn verify_missed_variant_1() {
        let mut verifier = Verifier::new("beco");
        assert_eq!(
            verifier.verify("ecoo"),
            GuessResult::Missed {
                hint: "???+".to_owned()
            }
        );
    }

    #[test]
    fn verify_missed_variant_2() {
        let mut verifier = Verifier::new("mohn");
        assert_eq!(
            verifier.verify("moor"),
            GuessResult::Missed {
                hint: "++?-".to_owned()
            }
        );
    }

    #[test]
    fn verify_guessed() {
        let mut verifier = Verifier::new("tier");
        assert_eq!(verifier.verify("tier"), GuessResult::Guessed);
    }
}
