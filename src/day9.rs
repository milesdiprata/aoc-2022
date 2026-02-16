use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

type Pos = aoc_2022::Pos<i64>;

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Motion {
    dir: Dir,
    steps: usize,
}

#[derive(Debug, Default)]
struct Rope {
    head: Pos,
    tail: Pos,
}

impl TryFrom<char> for Dir {
    type Error = Error;

    fn try_from(dir: char) -> Result<Self> {
        match dir {
            'U' => Ok(Self::Up),
            'R' => Ok(Self::Right),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            _ => bail!("invalid direction '{dir}'"),
        }
    }
}

impl FromStr for Motion {
    type Err = Error;

    fn from_str(motion: &str) -> Result<Self> {
        let (dir, steps) = motion
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid motion '{motion}'"))?;

        let dir = dir
            .chars()
            .next()
            .ok_or_else(|| anyhow!("invalid direction '{dir}'"))
            .map(Dir::try_from)??;
        let steps = steps.parse()?;

        Ok(Self { dir, steps })
    }
}

impl Rope {
    fn update_head(mut self, dir: Dir) -> Self {
        let step = match dir {
            Dir::Up => Pos::up,
            Dir::Right => Pos::right,
            Dir::Down => Pos::down,
            Dir::Left => Pos::left,
        };

        self.head = step(self.head);

        let dx = self.head.x() - self.tail.x();
        let dy = self.head.y() - self.tail.y();

        if dx.abs() > 1 || dy.abs() > 1 {
            self.tail = Pos::new(self.tail.x() + dx.signum(), self.tail.y() + dy.signum());
        }

        self
    }
}

fn part1(motions: &[Motion]) -> usize {
    let mut rope = Rope::default();
    let mut visited = HashSet::from([rope.tail]);

    for motion in motions {
        for _ in 0..motion.steps {
            rope = rope.update_head(motion.dir);
            visited.insert(rope.tail);
        }
    }

    visited.len()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let motions = fs::read_to_string("in/day9.txt")?
        .lines()
        .map(Motion::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&motions);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 5_960);
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
