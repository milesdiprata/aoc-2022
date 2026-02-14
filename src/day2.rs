use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

#[derive(Debug)]
struct StrategyGuide {
    rounds: Vec<(Shape, Shape)>,
}

impl TryFrom<char> for Shape {
    type Error = Error;

    fn try_from(shape: char) -> Result<Self> {
        match shape {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => bail!("invalid shape '{shape}'"),
        }
    }
}

impl FromStr for StrategyGuide {
    type Err = Error;

    fn from_str(guide: &str) -> Result<Self> {
        let mut rounds = Vec::with_capacity(guide.lines().count());

        for round in guide.lines() {
            let (opponent, me) = round
                .split_once(' ')
                .ok_or_else(|| anyhow!("invalid round '{round}'"))?;

            let opponent = (opponent.len() == 1)
                .then_some(opponent.chars().next())
                .flatten()
                .map(Shape::try_from)
                .ok_or_else(|| anyhow!("invalid opponent strategy '{opponent}'"))??;
            let me = (me.len() == 1)
                .then_some(me.chars().next())
                .flatten()
                .map(Shape::try_from)
                .ok_or_else(|| anyhow!("invalid me strategy '{me}'"))??;

            rounds.push((opponent, me));
        }

        Ok(Self { rounds })
    }
}

impl Shape {
    const fn score(self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    const fn play(self, other: Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => Outcome::Loss,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
}

impl Outcome {
    const fn score(self) -> u8 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

fn part1(guide: &StrategyGuide) -> u64 {
    guide
        .rounds
        .iter()
        .map(|&(opponent, me)| me.score() + me.play(opponent).score())
        .map(u64::from)
        .sum()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let guide = StrategyGuide::from_str(&fs::read_to_string("in/day2.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&guide);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 12_156);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
    };

    Ok(())
}
