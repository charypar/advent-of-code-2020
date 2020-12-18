use anyhow::bail;

use std::{
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let mut valid_count = 0;

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            let parts: Vec<_> = line.split(": ").collect();

            if let [policy, password] = parts[..] {
                if let Ok(policy) = policy.parse::<Policy>() {
                    if policy.is_valid(password) {
                        valid_count += 1;
                    }
                }
            }
        }
    }

    println!("{} passwords are valid.", valid_count);
}

#[derive(Debug, PartialEq)]
struct Policy {
    letter: char,
    reps: std::ops::Range<usize>,
}

impl Policy {
    fn is_valid(&self, s: &str) -> bool {
        let (matches, _) = s.chars().partition::<Vec<_>, _>(|c| *c == self.letter);

        self.reps.contains(&matches.len())
    }
}

impl FromStr for Policy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s.split(&['-', ' '][..]).collect::<Vec<&str>>();

        if let [from, to, character] = bits[..] {
            let from = from.parse::<usize>()?;
            let to = to.parse::<usize>()?;
            let character = character.chars().next().unwrap();

            return Ok(Policy {
                letter: character,
                reps: from..(to + 1),
            });
        }

        bail!("Bad input!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_string() {
        assert!("".parse::<Policy>().is_err())
    }

    #[test]
    fn parse_bad_string() {
        assert!("1-3-4 x x".parse::<Policy>().is_err())
    }

    #[test]
    fn parse_valid_policy_string() {
        assert_eq!(
            "1-3 x".parse::<Policy>().unwrap(),
            Policy {
                letter: 'x',
                reps: 1..4
            }
        );
    }

    #[test]
    fn check_invalid_password() {
        let policy = Policy {
            letter: 'b',
            reps: 2..5,
        };

        assert!(!policy.is_valid("aaaa"));
        assert!(!policy.is_valid("abaaa"));
        assert!(!policy.is_valid("ababbaabb"));
    }

    #[test]
    fn check_valid_password() {
        let policy = Policy {
            letter: 'b',
            reps: 2..5,
        };

        assert!(policy.is_valid("bb"));
        assert!(policy.is_valid("bbb"));
        assert!(policy.is_valid("bbbb"));

        assert!(policy.is_valid("bbaahahhahazu"));
        assert!(policy.is_valid("dasdbbaahhhahazu"));
        assert!(policy.is_valid("dasdbb"));
        assert!(policy.is_valid("dasdbadfabasdas"));
    }
}
