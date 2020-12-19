use std::io::BufRead;

fn main() {
    let mut max_seat = 0;

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        max_seat = std::cmp::max(max_seat, seat_id(&line));
    }

    println!("Max seat: {}", max_seat);
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
