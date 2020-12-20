use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn main() {
    let mut bags: HashMap<String, HashSet<String>> = HashMap::new(); // inner bag, possible outer bags

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        let (outer, inners) = parse_rule(&line);

        bags.entry(outer.to_string()).or_insert_with(HashSet::new);
        for (_, inner) in &inners {
            bags.entry(inner.to_string())
                .or_insert_with(HashSet::new)
                .insert(outer.to_string());
        }
    }

    // compute transitive closure of the "shiny gold" containers

    let mut bags_closure = bags.get("shiny gold").unwrap().clone();
    let mut length = 0;

    // repeat until we stop adding bags
    while bags_closure.len() > length {
        length = bags_closure.len();

        let new_bags: Vec<_> = bags_closure
            .iter()
            .flat_map(|bag| bags.get(bag).unwrap())
            .collect();

        for bag in new_bags {
            bags_closure.insert(bag.clone());
        }
    }

    println!("{:?}", bags_closure.len());
}

fn parse_rule<'a>(text: &'a str) -> (&'a str, Vec<(usize, &'a str)>) {
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

            (n.parse::<usize>().unwrap(), bags)
        })
        .collect::<Vec<(usize, &str)>>();

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
