use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Square {
    Empty,
    Tree,
}

struct Walk<'a> {
    map: &'a [Vec<Square>],
    width: usize,
    right: usize,
    down: usize,
    y: usize,
    x: usize,
}

impl<'a> Walk<'a> {
    fn new(map: &'a [Vec<Square>], right: usize, down: usize) -> Self {
        let width = map[0].len();

        Walk {
            map,
            width,
            right,
            down,
            y: 0,
            x: 0,
        }
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = Square;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        if self.map.len() <= self.y {
            return None;
        }

        let item = self.map[self.y][self.x];

        self.x = (self.x + self.right) % self.width;
        self.y += self.down;

        Some(item)
    }
}

fn main() {
    let mut map: Vec<Vec<Square>> = Vec::new();

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            map.push(parse_row(&line));
        }
    }

    let slopes = vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

    let result: u128 = slopes.iter().fold(1, |prod, [right, down]| {
        prod * Walk::new(&map, *right, *down).fold(0, |sum, square| {
            if let Square::Tree = square {
                sum + 1
            } else {
                sum
            }
        })
    });

    println!("Hit {} trees", result);
}

fn parse_row(row: &str) -> Vec<Square> {
    row.chars()
        .map(|c| match c {
            '.' => Square::Empty,
            '#' => Square::Tree,
            _ => Square::Empty,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Square::*, *};

    #[test]
    fn parses_a_row() {
        let expected = vec![
            Empty, Tree, Empty, Empty, Empty, Empty, Tree, Empty, Empty, Tree, Empty,
        ];

        assert_eq!(parse_row(".#....#..#."), expected);
    }
}
