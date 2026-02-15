use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use anyhow::Result;

#[derive(Debug)]
struct Signal {
    buffer: String,
}

impl Signal {
    fn find_unique_sequence(&self, len: usize) -> Option<usize> {
        self.buffer
            .as_bytes()
            .windows(len)
            .position(|w| w.iter().copied().collect::<HashSet<_>>().len() == len)
            .map(|i| i + len)
    }
}

fn main() -> Result<()> {
    let signal = Signal {
        buffer: fs::read_to_string("in/day6.txt")?,
    };

    {
        let start = Instant::now();
        let part1 = signal.find_unique_sequence(4).unwrap_or_default();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_238);
    };

    {
        let start = Instant::now();
        let part2 = signal.find_unique_sequence(14).unwrap_or_default();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 3_037);
    };

    Ok(())
}
