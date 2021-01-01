use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask {
        overwrite: 0,
        floating: vec![],
    };

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        let op = line.parse().unwrap();

        match op {
            Instruction::Mask(m) => {
                mask = m;
            }
            Instruction::Mem { address, value } => {
                for addr in mask.apply(address) {
                    memory.insert(addr, value);
                }
            }
        }
    }

    println!("Sum: {}", memory.values().sum::<u64>())
}

#[derive(Debug, PartialEq)]
struct Mask {
    overwrite: u64,
    floating: Vec<usize>,
}

impl Mask {
    fn apply(&self, input: u64) -> Vec<u64> {
        let base = input | self.overwrite;

        self.floating.iter().fold(vec![base], |acc, i| {
            acc.into_iter()
                .flat_map(|address| {
                    let mask = 1 << i;

                    vec![address | mask, address & !mask]
                })
                .collect()
        })
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mask(Mask),
    Mem { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match &line[0..4] {
            "mask" => {
                let overwrite = u64::from_str_radix(
                    line[7..]
                        .chars()
                        .map(|c| match c {
                            '1' => '1',
                            _ => '0',
                        })
                        .collect::<String>()
                        .as_ref(),
                    2,
                )
                .unwrap();
                let floating = line[7..]
                    .chars()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, c)| match c {
                        'X' => Some(i),
                        _ => None,
                    })
                    .collect();

                Ok(Instruction::Mask(Mask {
                    overwrite,
                    floating,
                }))
            }
            "mem[" => {
                let address = line[4..]
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse()
                    .map_err(|_| "cannot parse mem address")?;
                let value = line
                    .split("= ")
                    .nth(1)
                    .unwrap()
                    .parse()
                    .map_err(|_| "cannot parse mem value")?;

                Ok(Instruction::Mem { address, value })
            }
            word => Err(format!("Unknown instruction {}", word)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_mask_instruction() {
        let expected = Ok(Instruction::Mask(Mask {
            overwrite: 0b10010,
            floating: vec![0, 5],
        }));
        let actual = "mask = 000000000000000000000000000000X1001X".parse::<Instruction>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn reads_a_mem_instruction() {
        let expected = Ok(Instruction::Mem {
            address: 8,
            value: 11,
        });
        let actual = "mem[8] = 11".parse::<Instruction>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn applies_small_mask() {
        let mask = Mask {
            overwrite: 0b10010,
            floating: vec![0, 5],
        };

        let mut actual = mask.apply(0b101010);
        actual.sort();

        assert_eq!(actual, vec![26, 27, 58, 59]);
    }

    #[test]
    fn applies_bigger_mask() {
        let mask = Mask {
            overwrite: 0,
            floating: vec![0, 1, 3],
        };

        let mut actual = mask.apply(0b11010);
        actual.sort();

        assert_eq!(actual, vec![16, 17, 18, 19, 24, 25, 26, 27]);
    }
}
