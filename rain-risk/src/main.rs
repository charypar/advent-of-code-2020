use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
enum Cmd {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    L(i64),
    R(i64),
    F(i64),
}

impl FromStr for Cmd {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (letter, number) = (&s[0..1], &s[1..]);
        let number = number
            .parse::<i64>()
            .map_err(|_| "parameter not a number")?;

        match letter {
            "N" => Ok(Cmd::N(number)),
            "S" => Ok(Cmd::S(number)),
            "E" => Ok(Cmd::E(number)),
            "W" => Ok(Cmd::W(number)),
            "L" => Ok(Cmd::L(number)),
            "R" => Ok(Cmd::R(number)),
            "F" => Ok(Cmd::F(number)),
            _ => Err("unknown command"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn transform(&self, deg: i64, point: (i64, i64)) -> (i64, i64) {
        let t90 = [[0, -1], [1, 0]];
        let t180 = [[-1, 0], [0, -1]];
        let t270 = [[0, 1], [-1, 0]];

        let t = match (self, deg) {
            (Direction::Right, 90) | (Direction::Left, 270) => t270,
            (_, 180) => t180,
            (Direction::Right, 270) | (Direction::Left, 90) => t90,
            (_, _) => unreachable!(),
        };

        let (x, y) = point;

        (x * t[0][0] + y * t[0][1], x * t[1][0] + y * t[1][1])
    }
}

#[derive(Debug, Clone, Copy)]
struct NavData {
    position: (i64, i64),
    waypoint: (i64, i64), // relative to position
}

impl NavData {
    fn step(self, command: Cmd) -> NavData {
        let NavData { position, waypoint } = self;
        let (px, py) = position;
        let (wx, wy) = waypoint;

        match command {
            Cmd::N(n) => NavData {
                position,
                waypoint: (wx, wy + n),
            },
            Cmd::S(n) => NavData {
                position,
                waypoint: (wx, wy - n),
            },
            Cmd::E(n) => NavData {
                position,
                waypoint: (wx + n, wy),
            },
            Cmd::W(n) => NavData {
                position,
                waypoint: (wx - n, wy),
            },
            Cmd::L(n) => NavData {
                position,
                waypoint: Direction::Left.transform(n, waypoint),
            },
            Cmd::R(n) => NavData {
                position,
                waypoint: Direction::Right.transform(n, waypoint),
            },
            Cmd::F(n) => {
                let (dx, dy) = (wx * n, wy * n);

                NavData {
                    position: (px + dx, py + dy),
                    waypoint,
                }
            }
        }
    }
}

fn main() {
    let initial = NavData {
        position: (0, 0),
        waypoint: (10, 1),
    };
    let data = std::io::stdin()
        .lock()
        .lines()
        .fold(initial, |state, line| {
            let cmd = line.unwrap().parse().unwrap();

            state.step(cmd)
        });

    let (x, y) = data.position;

    println!("Done, ({}, {})", x, y);
    println!("Distance: {}", x.abs() + y.abs());
}

#[cfg(test)]
mod tests {
    use super::Direction::*;

    #[test]
    fn test_turning() {
        let point = (10, 1);

        assert_eq!(Left.transform(180, point), (-10, -1));
        assert_eq!(Right.transform(180, point), (-10, -1));

        assert_eq!(Left.transform(90, point), (-1, 10));
        assert_eq!(Left.transform(270, point), (1, -10));

        assert_eq!(Right.transform(90, point), (1, -10));
        assert_eq!(Right.transform(270, point), (-1, 10));
    }
}
