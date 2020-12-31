use std::io::BufRead;

fn main() {
    let inputs = std::io::stdin()
        .lock()
        .lines()
        .map(|it| it.unwrap())
        .collect::<Vec<_>>();

    let when = inputs[0].parse::<usize>().unwrap();
    let raw_lines: Vec<Option<usize>> = inputs[1]
        .split(',')
        .map(|bus| bus.parse::<usize>().ok())
        .collect();

    // part 1

    let lines: Vec<usize> = raw_lines.iter().filter_map(|it| *it).collect();

    let mut from_now: Vec<(usize, usize)> = lines
        .iter()
        .map(|line| (*line, line - (when % line)))
        .collect();

    from_now.sort_by_key(|(_, when)| *when);

    println!("Start at {}, lines: {:?}, in: {:?}", when, lines, from_now);

    // part 2

    let mut delays: Vec<(usize, usize)> = raw_lines
        .iter()
        .enumerate()
        .filter_map(|it| match it {
            (a, &Some(b)) => Some((b, a)),
            (_, &None) => None,
        })
        .collect();

    delays.sort_by(|(line_a, _), (line_b, _)| line_b.cmp(line_a));

    println!("Finding timestamp matching {:?}", delays);

    let time = find_time(&delays);

    println!("\nFound the time! {}", time);
}

fn find_time(delays: &[(usize, usize)]) -> u128 {
    let mut time = delays[0].0 as u128 - delays[0].1 as u128;
    let mut step = delays[0].0 as u128;

    let limit: u128 = delays.iter().map(|(line, _)| *line as u128).product();

    loop {
        let correct_lines = correct_lines(&delays, time);
        if correct_lines.len() == delays.len() || time > limit {
            return time;
        }

        let new_step = correct_lines.iter().product();

        if new_step > step {
            println!(
                "Found time {} that makes {} buses ({:?}) line up (new step {})",
                time,
                correct_lines.len(),
                correct_lines,
                new_step
            );
            step = new_step;
        }

        time += step;
    }
}

fn correct_lines(delays: &[(usize, usize)], now: u128) -> Vec<u128> {
    let mut lines = Vec::new();

    for (line, from_now) in delays.iter() {
        let wanted_at = now + *from_now as u128;

        if wanted_at % *line as u128 != 0 {
            return lines;
        } else {
            lines.push(*line as u128);
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_timestamp_1() {
        // 17,x,13,19
        let delays = [(19, 3), (17, 0), (13, 2)];

        assert_eq!(find_time(&delays), 3417);
    }

    #[test]
    fn finds_timestamp_2() {
        // 67,7,59,61
        let delays = [(67, 0), (61, 3), (59, 2), (7, 1)];

        assert_eq!(find_time(&delays), 754018);
    }

    #[test]
    fn finds_timestamp_3() {
        // 67,x,7,59,61
        let delays = [(67, 0), (61, 4), (59, 3), (7, 2)];

        assert_eq!(find_time(&delays), 779210);
    }

    #[test]
    fn finds_timestamp_4() {
        // 67,7,x,59,61
        let delays = [(67, 0), (59, 3), (61, 4), (7, 1)];

        assert_eq!(find_time(&delays), 1261476);
    }

    #[test]
    fn finds_timestamp_5() {
        // 1789,37,47,1889
        let delays = [(1889, 3), (1789, 0), (47, 2), (37, 1)];

        assert_eq!(find_time(&delays), 1202161486);
    }
}
