use std::collections::BinaryHeap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct Elf {
    calories: Vec<u32>,
}

impl FromStr for Elf {
    type Err = Error;

    fn from_str(calories: &str) -> Result<Self> {
        Ok(Self {
            calories: calories
                .lines()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Elf {
    fn total(&self) -> u64 {
        self.calories.iter().copied().map(u64::from).sum()
    }
}

fn part1(elves: &[Elf]) -> u64 {
    elves.iter().map(Elf::total).max().unwrap_or_default()
}

fn part2(elves: &[Elf]) -> u64 {
    let mut calories = elves.iter().map(Elf::total).collect::<BinaryHeap<_>>();

    (0..3)
        .map(|_| calories.pop())
        .map(Option::unwrap_or_default)
        .sum()
}

fn main() -> Result<()> {
    let elves = fs::read_to_string("in/day1.txt")?
        .split("\n\n")
        .map(Elf::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&elves);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 72_070);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&elves);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 211_805);
    };

    Ok(())
}
