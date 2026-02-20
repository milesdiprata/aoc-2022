use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

use aoc_2022::Pos;

#[derive(Debug)]
struct Path {
    points: Vec<Pos<i32>>,
}

#[derive(Debug)]
struct Cave {
    rocks: HashSet<Pos<i32>>,
    sand: HashSet<Pos<i32>>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let y_min = self.y_min.min(Self::SOURCE.y());

        for y in y_min..=self.y_max {
            if y > y_min {
                writeln!(f)?;
            }

            for x in self.x_min..=self.x_max {
                let pos = Pos::new(x, y);
                if pos == Self::SOURCE {
                    write!(f, "+")?;
                } else if self.rocks.contains(&pos) {
                    write!(f, "#")?;
                } else if self.sand.contains(&pos) {
                    write!(f, "o");
                } else {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self> {
        let path = path.split(" -> ");

        let mut points = Vec::with_capacity(path.clone().count());
        for point in path {
            let (x, y) = point
                .split_once(',')
                .ok_or_else(|| anyhow!("invalid point '{point}'"))?;

            points.push(Pos::new(x.parse()?, y.parse()?));
        }

        for window in points.windows(2) {
            let (start, end) = (window[0], window[1]);
            if start.x() != end.x() && start.y() != end.y() {
                bail!("path contains diagonal line");
            }
        }

        Ok(Self { points })
    }
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(paths: &str) -> Result<Self> {
        let paths = paths
            .lines()
            .map(Path::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self::from(paths.as_slice()))
    }
}

impl From<&[Path]> for Cave {
    fn from(paths: &[Path]) -> Self {
        let rocks = paths.iter().flat_map(Path::extend).collect::<HashSet<_>>();
        let x_min = rocks.iter().copied().map(Pos::x).min().unwrap_or_default();
        let x_max = rocks.iter().copied().map(Pos::x).max().unwrap_or_default();
        let y_min = rocks.iter().copied().map(Pos::y).min().unwrap_or_default();
        let y_max = rocks.iter().copied().map(Pos::y).max().unwrap_or_default();

        Self {
            rocks,
            sand: HashSet::new(),
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

impl Path {
    fn extend(&self) -> impl Iterator<Item = Pos<i32>> + '_ {
        self.points.windows(2).flat_map(|window| {
            let (start, end) = (window[0], window[1]);
            if start.x() == end.x() {
                let (y_min, y_max) = (start.y().min(end.y()), start.y().max(end.y()));
                Box::new((y_min..=y_max).map(move |y| Pos::new(start.x(), y)))
                    as Box<dyn Iterator<Item = _>>
            } else {
                let (x_min, x_max) = (start.x().min(end.x()), start.x().max(end.x()));
                Box::new((x_min..=x_max).map(move |x| Pos::new(x, start.y())))
            }
        })
    }
}

impl Cave {
    const SOURCE: Pos<i32> = Pos::new(500, 0);

    fn is_air(&self, pos: Pos<i32>) -> bool {
        !self.rocks.contains(&pos) && !self.sand.contains(&pos)
    }

    fn is_abyss(&self, pos: Pos<i32>) -> bool {
        self.is_air(pos) && pos.y() > self.y_max
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand = Self::SOURCE;

        while !self.is_abyss(sand) {
            if self.is_air(sand.down()) {
                sand = sand.down();
            } else if self.is_air(sand.down().left()) {
                sand = sand.down().left();
            } else if self.is_air(sand.down().right()) {
                sand = sand.down().right();
            } else {
                self.sand.insert(sand);
                return true;
            }
        }

        false
    }
}

fn part1(cave: &mut Cave) -> usize {
    while cave.drop_sand() {}
    cave.sand.len()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let mut cave = Cave::from_str(&fs::read_to_string("in/day14.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&mut cave);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 832);
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
