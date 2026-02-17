use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct Cpu {
    x: i32,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(instruction: &str) -> Result<Self> {
        if instruction == "noop" {
            Ok(Self::Noop)
        } else if let Some(val) = instruction.strip_prefix("addx ") {
            let val = val.parse()?;
            Ok(Self::Addx(val))
        } else {
            bail!("unknown instruction '{instruction}'")
        }
    }
}

impl Cpu {
    const fn new() -> Self {
        Self { x: 1 }
    }

    fn execute(&mut self, program: &[Instruction]) -> Vec<i32> {
        let mut snapshots = Vec::new();

        for &instruction in program {
            match instruction {
                Instruction::Noop => snapshots.push(self.x),
                Instruction::Addx(val) => {
                    snapshots.extend([self.x, self.x]);
                    self.x += val;
                }
            }
        }

        snapshots
    }
}

fn part1(program: &[Instruction]) -> i32 {
    let mut cpu = Cpu::new();
    let snapshots = cpu.execute(program);

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&c| c as i32 * snapshots[c - 1])
        .sum()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day10.txt")?
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&program);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 12_520);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 0);
    };

    Ok(())
}
