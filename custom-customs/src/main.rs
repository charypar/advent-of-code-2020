use std::{collections::HashSet, io::BufRead};

fn main() {
    let mut group: HashSet<char> = HashSet::new();
    let mut total = 0;

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        if line == "" {
            total += group.len();
            group.clear();
        }

        for c in line.chars() {
            group.insert(c);
        }
    }

    total += group.len();

    println!("Total questions answered: {}", total);
}
