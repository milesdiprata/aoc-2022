use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(instruction: &str) -> Result<Self> {
        let parts = instruction.split_ascii_whitespace().collect::<Vec<_>>();

        if parts.len() != 6 {
            bail!("invalid instruction '{instruction}'");
        }

        Ok(Self {
            quantity: parts[1].parse()?,
            from: parts[3].parse::<usize>()? - 1,
            to: parts[5].parse::<usize>()? - 1,
        })
    }
}

fn parse() -> Result<(Vec<Vec<char>>, Vec<Instruction>)> {
    let input = fs::read_to_string("in/day5.txt")?;
    let (crates, instructions) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input"))?;

    let mut crates = crates.lines().rev();
    let len = crates
        .next()
        .ok_or_else(|| anyhow!("missing stacks in input"))?
        .split_ascii_whitespace()
        .count();

    let mut stacks = vec![vec![]; len];
    for line in crates {
        for (i, chunk) in line.as_bytes().chunks(4).enumerate() {
            if chunk[0] == b'[' {
                stacks[i].push(chunk[1] as char);
            }
        }
    }

    let instructions = instructions
        .lines()
        .map(str::parse)
        .collect::<Result<_>>()?;

    Ok((stacks, instructions))
}

fn part1(stacks: &mut [Vec<char>], instructions: &[Instruction]) -> String {
    for &Instruction { quantity, from, to } in instructions {
        for _ in 0..quantity {
            if let Some(c) = stacks[from].pop() {
                stacks[to].push(c);
            }
        }
    }

    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .copied()
        .collect()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let (mut stacks, instructions) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&mut stacks, &instructions);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, "QNNTGTPFN");
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
