use std::io::BufRead;

fn main() {
    let numbers: Vec<u64> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();

    if let Some(invalid) = find_number(&numbers, 25) {
        println!("Found weakness: {}", invalid);

        for size in 2..numbers.len() {
            for set in numbers.windows(size) {
                if set.iter().sum::<u64>() == invalid {
                    let weakness = set.to_vec();
                    let min = weakness.iter().min().unwrap();
                    let max = weakness.iter().max().unwrap();

                    println!(
                        "{:?} add up to {}. Min: {}, Max: {}, Sum: {}",
                        set,
                        invalid,
                        min,
                        max,
                        min + max
                    )
                }
            }
        }
    }
}

fn find_number(numbers: &[u64], window_size: usize) -> Option<u64> {
    for window in numbers.windows(window_size + 1) {
        let terms = &window[0..window_size];
        let sum = window[window_size];

        if !is_valid(terms, sum) {
            return Some(sum);
        }
    }

    None
}

fn is_valid(terms: &[u64], sum: u64) -> bool {
    for i in 0..terms.len() {
        for j in 0..terms.len() {
            if terms[i] + terms[j] == sum {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_valid_number() {
        let terms = vec![35, 20, 15, 25, 47];
        let good_sum = 40;
        let bad_sum = 127;

        assert!(is_valid(&terms, good_sum));
        assert!(!is_valid(&terms, bad_sum));
    }
}
