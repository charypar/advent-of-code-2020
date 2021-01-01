use std::collections::HashMap;

fn main() {
    println!(
        "2020th number is {}",
        Seq::new(&[2, 15, 0, 9, 1, 20]).nth(2019).unwrap()
    );
}

struct Seq {
    input: Vec<usize>,
    memory: HashMap<usize, usize>, // number, turn
    last: usize,
    turn: usize,
}

impl Seq {
    fn new(initial: &[usize]) -> Self {
        Self {
            input: initial.iter().rev().cloned().collect(),
            memory: HashMap::new(),
            last: 0,
            turn: 1,
        }
    }
}

impl Iterator for Seq {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = if self.input.is_empty() {
            match self.memory.get(&self.last) {
                Some(number) => self.turn - 1 - number,
                None => 0,
            }
        } else {
            self.input.pop().unwrap()
        };

        if self.turn > 1 {
            self.memory.insert(self.last, self.turn - 1);
        }

        self.last = current;
        self.turn += 1;

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterates_correctly() {
        let actual: Vec<usize> = Seq::new(&[0, 3, 6]).take(10).collect();
        let expected = vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0];

        assert_eq!(actual, expected);
    }

    #[test]
    fn finds_2020th() {
        assert_eq!(Seq::new(&[0, 3, 6]).nth(2019).unwrap(), 436);

        assert_eq!(Seq::new(&[1, 3, 2]).nth(2019).unwrap(), 1);
        assert_eq!(Seq::new(&[2, 1, 3]).nth(2019).unwrap(), 10);
        assert_eq!(Seq::new(&[1, 2, 3]).nth(2019).unwrap(), 27);
        assert_eq!(Seq::new(&[2, 3, 1]).nth(2019).unwrap(), 78);
        assert_eq!(Seq::new(&[3, 2, 1]).nth(2019).unwrap(), 438);
        assert_eq!(Seq::new(&[3, 1, 2]).nth(2019).unwrap(), 1836);
    }
}
