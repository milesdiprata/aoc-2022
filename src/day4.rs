use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(Debug)]
struct Section {
    pair: (Range, Range),
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(range: &str) -> Result<Self> {
        let (min, max) = range
            .split_once('-')
            .ok_or_else(|| anyhow!("invalid range '{range}'"))?;

        Ok(Self {
            min: min.parse()?,
            max: max.parse()?,
        })
    }
}

impl FromStr for Section {
    type Err = Error;

    fn from_str(section: &str) -> Result<Self> {
        let (first, second) = section
            .split_once(',')
            .ok_or_else(|| anyhow!("invalid section '{section}'"))?;

        Ok(Self {
            pair: (first.parse()?, second.parse()?),
        })
    }
}

impl Range {
    const fn is_fully_contained(&self, other: &Self) -> bool {
        other.min <= self.min && self.max <= other.max
    }

    const fn is_overlapping(&self, other: &Self) -> bool {
        self.min <= other.max && other.min <= self.max
    }
}

impl Section {
    const fn is_any_fully_contained(&self) -> bool {
        let (first, second) = &self.pair;
        first.is_fully_contained(second) || second.is_fully_contained(first)
    }

    const fn is_overlapping(&self) -> bool {
        let (first, second) = &self.pair;
        first.is_overlapping(second)
    }
}

fn part1(sections: &[Section]) -> usize {
    sections
        .iter()
        .filter(|&section| section.is_any_fully_contained())
        .count()
}

fn part2(sections: &[Section]) -> usize {
    sections
        .iter()
        .filter(|&section| section.is_overlapping())
        .count()
}

fn main() -> Result<()> {
    let sections = fs::read_to_string("in/day4.txt")?
        .lines()
        .map(Section::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&sections);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 450);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&sections);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 837);
    };

    Ok(())
}
