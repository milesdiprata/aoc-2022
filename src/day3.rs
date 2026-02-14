use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

#[derive(Debug)]
struct Rucksack {
    items: Vec<char>,
}

trait Priority: Copy {
    fn priority(self) -> u8;
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(rucksack: &str) -> Result<Self> {
        if rucksack.chars().any(|item| !item.is_alphabetic()) {
            bail!("invalid rucksack '{rucksack}'");
        }

        Ok(Self {
            items: rucksack.chars().collect(),
        })
    }
}

impl Priority for char {
    fn priority(self) -> u8 {
        if self.is_ascii_uppercase() {
            self as u8 - b'A' + 27
        } else {
            self as u8 - b'a' + 1
        }
    }
}

impl Rucksack {
    fn find_shared_item(&self) -> Option<char> {
        let (c1, c2) = self.items.split_at(self.items.len() / 2);
        let c1 = c1.iter().copied().collect::<HashSet<_>>();

        c2.iter().copied().find(|c| c1.contains(c))
    }
}

fn part1(rucksacks: &[Rucksack]) -> u64 {
    rucksacks
        .iter()
        .filter_map(Rucksack::find_shared_item)
        .map(char::priority)
        .map(u64::from)
        .sum()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let rucksacks = fs::read_to_string("in/day3.txt")?
        .lines()
        .map(Rucksack::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&rucksacks);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 8_515);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 2_434);
    };

    Ok(())
}
