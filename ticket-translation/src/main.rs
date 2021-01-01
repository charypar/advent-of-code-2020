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

    println!("Rules: {:?}", rules);
    println!("My ticket: {:?}", ticket);
    println!("Nearby tickets: {:?}", tickets);

    let invalids = invalids(&tickets, &rules);
    println!(
        "Invalid values: {:?}, total = {}",
        invalids,
        invalids.iter().sum::<usize>()
    );
}

fn invalids(tickets: &[Ticket], rules: &[Rule]) -> Vec<usize> {
    let mut invalids = vec![];

    for ticket in tickets {
        for number in &ticket.0 {
            let none_match = rules.iter().all(|rule| {
                let Rule((_, (first, second))) = rule;

                !first.contains(&number) && !second.contains(&number)
            });

            if none_match {
                invalids.push(*number);
            }
        }
    }

    invalids
}

#[derive(Debug, PartialEq)]
struct Rule((String, (RangeInclusive<usize>, RangeInclusive<usize>)));

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
}
