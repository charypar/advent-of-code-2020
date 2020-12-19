use std::{collections::HashSet, io::BufRead};

fn main() {
    let mut seats: HashSet<usize> = HashSet::with_capacity(128 * 8);

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        seats.insert(seat_id(&line));
    }

    let mut seats = seats.iter().collect::<Vec<_>>();
    seats.sort();

    for pair in seats.windows(2) {
        if let [s1, s2] = pair {
            if *s2 - *s1 > 1 {
                println!("Seat is: {}", *s1 + 1);
                return;
            }
        }
    }
}

fn seat_id(code: &str) -> usize {
    let (row, column) = decode(code);

    row * 8 + column
}

fn decode(code: &str) -> (usize, usize) {
    (
        decode_one(&code[0..7], 'F', 'B'),
        decode_one(&code[7..10], 'L', 'R'),
    )
}

fn decode_one(code: &str, zero: char, one: char) -> usize {
    let binary = code
        .chars()
        .map(|c| match c {
            x if x == zero => "0",
            x if x == one => "1",
            _ => unreachable!(),
        })
        .collect::<String>();

    usize::from_str_radix(binary.as_ref(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_row() {
        assert_eq!(decode_one("FBFBBFF", 'F', 'B'), 44);
    }

    #[test]
    fn decodes_column() {
        assert_eq!(decode_one("RLR", 'L', 'R'), 5);
    }

    #[test]
    fn decodes_seat() {
        assert_eq!(decode("FBFBBFFRLR"), (44, 5));
        assert_eq!(decode("BFFFBBFRRR"), (70, 7));
        assert_eq!(decode("FFFBBBFRRR"), (14, 7));
        assert_eq!(decode("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn gets_seat_id() {
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}
