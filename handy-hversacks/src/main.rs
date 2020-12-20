use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let mut bags: HashMap<String, Vec<(u64, String)>> = HashMap::new();

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        let (outer, inners) = parse_rule(&line);

        bags.insert(
            outer.to_string(),
            inners.iter().map(|(n, b)| (*n, b.to_string())).collect(),
        );
    }

    println!("{:?}", count_bags(&bags, "shiny gold", 0) - 1);
}

fn count_bags(rules: &HashMap<String, Vec<(u64, String)>>, colour: &str, depth: usize) -> u64 {
    let prefix = vec!["| "; depth].join("");

    println!(
        "{}With {} bag, I also need to take {:?}.",
        prefix, colour, rules[colour]
    );

    let total = 1 + rules[colour]
        .iter()
        .map(|(n, col)| n * count_bags(rules, col, depth + 1))
        .sum::<u64>();

    println!("{}That's a total of {} bags.", prefix, total - 1);

    total
}

fn parse_rule<'a>(text: &'a str) -> (&'a str, Vec<(u64, &'a str)>) {
    let bags_pattern = Regex::new(r"(\d)+ ([a-z ]+) bags?[,.]?").unwrap();
    let parts = text.split(" bags contain ").collect::<Vec<_>>();

    if parts[1] == "no other bags." {
        return (parts[0], vec![]);
    }

    let pairs = parts[1]
        .split(", ")
        .map(|bags| {
            let captures = bags_pattern.captures(bags).unwrap();
            let n = captures.get(1).unwrap().as_str();
            let bags = captures.get(2).unwrap().as_str();

            (n.parse::<u64>().unwrap(), bags)
        })
        .collect::<Vec<(u64, &str)>>();

    (parts[0], pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_rule() {
        let expected = ("light red", vec![(1, "bright white"), (2, "muted yellow")]);

        assert_eq!(
            parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            expected
        )
    }
}
