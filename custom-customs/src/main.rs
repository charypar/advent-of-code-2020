use std::{collections::HashMap, io::BufRead};

fn main() {
    let mut group: HashMap<char, usize> = HashMap::new();
    let mut group_length = 0;
    let mut total = 0;

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        if line == "" {
            total += group.iter().filter(|(_, v)| **v == group_length).count();
            group_length = 0;
            group.clear();
            continue;
        }

        group_length += 1;
        for c in line.chars() {
            group.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }
    }

    total += group.iter().filter(|(_, v)| **v == group_length).count();

    println!(
        "Total questions answered by everyone in the group: {}",
        total
    );
}
