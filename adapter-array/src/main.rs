use std::{collections::HashMap, io::BufRead};

fn main() {
    let adapters: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .map(|it| it.unwrap().parse::<usize>().unwrap())
        .collect();

    let diffs = differences(&adapters);

    println!("Differences: {:?}", diffs);
}

fn differences(adapters: &[usize]) -> HashMap<usize, usize> {
    let mut adapters = adapters.to_vec();
    adapters.push(0);
    adapters.sort();

    let mut differences = HashMap::new();
    differences.insert(3, 1);

    for pair in adapters.windows(2) {
        let diff = pair[1] - pair[0];

        *differences.entry(diff).or_insert(0) += 1;
    }

    differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_differences() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let expected: HashMap<usize, usize> = vec![(1, 7), (3, 5)].into_iter().collect();

        assert_eq!(differences(&adapters), expected);
    }
}
