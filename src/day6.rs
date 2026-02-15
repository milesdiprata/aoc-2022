use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use anyhow::Result;

fn part1(buffer: &str) -> Option<usize> {
    buffer
        .as_bytes()
        .windows(4)
        .position(|w| w.iter().copied().collect::<HashSet<_>>().len() == 4)
        .map(|i| i + 4)
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let buffer = fs::read_to_string("in/day6.txt")?;

    {
        let start = Instant::now();
        let part1 = self::part1(&buffer).unwrap_or_default();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_238);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 3_037);
    };

    Ok(())
}
