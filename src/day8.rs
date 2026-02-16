use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

type Pos = aoc_2022::Pos<usize>;

struct HeightMap {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for HeightMap {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let grid = map
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10)))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("invalid digit in height map"))?
            .into_iter()
            .map(u8::try_from)
            .collect::<Result<_, _>>()?;

        let width = map
            .lines()
            .next()
            .ok_or_else(|| anyhow!("invalid height map"))?
            .len();
        let height = map.lines().count();

        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

impl HeightMap {
    fn get(&self, pos: Pos) -> Option<u8> {
        if pos.x() < self.width && pos.y() < self.height {
            Some(self.grid[(self.width * pos.y()) + pos.x()])
        } else {
            None
        }
    }

    fn is_visible(&self, pos: Pos) -> bool {
        let Some(height) = self.get(pos) else {
            return false;
        };

        [Pos::up, Pos::right, Pos::down, Pos::left]
            .iter()
            .any(|step| {
                let mut curr = pos;
                while let Some(next) = step(curr) {
                    match self.get(next) {
                        Some(height_next) if height_next >= height => return false,
                        None => break,
                        _ => (),
                    }

                    curr = next;
                }

                true
            })
    }
}

fn part1(heights: &HeightMap) -> usize {
    (0..heights.width)
        .flat_map(|x| (0..heights.height).map(move |y| Pos::new(x, y)))
        .filter(|&pos| heights.is_visible(pos))
        .count()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let heights = HeightMap::from_str(&fs::read_to_string("in/day8.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&heights);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_798);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        // assert_eq!(part2, 259308);
    };

    Ok(())
}
