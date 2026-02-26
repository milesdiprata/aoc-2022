use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Debug)]
struct EncryptedFile {
    vals: Vec<i64>,
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
    fn mix(self, rounds: usize) -> Result<Self> {
        let mut idxs = (0..self.vals.len()).collect::<Vec<_>>();

        for _ in 0..rounds {
            for i in 0..self.vals.len() {
                let idx = idxs.iter().position(|&idx| idx == i).unwrap();
                idxs.remove(idx);

                let idx_new = (isize::try_from(self.vals[i])? + idx.cast_signed())
                    .rem_euclid(idxs.len().cast_signed());
                idxs.insert(idx_new.cast_unsigned(), i);
            }
        }

        Ok(Self {
            vals: idxs.into_iter().map(|i| self.vals[i]).collect(),
        })
    }

    fn grove_coords(&self) -> [i64; 3] {
        let idx_zero = self.vals.iter().position(|&val| val == 0).unwrap();
        [1_000, 2_000, 3_000]
            .map(|idx| (idx + idx_zero) % self.vals.len())
            .map(|idx| self.vals[idx])
    }
}

fn part1(file: EncryptedFile) -> Result<i64> {
    Ok(file.mix(1)?.grove_coords().iter().sum())
}

fn part2(file: EncryptedFile) -> Result<i64> {
    const DECRYPTION_KEY: i64 = 811_589_153;

    let file = EncryptedFile {
        vals: file
            .vals
            .into_iter()
            .map(|val| DECRYPTION_KEY * val)
            .collect(),
    };

    Ok(file.mix(10)?.grove_coords().iter().sum())
}

fn main() -> Result<()> {
    let file = EncryptedFile::from_str(&fs::read_to_string("in/day20.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(file.clone())?;
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 7_225);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(file)?;
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 548_634_267_428);
    };

    Ok(())
}
