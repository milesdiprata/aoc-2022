use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Clone, Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

trait CrateMover {
    fn move_crates(stacks: &mut Vec<Vec<char>>, instruction: &Instruction);
}

struct CrateMover9000;
struct CrateMover9001;

#[derive(Clone, Debug)]
struct Puzzle {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
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

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
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

        Ok(Self {
            stacks,
            instructions,
        })
    }
}

impl CrateMover for CrateMover9000 {
    fn move_crates(stacks: &mut Vec<Vec<char>>, instruction: &Instruction) {
        for _ in 0..instruction.quantity {
            if let Some(c) = stacks[instruction.from].pop() {
                stacks[instruction.to].push(c);
            }
        }
    }
}

impl CrateMover for CrateMover9001 {
    fn move_crates(stacks: &mut Vec<Vec<char>>, instruction: &Instruction) {
        let from = &mut stacks[instruction.from];
        let crates = from.split_off(from.len() - instruction.quantity);
        stacks[instruction.to].extend(crates);
    }
}

impl Puzzle {
    fn execute<M: CrateMover>(mut self) -> String {
        for instruction in &self.instructions {
            M::move_crates(&mut self.stacks, instruction);
        }

        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect()
    }
}

fn main() -> Result<()> {
    let puzzle = Puzzle::from_str(&fs::read_to_string("in/day5.txt")?)?;

    {
        let start = Instant::now();
        let part1 = puzzle.clone().execute::<CrateMover9000>();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, "QNNTGTPFN");
    };

    {
        let start = Instant::now();
        let part2 = puzzle.execute::<CrateMover9001>();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, "GGNPJBTTR");
    };

    Ok(())
}
