use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;
use aoc_2022::Pos;

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Grove {
    elves: HashSet<Pos<i64>>,
    dir_idx: usize,
}

impl std::fmt::Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.x_min() - 1;
        let x_max = self.x_max() + 1;
        let y_min = self.y_min() - 1;
        let y_max = self.y_max() + 1;

        for y in y_min..=y_max {
            if y > y_min {
                writeln!(f)?;
            }

            for x in x_min..=x_max {
                if self.elves.contains(&Pos::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }

        Ok(())
    }
}

impl FromStr for Grove {
    type Err = Error;

    fn from_str(grid: &str) -> Result<Self> {
        let mut elves = HashSet::new();

        for (y, row) in grid.lines().enumerate() {
            for (x, tile) in row.char_indices() {
                if tile == '#' {
                    let x = i64::try_from(x)?;
                    let y = i64::try_from(y)?;
                    elves.insert(Pos::new(x, y));
                }
            }
        }

        if elves.is_empty() {
            bail!("no elves in the grove!");
        }

        Ok(Self { elves, dir_idx: 0 })
    }
}

impl Dir {
    const LEN: usize = 4;

    fn iter() -> impl Clone + Iterator<Item = Self> {
        [Self::North, Self::South, Self::West, Self::East].into_iter()
    }

    fn adj(self, pos: Pos<i64>) -> impl Iterator<Item = Pos<i64>> {
        match self {
            Self::North => [pos.up(), pos.up().right(), pos.up().left()],
            Self::East => [pos.right(), pos.right().up(), pos.right().down()],
            Self::South => [pos.down(), pos.down().right(), pos.down().left()],
            Self::West => [pos.left(), pos.left().up(), pos.left().down()],
        }
        .into_iter()
    }

    const fn move_pos(self, pos: Pos<i64>) -> Pos<i64> {
        match self {
            Self::North => pos.up(),
            Self::East => pos.right(),
            Self::South => pos.down(),
            Self::West => pos.left(),
        }
    }
}

impl Grove {
    fn x_min(&self) -> i64 {
        self.elves
            .iter()
            .copied()
            .map(Pos::x)
            .min()
            .unwrap_or_default()
    }

    fn x_max(&self) -> i64 {
        self.elves
            .iter()
            .copied()
            .map(Pos::x)
            .max()
            .unwrap_or_default()
    }

    fn y_min(&self) -> i64 {
        self.elves
            .iter()
            .copied()
            .map(Pos::y)
            .min()
            .unwrap_or_default()
    }

    fn y_max(&self) -> i64 {
        self.elves
            .iter()
            .copied()
            .map(Pos::y)
            .max()
            .unwrap_or_default()
    }

    fn dirs(&self) -> impl Iterator<Item = Dir> {
        Dir::iter().cycle().skip(self.dir_idx).take(Dir::LEN)
    }

    fn simulate_round(&mut self) -> bool {
        let mut proposals = Vec::with_capacity(self.elves.len());
        let mut counts = HashMap::with_capacity(self.elves.len());
        let mut moved = false;

        for &elf in &self.elves {
            if !self
                .dirs()
                .any(|dir| dir.adj(elf).any(|adj| self.elves.contains(&adj)))
            {
                // No neighbors
                continue;
            }

            if let Some(to) = self
                .dirs()
                .find(|&dir| dir.adj(elf).all(|adj| !self.elves.contains(&adj)))
                .map(|dir| dir.move_pos(elf))
            {
                proposals.push((elf, to));
                *counts.entry(to).or_insert(0_usize) += 1;
            }
        }

        for (from, to) in proposals {
            if counts.get(&to).is_some_and(|&count| count == 1) {
                self.elves.remove(&from);
                self.elves.insert(to);
                moved = true;
            }
        }

        self.dir_idx = (self.dir_idx + 1) % Dir::LEN;

        moved
    }
}

fn part1(grove: &mut Grove) -> u64 {
    const ROUNDS: usize = 10;

    for _ in 0..ROUNDS {
        grove.simulate_round();
    }

    let area = (grove.x_max() - grove.x_min() + 1) * (grove.y_max() - grove.y_min() + 1);
    area.cast_unsigned() - grove.elves.len() as u64
}

fn part2(grove: &mut Grove) -> usize {
    let mut round = 1;

    while grove.simulate_round() {
        round += 1;
    }

    round
}

fn main() -> Result<()> {
    let mut grove = Grove::from_str(&fs::read_to_string("in/day23.txt")?)?;

    {
        let mut grove = grove.clone();
        let start = Instant::now();
        let part1 = self::part1(&mut grove);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 3_925);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&mut grove);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 903);
    };

    Ok(())
}
