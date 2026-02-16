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

#[derive(Debug)]
struct Rope<const N: usize> {
    knots: [Pos; N],
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

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            knots: [Pos::default(); N],
        }
    }

    const fn tail(&self) -> Pos {
        self.knots[N - 1]
    }

    fn update_head(mut self, dir: Dir) -> Self {
        let step = match dir {
            Dir::Up => Pos::up,
            Dir::Right => Pos::right,
            Dir::Down => Pos::down,
            Dir::Left => Pos::left,
        };

        self.knots[0] = step(self.knots[0]);

        for i in 1..N {
            let (head, tail) = (self.knots[i - 1], self.knots[i]);
            let dx = head.x() - tail.x();
            let dy = head.y() - tail.y();

            if dx.abs() > 1 || dy.abs() > 1 {
                self.knots[i] = Pos::new(tail.x() + dx.signum(), tail.y() + dy.signum());
            }
        }

        self
    }
}

fn solve<const N: usize>(motions: &[Motion]) -> usize {
    let mut rope = Rope::<N>::new();
    let mut visited = HashSet::from([rope.tail()]);

    for motion in motions {
        for _ in 0..motion.steps {
            rope = rope.update_head(motion.dir);
            visited.insert(rope.tail());
        }
    }

    visited.len()
}

fn main() -> Result<()> {
    let motions = fs::read_to_string("in/day9.txt")?
        .lines()
        .map(Motion::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::solve::<2>(&motions);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 5_960);
    };

    {
        let start = Instant::now();
        let part2 = self::solve::<10>(&motions);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 2_327);
    };

    Ok(())
}
