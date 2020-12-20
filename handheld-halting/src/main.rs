use std::io::BufRead;

fn main() {
    let program = std::io::stdin()
        .lock()
        .lines()
        .map(|line| match line {
            Ok(line) => parse_line(&line),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let mut visited = vec![false; program.len()];

    let mut acc: i64 = 0;
    let mut pc: usize = 0;

    while !visited[pc] {
        visited[pc] = true;

        match program[pc] {
            Op::Nop => pc += 1,
            Op::Jmp(n) => pc = (pc as i64 + n) as usize,
            Op::Acc(n) => {
                acc += n;
                pc += 1;
            }
        }
    }

    println!("Stopped at pc: {}, acc: {} ", pc, acc);
}

#[derive(Debug)]
enum Op {
    Acc(i64),
    Jmp(i64),
    Nop,
}

fn parse_line(line: &str) -> Op {
    let parts: Vec<_> = line.split(" ").collect();

    match parts[0..2] {
        ["acc", num] => Op::Acc(num.parse().unwrap()),
        ["jmp", num] => Op::Jmp(num.parse().unwrap()),
        ["nop", _] => Op::Nop,
        _ => unreachable!(),
    }
}
