use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::str::FromStr;

enum State {
    Rules,
    Waiting,
    YourTicket,
    NearbyTicket,
}

fn main() {
    let mut state = State::Rules;
    let mut rules = vec![];
    let mut tickets = vec![];
    let mut ticket = Ticket(vec![]);

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        match (&state, line.as_ref()) {
            (_, "") => state = State::Waiting,
            (State::Rules, r) => rules.push(r.parse::<Rule>().unwrap()),
            (State::Waiting, "your ticket:") => state = State::YourTicket,
            (State::Waiting, "nearby tickets:") => state = State::NearbyTicket,
            (State::NearbyTicket, t) => tickets.push(t.parse::<Ticket>().unwrap()),
            (State::YourTicket, t) => ticket = t.parse::<Ticket>().unwrap(),
            _ => panic!("Bad input"),
        }
    }

    tickets = tickets
        .into_iter()
        .filter(|ticket| ticket.valid(&rules))
        .collect();

    let fields = infer_fields(&tickets, &rules);

    let answer: usize = fields
        .iter()
        .enumerate()
        .filter_map(|(i, name)| {
            if name.starts_with("departure") {
                Some(ticket.0[i])
            } else {
                None
            }
        })
        .product();

    println!("{}", answer);
}

fn infer_fields(tickets: &[Ticket], rules: &[Rule]) -> Vec<String> {
    let Ticket(first_ticket) = &tickets[0];

    let mut candidates: Vec<_> = (0..first_ticket.len())
        .map(|i| {
            let unique: HashSet<usize> = tickets.iter().map(|ticket| ticket.0[i]).collect();

            let matching = rules
                .iter()
                .filter_map(|rule| {
                    let Rule((name, _)) = rule;

                    if unique.iter().all(|v| rule.matches(v)) {
                        Some(name.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            (i, matching)
        })
        .collect();

    candidates.sort_by_key(|(_, cs)| cs.len());

    let mut assigned_fields: HashSet<String> = HashSet::new();
    let mut fields: Vec<_> = candidates
        .iter()
        .map(|(i, cdts)| {
            let unused: Vec<_> = cdts
                .iter()
                .filter(|c| !assigned_fields.contains(*c))
                .collect();

            assert_eq!(unused.len(), 1);
            assigned_fields.insert(unused[0].clone());

            (i, unused[0].clone())
        })
        .collect();

    fields.sort_by_key(|(i, _)| **i);

    fields.into_iter().map(|(_, f)| f).collect()
}

#[derive(Debug, PartialEq)]
struct Rule((String, (RangeInclusive<usize>, RangeInclusive<usize>)));

impl Rule {
    fn matches(&self, number: &usize) -> bool {
        let Rule((_, (first, second))) = self;

        first.contains(number) || second.contains(number)
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        // Don't look...
        let mut parts = input.split(": ");
        let field = parts.next().ok_or(format!("Invalid rule: {}", input))?;
        let ranges = parts.next().ok_or(format!("Invalid rule: {}", input))?;

        let mut rngs = ranges.split(" or ").map::<Result<_, String>, _>(|range| {
            let mut limits = range.split('-').map(|it| {
                it.parse::<usize>()
                    .map_err(|_| format!("Invalid range limit: {}", it))
            });

            let from = limits.next().ok_or(format!("Invalid range: {}", range))??;
            let to = limits.next().ok_or(format!("Invalid range: {}", range))??;

            Ok(from..=to)
        });

        let first = rngs.next().ok_or(format!("Invalid ranges: {}", ranges))??;
        let second = rngs.next().ok_or(format!("Invalid ranges: {}", ranges))??;

        Ok(Rule((field.to_string(), (first, second))))
    }
}

#[derive(Debug, PartialEq)]
struct Ticket(Vec<usize>);

impl FromStr for Ticket {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|it| {
                it.parse::<usize>()
                    .map_err(|_| format!("Invalid ticket {}", s))
            })
            .collect::<Result<_, _>>()
            .map(Ticket)
    }
}

impl Ticket {
    fn valid(&self, rules: &[Rule]) -> bool {
        !self
            .0
            .iter()
            .any(|number| rules.iter().all(|rule| !rule.matches(number)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_rules() {
        let input = "class: 1-3 or 5-7";
        let expected = Rule(("class".to_string(), (1..=3, 5..=7)));

        assert_eq!(input.parse::<Rule>().unwrap(), expected)
    }

    #[test]
    fn reads_ticket() {
        let input = "7,1,14";
        let expected = Ticket(vec![7, 1, 14]);

        assert_eq!(input.parse::<Ticket>().unwrap(), expected)
    }

    #[test]
    fn infers_fields() {
        let rules: Vec<_> = vec![
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
        ]
        .iter()
        .map(|it| it.parse::<Rule>().unwrap())
        .collect();

        let tickets = vec![
            Ticket(vec![3, 9, 18]),
            Ticket(vec![15, 1, 5]),
            Ticket(vec![5, 14, 9]),
        ];

        let expected = vec!["row".to_string(), "class".to_string(), "seat".to_string()];

        assert_eq!(infer_fields(&tickets, &rules), expected);
    }
}
