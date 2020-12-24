use std::io::BufRead;

enum Cmd {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    L(i64),
    R(i64),
    F(i64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn left(&self, d: i64) -> Self {
        let shift = d / 90;
        let compas = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];

        let i = compas.iter().position(|dir| dir == self).unwrap() as i64;

        compas[(i + shift) as usize % 4]
    }

    fn right(&self, d: i64) -> Self {
        self.left(-d)
    }

    fn forward(&self, d: i64) -> (i64, i64) {
        match self {
            Direction::North => (0, d),
            Direction::West => (-d, 0),
            Direction::South => (0, -d),
            Direction::East => (d, 0),
        }
    }
}

fn main() {
    let (x, y, facing) = std::io::stdin()
        .lock()
        .lines()
        .fold((0, 0, Direction::East), |state, line| {
            step(state, parse(&line.unwrap()))
        });

    println!("Done, ({}, {}) facing {:?}", x, y, facing);
    println!("Distance: {}", x.abs() + y.abs());
}

fn step(position: (i64, i64, Direction), command: Cmd) -> (i64, i64, Direction) {
    let (x, y, facing) = position;
    match command {
        Cmd::N(n) => (x, y + n, facing),
        Cmd::S(n) => (x, y - n, facing),
        Cmd::E(n) => (x + n, y, facing),
        Cmd::W(n) => (x - n, y, facing),
        Cmd::L(n) => (x, y, facing.left(n)),
        Cmd::R(n) => (x, y, facing.right(n)),
        Cmd::F(n) => {
            let (dx, dy) = facing.forward(n);

            (x + dx, y + dy, facing)
        }
    }
}

fn parse(command: &str) -> Cmd {
    let (letter, number) = (&command[0..1], &command[1..]);
    let number = number.parse::<i64>().unwrap();

    match letter {
        "N" => Cmd::N(number),
        "S" => Cmd::S(number),
        "E" => Cmd::E(number),
        "W" => Cmd::W(number),
        "L" => Cmd::L(number),
        "R" => Cmd::R(number),
        "F" => Cmd::F(number),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turning_left() {
        let direction = Direction::East;

        assert_eq!(direction.left(90), Direction::North);
        assert_eq!(direction.left(180), Direction::West);
        assert_eq!(direction.left(270), Direction::South);
    }

    #[test]
    fn turning_right() {
        let direction = Direction::East;

        assert_eq!(direction.right(90), Direction::South);
        assert_eq!(direction.right(180), Direction::West);
        assert_eq!(direction.right(270), Direction::North);
    }
}
