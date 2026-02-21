use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

use anyhow::anyhow;
use aoc_2022::Pos;

#[derive(Debug)]
struct Sensor {
    pos: Pos<i32>,
    beacon: Pos<i32>,
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(sensor: &str) -> Result<Self> {
        let mut nums = sensor
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .flat_map(str::parse);

        let pos = Pos::new(
            nums.next()
                .ok_or_else(|| anyhow!("missing sensor x-position"))?,
            nums.next()
                .ok_or_else(|| anyhow!("missing sensor y-position"))?,
        );
        let beacon = Pos::new(
            nums.next()
                .ok_or_else(|| anyhow!("missing beacon x-position"))?,
            nums.next()
                .ok_or_else(|| anyhow!("missing beacon y-position"))?,
        );

        Ok(Self { pos, beacon })
    }
}

impl Sensor {
    const fn dist_beacon(&self) -> i32 {
        (self.pos.x() - self.beacon.x()).abs() + (self.pos.y() - self.beacon.y()).abs()
    }

    const fn coverage_at_y(&self, y: i32) -> Option<(i32, i32)> {
        let dist = self.dist_beacon();
        let dy = (self.pos.y() - y).abs();
        let remaining = dist - dy;

        if remaining < 0 {
            return None;
        }

        Some((self.pos.x() - remaining, self.pos.x() + remaining))
    }
}

fn merge(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut intervals = {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|&(start, _)| start);
        intervals.into_iter()
    };

    let mut merged = if let Some((start, end)) = intervals.next() {
        vec![(start, end)]
    } else {
        return vec![];
    };

    for (start, end) in intervals {
        let (_, end_prev) = merged.last_mut().unwrap();

        if start <= *end_prev {
            *end_prev = (*end_prev).max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}

fn part1(sensors: &[Sensor]) -> usize {
    const Y_TARGET: i32 = 2_000_000;

    let intervals = self::merge(
        sensors
            .iter()
            .filter_map(|sensor| sensor.coverage_at_y(Y_TARGET))
            .collect(),
    );

    #[allow(clippy::cast_sign_loss)]
    let coverage_at_y = intervals
        .into_iter()
        .map(|(start, end)| (end - start + 1) as usize)
        .sum::<usize>();

    let beacons_at_y = sensors
        .iter()
        .map(|sensor| sensor.beacon)
        .filter(|beacon| beacon.y() == Y_TARGET)
        .collect::<HashSet<_>>()
        .len();

    coverage_at_y - beacons_at_y
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let sensors = fs::read_to_string("in/day15.txt")?
        .lines()
        .map(Sensor::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&sensors);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 5_147_333);
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
