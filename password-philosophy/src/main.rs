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
    positions: [usize; 2],
}

impl Policy {
    fn is_valid(&self, s: &str) -> bool {
        let [p1, p2] = self.positions;
        let chars = s.chars().collect::<Vec<_>>();
        let letter = &self.letter;

        if let [Some(first), Some(second)] = [chars.get(p1 - 1), chars.get(p2 - 1)] {
            return first == letter && second != letter || first != letter && second == letter;
        }

        true
    }
}

impl FromStr for Policy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s.split(&['-', ' '][..]).collect::<Vec<&str>>();

        if let [first, second, character] = bits[..] {
            let first = first.parse::<usize>()?;
            let second = second.parse::<usize>()?;
            let character = character.chars().next().unwrap();

            return Ok(Policy {
                letter: character,
                positions: [first, second],
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
                positions: [1, 3]
            }
        );
    }

    #[test]
    fn check_invalid_password() {
        let policy = Policy {
            letter: 'b',
            positions: [2, 5],
        };

        assert!(!policy.is_valid("aaaakgrffgfjgah"));
        assert!(!policy.is_valid("abaabaaaa"));
    }

    #[test]
    fn check_valid_password() {
        let policy = Policy {
            letter: 'b',
            positions: [2, 5],
        };

        assert!(policy.is_valid("abxcxaasd"));
        assert!(policy.is_valid("axaabew"));
    }
}
