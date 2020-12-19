use regex::Regex;
use std::{collections::HashMap, io::BufRead};

fn main() {
    let mut passport: HashMap<String, String> = HashMap::new();
    let mut valid_count = 0;

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        if line == "" {
            if is_valid(&passport) {
                valid_count += 1;
            }

            passport.clear();
            continue;
        }

        for pair in line.split(' ') {
            if let [k, v, ..] = pair.split(':').take(2).collect::<Vec<_>>()[..] {
                passport.insert(k.to_string(), v.to_string());
            }
        }
    }

    if is_valid(&passport) {
        valid_count += 1;
    }

    println!("Valid passports: {}", valid_count);
}

fn is_valid(passport: &HashMap<String, String>) -> bool {
    let hgt_regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecls = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();

    let p_keys = passport.keys().collect::<Vec<_>>();
    let all_present = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|k| p_keys.contains(&&k.to_string()));

    all_present
        && passport.iter().all(|(k, v)| match k.as_ref() {
            "byr" => v.parse::<usize>().map_or(false, |y| y >= 1920 && y <= 2002),
            "iyr" => v.parse::<usize>().map_or(false, |y| y >= 2010 && y <= 2020),
            "eyr" => v.parse::<usize>().map_or(false, |y| y >= 2020 && y <= 2030),
            "hgt" => hgt_regex.captures(v).map_or(false, |c| match &c[2] {
                "cm" => c[1]
                    .parse::<usize>()
                    .map_or(false, |h| h >= 150 && h <= 193),
                "in" => c[1].parse::<usize>().map_or(false, |h| h >= 59 && h <= 76),
                _ => false,
            }),
            "hcl" => hcl_regex.is_match(v),
            "ecl" => ecls.contains(&v.as_ref()),
            "pid" => pid_regex.is_match(v),
            _ => true,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_invalid_passports() {
        let passport1 = vec![
            ("eyr", "1972"),
            ("cid", "100"),
            ("hcl", "#18171d"),
            ("ecl", "amb"),
            ("hgt", "170"),
            ("pid", "186cm"),
            ("iyr", "2018"),
            ("byr", "1926"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        let passport2 = vec![
            ("iyr", "2019"),
            ("hcl", "#602927"),
            ("eyr", "1967"),
            ("hgt", "170cm"),
            ("ecl", "grn"),
            ("pid", "012533040"),
            ("byr", "1946"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
        let passport3 = vec![
            ("hcl", "dab227"),
            ("iyr", "2012"),
            ("ecl", "brn"),
            ("hgt", "182cm"),
            ("pid", "021572410"),
            ("eyr", "2020"),
            ("byr", "1992"),
            ("cid", "277"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        let passport4 = vec![
            ("hgt", "59cm"),
            ("ecl", "zzz"),
            ("eyr", "2038"),
            ("hcl", "74454a"),
            ("iyr", "2023"),
            ("pid", "3556412378"),
            ("byr", "2007"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        assert!(
            !is_valid(&passport1),
            format!("{:?} should not be valid!", passport1)
        );
        assert!(
            !is_valid(&passport2),
            format!("{:?} should not be valid!", passport2)
        );
        assert!(
            !is_valid(&passport3),
            format!("{:?} should not be valid!", passport3)
        );
        assert!(
            !is_valid(&passport4),
            format!("{:?} should not be valid!", passport4)
        );
    }

    #[test]
    fn accepts_valid_passports() {
        let passport1 = vec![
            ("pid", "087499704"),
            ("hgt", "74in"),
            ("ecl", "grn"),
            ("iyr", "2012"),
            ("eyr", "2030"),
            ("byr", "1980"),
            ("hcl", "#623a2f"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        let passport2 = vec![
            ("eyr", "2029"),
            ("ecl", "blu"),
            ("cid", "129"),
            ("byr", "1989"),
            ("iyr", "2014"),
            ("pid", "896056539"),
            ("hcl", "#a97842"),
            ("hgt", "165cm"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        let passport3 = vec![
            ("hcl", "#888785"),
            ("hgt", "164cm"),
            ("byr", "2001"),
            ("iyr", "2015"),
            ("cid", "88"),
            ("pid", "545766238"),
            ("ecl", "hzl"),
            ("eyr", "2022"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        let passport4 = vec![
            ("iyr", "2010"),
            ("hgt", "158cm"),
            ("hcl", "#b6652a"),
            ("ecl", "blu"),
            ("byr", "1944"),
            ("eyr", "2021"),
            ("pid", "093154719"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

        assert!(
            is_valid(&passport1),
            format!("{:?} should be valid!", passport1)
        );
        assert!(
            is_valid(&passport2),
            format!("{:?} should be valid!", passport2)
        );
        assert!(
            is_valid(&passport3),
            format!("{:?} should be valid!", passport3)
        );
        assert!(
            is_valid(&passport4),
            format!("{:?} should be valid!", passport4)
        );
    }
}
