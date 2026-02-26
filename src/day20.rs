use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct EncryptedFile {
    vals: Vec<i16>,
}

impl FromStr for EncryptedFile {
    type Err = Error;

    fn from_str(file: &str) -> Result<Self> {
        Ok(Self {
            vals: file.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

impl EncryptedFile {
    fn mix(self) -> Self {
        let mut idxs = (0..self.vals.len()).collect::<Vec<_>>();

        for i in 0..self.vals.len() {
            let idx = idxs.iter().position(|&idx| idx == i).unwrap();
            idxs.remove(idx);

            let idx_new = (isize::from(self.vals[i]) + idx.cast_signed())
                .rem_euclid(idxs.len().cast_signed());
            idxs.insert(idx_new.cast_unsigned(), i);
        }

        Self {
            vals: idxs.into_iter().map(|i| self.vals[i]).collect(),
        }
    }
}

fn part1(file: EncryptedFile) -> i16 {
    let file = file.mix();
    let idx_zero = file.vals.iter().position(|&val| val == 0).unwrap();

    [1_000, 2_000, 3_000]
        .into_iter()
        .map(|idx| (idx + idx_zero) % file.vals.len())
        .map(|idx| file.vals[idx])
        .sum()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let file = EncryptedFile::from_str(&fs::read_to_string("in/day20.txt")?)?;

    dbg!(&file);

    {
        let start = Instant::now();
        let part1 = self::part1(file);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 7_225);
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
