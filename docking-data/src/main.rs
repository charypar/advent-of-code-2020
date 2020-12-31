use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask {
        overwrite: 0,
        default: 0,
    };

    while let Some(Ok(line)) = std::io::stdin().lock().lines().next() {
        let op = line.parse().unwrap();

        match op {
            Instruction::Mask(m) => {
                mask = m;
            }
            Instruction::Mem { address, value } => {
                memory.insert(address, mask.apply(value));
            }
        }
    }

    println!("Sum: {}", memory.values().sum::<u64>())
}

#[derive(Debug, PartialEq)]
struct Mask {
    overwrite: u64,
    default: u64,
}

impl Mask {
    fn apply(&self, input: u64) -> u64 {
        (self.overwrite & self.default) | (!self.overwrite & input)
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
                            'X' => '0',
                            _ => '1',
                        })
                        .collect::<String>()
                        .as_ref(),
                    2,
                )
                .unwrap();
                let default = u64::from_str_radix(
                    line[7..]
                        .chars()
                        .map(|c| match c {
                            'X' => '0',
                            x => x,
                        })
                        .collect::<String>()
                        .as_ref(),
                    2,
                )
                .unwrap();

                Ok(Instruction::Mask(Mask { overwrite, default }))
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
            overwrite: 0b1000010,
            default: 0b1000000,
        }));
        let actual = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse::<Instruction>();

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
    fn applies_mask() {
        let mask = Mask {
            overwrite: 0b1000010,
            default: 0b1000000,
        };

        assert_eq!(mask.apply(0b1011), 0b1001001);
    }
}
