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

    for i in 0..program.len() {
        let new_op = match program[i] {
            Op::Nop(n) => Op::Jmp(n),
            Op::Jmp(n) => Op::Nop(n),
            Op::Acc(_) => continue,
        };

        let mut modified_program = program.clone();
        modified_program[i] = new_op;

        let (pc, acc) = run(&modified_program);

        if pc == program.len() {
            println!("Program {} ends with pc = {}, acc = {}", i, pc, acc);
            break;
        }
    }
}

fn run(program: &[Op]) -> (usize, i64) {
    let mut visited = vec![false; program.len()];

    let mut acc: i64 = 0;
    let mut pc: usize = 0;

    while pc < program.len() && !visited[pc] {
        visited[pc] = true;

        match program[pc] {
            Op::Nop(_) => pc += 1,
            Op::Jmp(n) => pc = (pc as i64 + n) as usize,
            Op::Acc(n) => {
                acc += n;
                pc += 1;
            }
        }
    }

    (pc, acc)
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn parse_line(line: &str) -> Op {
    let parts: Vec<_> = line.split(" ").collect();

    match parts[0..2] {
        ["acc", num] => Op::Acc(num.parse().unwrap()),
        ["jmp", num] => Op::Jmp(num.parse().unwrap()),
        ["nop", num] => Op::Nop(num.parse().unwrap()),
        _ => unreachable!(),
    }
}
