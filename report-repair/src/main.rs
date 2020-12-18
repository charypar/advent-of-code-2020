use std::io::{self, BufRead};

fn main() {
    let mut numbers: Vec<usize> = Vec::new();
    let total = 2020;

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            if let Ok(number) = line.parse::<usize>() {
                if let Some(pair) = check(&numbers, number, total) {
                    println!("Found match: {} + {} == {}", number, pair, total,);
                    println!("Answer is {} * {} = {}", number, pair, number * pair)
                } else {
                    numbers.push(number);
                }
            }
        }
    }
}

fn check(numbers: &Vec<usize>, n: usize, total: usize) -> Option<usize> {
    for num in numbers {
        if n + num == total {
            return Some(*num);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_empty_list() {
        let list = Vec::new();
        let expected = None;
        let actual = check(&list, 3, 10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_no_match() {
        let list = vec![1, 2, 3];
        let expected = None;
        let actual = check(&list, 3, 10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn check_match() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = Some(7);
        let actual = check(&list, 3, 10);

        assert_eq!(expected, actual);
    }
}
