use std::io::BufRead;
#[derive(Debug, PartialEq, Copy, Clone)]
enum Square {
    Floor,
    Seat(bool),
}
fn main() {
    let mut map = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Square::Floor,
                    'L' => Square::Seat(false),
                    '#' => Square::Seat(true),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut steps: usize = 0;

    loop {
        let new_map = step(&map);
        steps += 1;

        if new_map == map {
            break;
        }

        map = new_map;
    }

    let count = map
        .iter()
        .map(|row| row.iter().filter(|sq| **sq == Square::Seat(true)).count())
        .sum::<usize>();

    println!("Done in step {}, occupied seats: {}", steps, count);
}

fn step(map: &Vec<Vec<Square>>) -> Vec<Vec<Square>> {
    (0..map.len())
        .map(|i| {
            (0..map[0].len())
                .map(|j| {
                    let ns = neighbours(&map, (i, j));
                    let occs = ns.into_iter().filter(|s| *s == Square::Seat(true)).count();

                    if map[i][j] == Square::Seat(true) && occs > 3 {
                        return Square::Seat(false);
                    }

                    if map[i][j] == Square::Seat(false) && occs == 0 {
                        return Square::Seat(true);
                    }

                    map[i][j]
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn neighbours(map: &Vec<Vec<Square>>, coordinates: (usize, usize)) -> Vec<Square> {
    let y: i64 = coordinates.0 as i64;
    let x: i64 = coordinates.1 as i64;

    ((y - 1)..=(y + 1))
        .flat_map(|i| {
            ((x - 1)..=(x + 1)).filter_map(move |j| match (i, j) {
                (a, b) if a == y && b == x => None,
                (a, b) if a < 0 || b < 0 || a >= map.len() as i64 || b >= map[0].len() as i64 => {
                    Some(Square::Floor)
                }
                (a, b) => Some(map[a as usize][b as usize]),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        Square::{Floor as F, Seat as S},
        *,
    };

    #[test]
    fn gets_neighbours() {
        let map = vec![
            vec![S(true), F, S(false), F],
            vec![F, S(false), F, S(true)],
            vec![F, F, F, F],
            vec![S(true), S(false), S(true), S(false)],
        ];

        assert_eq!(
            neighbours(&map, (0, 0)),
            vec![F, F, F, F, F, F, F, S(false)]
        );

        assert_eq!(
            neighbours(&map, (2, 2)),
            vec![S(false), F, S(true), F, F, S(false), S(true), S(false)]
        );

        assert_eq!(
            neighbours(&map, (1, 2)),
            vec![F, S(false), F, S(false), S(true), F, F, F]
        )
    }

    #[test]
    fn makes_a_step() {
        let map = vec![
            vec![S(false), F, S(false), F],
            vec![F, S(false), F, S(true)],
            vec![F, F, S(true), S(true)],
            vec![S(true), S(false), S(true), S(true)],
        ];

        let new_map = step(&map);

        assert_eq!(new_map[0][0], S(true));
        assert_eq!(new_map[2][3], S(false));
        assert_eq!(new_map[1][0], F);
        assert_eq!(new_map[0][2], S(false));
        assert_eq!(new_map[1][3], S(true));
    }
}
