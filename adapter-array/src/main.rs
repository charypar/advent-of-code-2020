use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let adapters: Vec<u64> = std::io::stdin()
        .lock()
        .lines()
        .map(|it| it.unwrap().parse::<u64>().unwrap())
        .collect();

    let diffs = differences(&adapters);
    println!("Differences: {:?}", diffs);

    let runs = runs_of_ones(&diffs);
    println!("Runs of ones: {:?}", runs);

    let ways = runs
        .into_iter()
        .map(|r| rewrites(&vec![1; r as usize]).len())
        .collect::<Vec<_>>();

    println!(
        "Ways of rewriting runs: {:?} total {}",
        ways,
        ways.iter().product::<usize>()
    );
}

fn runs_of_ones(list: &[u64]) -> Vec<u64> {
    if list.len() < 2 {
        if list[0] == 1 {
            return vec![1];
        }

        return vec![];
    }

    let mut runs = Vec::new();
    let mut length = 1;

    for i in 1..list.len() {
        if list[i] != list[i - 1] {
            if list[i - 1] == 1 {
                runs.push(length);
            }

            length = 1;
        } else {
            length += 1;
        }
    }

    if list[list.len() - 1] == 1 {
        runs.push(length);
    }

    runs
}

fn rewrites(run: &[usize]) -> HashSet<Vec<usize>> {
    let mut results = HashSet::new();
    results.insert(run.to_vec());

    if run.len() < 2 {
        return results;
    }

    if run.len() < 3 {
        results.insert(vec![2]);
        return results;
    }

    results.extend(rewrites_n(run, 2));
    results.extend(rewrites_n(run, 3));

    results
}

fn rewrites_n(run: &[usize], n: usize) -> HashSet<Vec<usize>> {
    let mut results = HashSet::new();
    results.insert(run.to_vec());

    for a in 0..=(run.len() - n) {
        for f in &rewrites(&vec![1; a]) {
            for b in &rewrites(&vec![1; run.len() - n - a]) {
                results.insert([f as &[usize], b as &[usize]].join(&n));
            }
        }
    }

    results
}

fn differences(adapters: &[u64]) -> Vec<u64> {
    let mut adapters = adapters.to_vec();
    adapters.push(0);
    adapters.sort();

    let mut chain: Vec<_> = adapters.windows(2).map(|pair| pair[1] - pair[0]).collect();
    chain.push(3);

    chain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_differences() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let expected = vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3];

        assert_eq!(differences(&adapters), expected);
    }

    #[test]
    fn generates_rewrites_for_empty() {
        let expected: HashSet<Vec<usize>> = vec![vec![]].into_iter().collect();
        assert_eq!(rewrites(&[]), expected);
    }

    #[test]
    fn generates_rewrites_for_1() {
        let expected: HashSet<Vec<usize>> = vec![vec![1]].into_iter().collect();
        assert_eq!(rewrites(&[1]), expected);
    }

    #[test]
    fn generates_rewrites_for_11() {
        let expected: HashSet<Vec<usize>> = vec![vec![1, 1], vec![2]].into_iter().collect();
        assert_eq!(rewrites(&[1, 1]), expected);
    }

    #[test]
    fn generates_rewrites_for_111() {
        let expected: HashSet<Vec<usize>> = vec![vec![1, 1, 1], vec![2, 1], vec![1, 2], vec![3]]
            .into_iter()
            .collect();
        assert_eq!(rewrites(&[1, 1, 1]), expected);
    }

    #[test]
    fn generates_rewrites_for_1111() {
        let expected: HashSet<Vec<usize>> = vec![
            vec![1, 1, 1, 1],
            vec![2, 1, 1],
            vec![2, 2],
            vec![1, 2, 1],
            vec![1, 1, 2],
            vec![3, 1],
            vec![1, 3],
        ]
        .into_iter()
        .collect();
        assert_eq!(rewrites(&[1, 1, 1, 1]), expected);
    }

    #[test]
    fn measure_runs() {
        assert_eq!(runs_of_ones(&[1]), vec![1]);
        assert_eq!(runs_of_ones(&[2]), vec![]);
        assert_eq!(runs_of_ones(&[3, 1, 1]), vec![2]);
        assert_eq!(runs_of_ones(&[3, 1, 1, 1]), vec![3]);
        assert_eq!(runs_of_ones(&[3, 3, 1, 1, 2, 2]), vec![2]);
        assert_eq!(runs_of_ones(&[1, 1, 2, 1, 1, 1]), vec![2, 3]);
    }
}
