use std::io::BufRead;

fn main() {
    let numbers: Vec<u64> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();

    let window_size = 25;

    for window in numbers.windows(window_size + 1) {
        let terms = &window[0..window_size];
        let sum = window[window_size];

        if !is_valid(terms, sum) {
            return println!("{} is not valid!", sum);
        }
    }
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
