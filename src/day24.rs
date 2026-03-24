use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

use anyhow::anyhow;
use anyhow::bail;
use aoc_2022::Pos;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Ground,
    Blizzard(Dir),
    Blizzards(usize),
}

#[derive(Debug)]
struct Valley {
    walls: Vec<bool>,
    blizzards: Vec<(Pos<usize>, Dir)>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Ground => write!(f, "."),
            &Self::Blizzard(dir) => write!(f, "{dir}"),
            &Self::Blizzards(count) => write!(f, "{count}"),
        }
    }
}

impl std::fmt::Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn get(valley: &Valley, pos: Pos<usize>) -> Option<Tile> {
            let mut blizzards = valley
                .blizzards
                .iter()
                .filter(|(blizzard, _)| *blizzard == pos);

            if let Some(&(_, dir)) = blizzards.next() {
                Some(match blizzards.count() {
                    0 => Tile::Blizzard(dir),
                    n => Tile::Blizzards(n + 1),
                })
            } else if pos.x() < valley.width && pos.y() < valley.height {
                Some(if valley.walls[(valley.width * pos.y()) + pos.x()] {
                    Tile::Wall
                } else {
                    Tile::Ground
                })
            } else {
                None
            }
        }

        for y in 0..self.height {
            if y > 0 {
                writeln!(f)?;
            }

            for x in 0..self.width {
                write!(f, "{}", get(self, Pos::new(x, y)).ok_or(std::fmt::Error)?)?;
            }
        }

        Ok(())
    }
}

impl TryFrom<char> for Dir {
    type Error = Error;

    fn try_from(dir: char) -> Result<Self> {
        match dir {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => bail!("invalid dir '{dir}'"),
        }
    }
}

impl FromStr for Valley {
    type Err = Error;

    fn from_str(valley: &str) -> Result<Self> {
        let height = valley.lines().count();
        let width = valley
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty valley"))?
            .len();

        let walls = valley
            .lines()
            .flat_map(|row| row.chars())
            .map(|c| c == '#')
            .collect();

        let blizzards = valley
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.char_indices().filter_map(move |(x, c)| {
                    Dir::try_from(c).ok().map(|dir| (Pos::new(x, y), dir))
                })
            })
            .collect();

        Ok(Self {
            walls,
            blizzards,
            width,
            height,
        })
    }
}

impl Valley {
    const fn start(&self) -> Pos<usize> {
        Pos::new(1, 0)
    }

    const fn goal(&self) -> Pos<usize> {
        Pos::new(self.width - 2, self.height - 1)
    }

    fn is_open(&self, pos: Pos<usize>) -> bool {
        pos.x() < self.width
            && pos.y() < self.height
            && !self.walls[(self.width * pos.y()) + pos.x()]
    }

    fn update_blizzards(&mut self) {
        for (pos, dir) in &mut self.blizzards {
            *pos = match dir {
                Dir::Up if pos.y() == 1 => Pos::new(pos.x(), self.height - 2),
                Dir::Down if pos.y() == self.height - 2 => Pos::new(pos.x(), 1),
                Dir::Left if pos.x() == 1 => Pos::new(self.width - 2, pos.y()),
                Dir::Right if pos.x() == self.width - 2 => Pos::new(1, pos.y()),
                Dir::Up => Pos::new(pos.x(), pos.y() - 1),
                Dir::Down => Pos::new(pos.x(), pos.y() + 1),
                Dir::Left => Pos::new(pos.x() - 1, pos.y()),
                Dir::Right => Pos::new(pos.x() + 1, pos.y()),
            };
        }
    }

    fn blizzard_snapshots(&mut self) -> Vec<HashSet<Pos<usize>>> {
        const fn lcm(a: usize, b: usize) -> usize {
            const fn gcd(a: usize, b: usize) -> usize {
                if b == 0 { a } else { gcd(b, a % b) }
            }

            a / gcd(a, b) * b
        }

        let period = lcm(self.width - 2, self.height - 2);
        let mut snapshots = Vec::with_capacity(period);

        for _ in 0..period {
            snapshots.push(self.blizzards.iter().map(|(p, _)| *p).collect());
            self.update_blizzards();
        }

        snapshots
    }

    fn bfs(
        &self,
        snapshots: &[HashSet<Pos<usize>>],
        start: Pos<usize>,
        goal: Pos<usize>,
        start_time: usize,
    ) -> usize {
        let period = snapshots.len();

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((start, start_time));
        visited.insert((start, start_time % period));

        while let Some((pos, time)) = queue.pop_front() {
            let next_time = time + 1;
            let snap = &snapshots[next_time % period];

            for next_pos in [Some(pos), pos.up(), pos.down(), pos.left(), pos.right()]
                .into_iter()
                .flatten()
            {
                if next_pos == goal {
                    return next_time;
                }

                if !self.is_open(next_pos) || snap.contains(&next_pos) {
                    continue;
                }

                if visited.insert((next_pos, next_time % period)) {
                    queue.push_back((next_pos, next_time));
                }
            }
        }

        unreachable!()
    }
}

fn part1(valley: &Valley, snapshots: &[HashSet<Pos<usize>>]) -> usize {
    valley.bfs(snapshots, valley.start(), valley.goal(), 0)
}

fn part2(valley: &Valley, snapshots: &[HashSet<Pos<usize>>]) -> usize {
    let t1 = valley.bfs(snapshots, valley.start(), valley.goal(), 0);
    let t2 = valley.bfs(snapshots, valley.goal(), valley.start(), t1);
    valley.bfs(snapshots, valley.start(), valley.goal(), t2)
}

fn main() -> Result<()> {
    let mut valley = Valley::from_str(&fs::read_to_string("in/day24.txt")?)?;
    let snapshots = valley.blizzard_snapshots();

    {
        let start = Instant::now();
        let part1 = self::part1(&valley, &snapshots);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 245);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&valley, &snapshots);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 798);
    };

    Ok(())
}
