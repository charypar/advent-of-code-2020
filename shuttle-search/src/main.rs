use std::io::BufRead;

fn main() {
    let inputs = std::io::stdin()
        .lock()
        .lines()
        .map(|it| it.unwrap())
        .collect::<Vec<_>>();

    let when = inputs[0].parse::<usize>().unwrap();
    let lines: Vec<usize> = inputs[1]
        .split(',')
        .filter_map(|bus| bus.parse::<usize>().ok())
        .collect();

    let mut from_now: Vec<(usize, usize)> = lines
        .iter()
        .map(|line| (*line, line - (when % line)))
        .collect();

    from_now.sort_by_key(|(_, when)| *when);

    println!("Start at {}, lines: {:?}, in: {:?}", when, lines, from_now);
}
