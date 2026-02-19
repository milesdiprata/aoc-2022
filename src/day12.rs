use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

type Pos = aoc_2022::Pos<usize>;

#[derive(Debug)]
struct HeightMap {
    grid: Vec<char>,
    width: usize,
    height: usize,
    start: Pos,
    goal: Pos,
}

impl FromStr for HeightMap {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let mut grid = map
            .lines()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = grid
            .first()
            .map(Vec::len)
            .ok_or_else(|| anyhow!("empty height map"))?;
        let height = grid.len();

        let mut start = Pos::new(0, 0);
        let mut goal = Pos::new(0, 0);

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, tile) in row.iter_mut().enumerate() {
                if *tile == 'S' {
                    start = Pos::new(x, y);
                    *tile = 'a';
                } else if *tile == 'E' {
                    goal = Pos::new(x, y);
                    *tile = 'z';
                }
            }
        }

        Ok(Self {
            grid: grid.into_iter().flatten().collect(),
            width,
            height,
            start,
            goal,
        })
    }
}

impl HeightMap {
    fn get(&self, pos: Pos) -> Option<char> {
        if pos.x() < self.width && pos.y() < self.height {
            Some(self.grid[(pos.y() * self.width) + pos.x()])
        } else {
            None
        }
    }

    fn neighbors(
        &self,
        pos: Pos,
        f: impl FnOnce((char, char)) -> bool + Copy,
    ) -> impl Iterator<Item = Pos> {
        pos.adj()
            .filter(move |&adj| self.get(pos).zip(self.get(adj)).is_some_and(f))
    }
}

fn dfs(
    map: &HeightMap,
    start: Pos,
    goal: impl FnOnce(Pos) -> bool + Copy,
    neighbor: impl FnOnce((char, char)) -> bool + Copy,
) -> usize {
    let mut queue = VecDeque::from([start]);
    let mut visited = HashSet::from([start]);
    let mut steps = 0;

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let pos = queue.pop_front().unwrap();

            if goal(pos) {
                return steps;
            }

            for next in map.neighbors(pos, neighbor) {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }

        steps += 1;
    }

    steps
}

fn main() -> Result<()> {
    let map = HeightMap::from_str(&fs::read_to_string("in/day12.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::dfs(
            &map,
            map.start,
            |pos| pos == map.goal,
            |(tile, next)| (next as u8) <= (tile as u8) + 1,
        );
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 497);
    };

    {
        let start = Instant::now();
        let part2 = self::dfs(
            &map,
            map.goal,
            |pos| map.get(pos).is_some_and(|tile| tile == 'a'),
            |(tile, next)| tile as u8 <= next as u8 + 1,
        );
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 492);
    };

    Ok(())
}
