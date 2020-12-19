use std::{collections::HashSet, io::BufRead};

fn main() {
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let mut passport: HashSet<String> = HashSet::new();
    let mut valid_count = 0;

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        if line == "" {
            if is_valid(&passport, &keys) {
                valid_count += 1;
            }

            passport.clear();
            continue;
        }

        for pair in line.split(' ') {
            passport.insert(pair.split(':').next().unwrap().to_string());
        }
    }

    if is_valid(&passport, &keys) {
        valid_count += 1;
    }

    println!("Valid passports: {}", valid_count);
}

fn is_valid(passport: &HashSet<String>, keys: &[&str]) -> bool {
    for key in keys {
        if *key != "cid" && !passport.contains(&key.to_string()) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn validates_a_full_passport() {
        let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        let passport = keys.iter().map(|s| s.to_string()).collect::<HashSet<_>>();

        assert!(is_valid(&passport, &keys));
    }

    #[test]
    fn validates_a_full_passport_without_cid() {
        let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        let passport = keys[0..7]
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();

        assert!(is_valid(&passport, &keys));
    }

    #[test]
    fn rejects_a_partial_passport() {
        let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        let passport = vec!["byr", "eyr", "hgt", "ecl"]
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();

        assert!(!is_valid(&passport, &keys));
    }
}
