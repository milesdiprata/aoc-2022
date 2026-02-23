use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cube {
    x: i8,
    y: i8,
    z: i8,
}

impl FromStr for Cube {
    type Err = Error;

    fn from_str(cube: &str) -> Result<Self> {
        let err = || anyhow!("invalid cube '{cube}'");
        let mut coords = cube.split(',').map(str::parse);

        Ok(Self {
            x: coords.next().ok_or_else(err)??,
            y: coords.next().ok_or_else(err)??,
            z: coords.next().ok_or_else(err)??,
        })
    }
}

impl Cube {
    fn neighbors(self) -> impl Iterator<Item = Self> {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .map(|(dx, dy, dz)| Self {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        })
        .into_iter()
    }
}

fn part1(cubes: &[Cube]) -> usize {
    let mut surface_area = 6 * cubes.len();

    let cubes = cubes.iter().copied().collect::<HashSet<_>>();
    for &cube in &cubes {
        surface_area -= cube
            .neighbors()
            .filter(|neighbor| cubes.contains(neighbor))
            .count();
    }

    surface_area
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let cubes = fs::read_to_string("in/day18.txt")?
        .lines()
        .map(Cube::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&cubes);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 4_474);
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
