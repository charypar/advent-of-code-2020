use std::collections::HashMap;
use std::io::{self, BufRead};

struct Index {
    numbers: Vec<usize>,
    pairwise_sums: HashMap<usize, (usize, usize)>,
}

impl Index {
    fn new() -> Index {
        Index {
            numbers: Vec::new(),
            pairwise_sums: HashMap::new(),
        }
    }

    fn check_pair(&self, n: usize, total: usize) -> Option<usize> {
        for num in &self.numbers {
            if n + num == total {
                return Some(*num);
            }
        }

        None
    }

    fn check_triple(&self, n: usize, total: usize) -> Option<(usize, usize)> {
        for num in self.pairwise_sums.keys() {
            if n + num == total {
                return Some(self.pairwise_sums[num]);
            }
        }

        None
    }

    fn push(&mut self, n: usize) {
        for num in &self.numbers {
            self.pairwise_sums.insert(*num + n, (*num, n));
        }
        self.numbers.push(n);
    }

    #[allow(dead_code)]
    fn push_vec(&mut self, ns: Vec<usize>) {
        for n in ns {
            self.push(n);
        }
    }
}

fn main() {
    let mut index = Index::new();
    let total = 2020;

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            if let Ok(number) = line.parse::<usize>() {
                if let Some(pair) = index.check_pair(number, total) {
                    println!("Found a pair match: {} + {} == {}", number, pair, total,);
                    println!("Pair answer is {} * {} = {}", number, pair, number * pair)
                }

                if let Some(triple) = index.check_triple(number, total) {
                    println!(
                        "Found a triple match: {} + {} + {} == {}",
                        number, triple.0, triple.1, total,
                    );
                    println!(
                        "Triple answer is {} * {} * {} = {}",
                        number,
                        triple.0,
                        triple.1,
                        number * triple.0 * triple.1
                    )
                }

                index.push(number);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_empty_list() {
        let index = Index::new();
        let expected = None;
        let actual = index.check_pair(3, 10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_no_pair_match() {
        let mut index = Index::new();
        index.push_vec(vec![1, 2, 3]);

        let expected = None;
        let actual = index.check_pair(3, 10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_pair_match() {
        let mut index = Index::new();
        index.push_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);

        let expected = Some(7);
        let actual = index.check_pair(3, 10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_triple_match() {
        let mut index = Index::new();
        index.push_vec(vec![1, 2, 3, 4]);

        let expected = Some((3, 4));
        let actual = index.check_triple(3, 10);

        assert_eq!(expected, actual);
    }
}
