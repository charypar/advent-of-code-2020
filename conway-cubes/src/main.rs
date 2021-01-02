use core::str::FromStr;
use std::fmt::Debug;
use std::io::Read;
use std::{collections::HashMap, ops::RangeInclusive};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut space: Space = input.parse().unwrap();

    for _ in 1..=6 {
        space.step(&step);
    }

    println!("{}", space.0.len());
}

fn step(cube: &Cube, neighbours: &[&Cube]) -> Cube {
    let active_neighbours = neighbours
        .iter()
        .filter(|it| match it {
            Cube::Active => true,
            _ => false,
        })
        .count();

    match (cube, active_neighbours) {
        (Cube::Active, (2..=3)) => Cube::Active,
        (Cube::Inactive, 3) => Cube::Active,
        _ => Cube::Inactive,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate((isize, isize, isize));

impl Coordinate {
    fn neighbourhood(&self) -> Vec<Coordinate> {
        let Coordinate((x, y, z)) = self;

        (-1..=1)
            .flat_map(move |i| {
                (-1..=1).flat_map(move |j| {
                    (-1..=1).filter_map(move |k| {
                        if (i, j, k) != (0, 0, 0) {
                            Some(Coordinate((*x + i, *y + j, *z + k)))
                        } else {
                            None
                        }
                    })
                })
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cube {
    Active,
    Inactive,
}

#[derive(PartialEq)]
struct Space(HashMap<Coordinate, Cube>);

impl Space {
    fn get(&self, c: &Coordinate) -> &Cube {
        self.0.get(c).unwrap_or(&Cube::Inactive)
    }

    fn step<F>(&mut self, step_fn: F)
    where
        F: Fn(&Cube, &[&Cube]) -> Cube,
    {
        let mut next = HashMap::new();

        for cell in self.0.keys() {
            for c in cell.neighbourhood() {
                let neighbours = c
                    .neighbourhood()
                    .iter()
                    .map(|it| self.get(it))
                    .collect::<Vec<_>>();
                let cube = self.get(&c);

                assert_eq!(neighbours.len(), 26);

                if let Cube::Active = step_fn(cube, &neighbours) {
                    next.insert(c, Cube::Active);
                }
            }
        }

        self.0 = next;
    }

    fn limits(
        &self,
    ) -> (
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ) {
        let (mut xs, mut ys, mut zs) = (vec![], vec![], vec![]);

        for Coordinate((x, y, z)) in self.0.keys() {
            xs.push(*x);
            ys.push(*y);
            zs.push(*z);
        }

        (
            (*xs.iter().min().unwrap())..=(*xs.iter().max().unwrap()),
            (*ys.iter().min().unwrap())..=(*ys.iter().max().unwrap()),
            (*zs.iter().min().unwrap())..=(*zs.iter().max().unwrap()),
        )
    }
}

impl FromStr for Space {
    type Err = String;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(Space(
            input
                .lines()
                .enumerate()
                .flat_map(move |(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(x, cube)| match cube {
                            '#' => Some((Coordinate((x as isize, y as isize, 0)), Cube::Active)),
                            _ => None,
                        })
                })
                .collect(),
        ))
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let (xs, ys, zs) = self.limits();

        for z in zs {
            writeln!(f, "z={}", z)?;

            for y in ys.clone() {
                writeln!(
                    f,
                    "{}",
                    xs.clone()
                        .map(|x| match self.get(&Coordinate((x, y, z))) {
                            Cube::Active => '#',
                            _ => '.',
                        })
                        .collect::<String>()
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_neigbouhood() {
        let exepected = vec![
            Coordinate((0, 1, 2)),
            Coordinate((0, 1, 3)),
            Coordinate((0, 1, 4)),
            Coordinate((0, 2, 2)),
            Coordinate((0, 2, 3)),
            Coordinate((0, 2, 4)),
            Coordinate((0, 3, 2)),
            Coordinate((0, 3, 3)),
            Coordinate((0, 3, 4)),
            Coordinate((1, 1, 2)),
            Coordinate((1, 1, 3)),
            Coordinate((1, 1, 4)),
            Coordinate((1, 2, 2)),
            Coordinate((1, 2, 4)),
            Coordinate((1, 3, 2)),
            Coordinate((1, 3, 3)),
            Coordinate((1, 3, 4)),
            Coordinate((2, 1, 2)),
            Coordinate((2, 1, 3)),
            Coordinate((2, 1, 4)),
            Coordinate((2, 2, 2)),
            Coordinate((2, 2, 3)),
            Coordinate((2, 2, 4)),
            Coordinate((2, 3, 2)),
            Coordinate((2, 3, 3)),
            Coordinate((2, 3, 4)),
        ];
        let actual = Coordinate((1, 2, 3)).neighbourhood();

        assert_eq!(actual, exepected)
    }

    #[test]
    fn parses_text() {
        let actual = ".#.\n..#\n###".parse::<Space>().unwrap();
        let expected = Space(
            vec![
                (Coordinate((1, 0, 0)), Cube::Active),
                (Coordinate((2, 1, 0)), Cube::Active),
                (Coordinate((0, 2, 0)), Cube::Active),
                (Coordinate((1, 2, 0)), Cube::Active),
                (Coordinate((2, 2, 0)), Cube::Active),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn finds_limits() {
        let actual = ".#.\n..#\n###".parse::<Space>().unwrap().limits();
        let expected = (0..=2, 0..=2, 0..=0);

        assert_eq!(actual, expected);
    }
    #[test]
    fn formats_debug_output() {
        let actual = format!("{:?}", ".#.\n..#\n###".parse::<Space>().unwrap());
        let expected = "z=0\n.#.\n..#\n###\n\n";

        assert_eq!(actual, expected);
    }

    #[test]
    fn makes_step() {
        let mut space = ".#.\n..#\n###".parse::<Space>().unwrap();
        space.step(&step);

        let actual = format!("{:?}", space);
        println!("{:?}", space);

        let expected = "z=-1
#..
..#
.#.

z=0
#.#
.##
.#.

z=1
#..
..#
.#.\n\n";

        assert_eq!(actual, expected);
    }
}
