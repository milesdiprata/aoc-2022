use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

#[derive(Debug)]
struct Snafu {
    snafu: String,
    decimal: i64,
}

impl FromStr for Snafu {
    type Err = Error;

    fn from_str(snafu: &str) -> Result<Self> {
        let mut decimal = 0;
        let mut power = 1;

        for digit in snafu.chars().rev() {
            let val = match digit {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => bail!("invalid SNAFU digit '{digit}'"),
            };

            decimal += val * power;
            power *= 5;
        }

        Ok(Self {
            snafu: snafu.to_string(),
            decimal,
        })
    }
}

impl From<i64> for Snafu {
    fn from(mut decimal: i64) -> Self {
        let mut snafu = String::new();

        while decimal > 0 {
            let rem = decimal % 5;
            decimal /= 5;

            match rem {
                0 => snafu.push('0'),
                1 => snafu.push('1'),
                2 => snafu.push('2'),
                3 => {
                    // 3 = -2 + 5
                    snafu.push('=');
                    decimal += 1;
                }
                4 => {
                    // 4 = -1 + 5
                    snafu.push('-');
                    decimal += 1;
                }
                _ => unreachable!(),
            }
        }

        Self {
            snafu: snafu.chars().rev().collect(),
            decimal,
        }
    }
}

fn part1(fuel: &[Snafu]) -> String {
    Snafu::from(fuel.iter().map(|snafu| snafu.decimal).sum::<i64>()).snafu
}

fn main() -> Result<()> {
    let fuel = fs::read_to_string("in/day25.txt")?
        .lines()
        .map(Snafu::from_str)
        .collect::<Result<Vec<_>>>()?;

    dbg!(&fuel);

    {
        let start = Instant::now();
        let part1 = self::part1(&fuel);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, "2-2=12=1-=-1=000=222");
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(Snafu::from_str("1=-0-2").unwrap().decimal, 1747);
        assert_eq!(Snafu::from_str("12111").unwrap().decimal, 906);
        assert_eq!(Snafu::from_str("2=0=").unwrap().decimal, 198);
        assert_eq!(Snafu::from_str("21").unwrap().decimal, 11);
        assert_eq!(Snafu::from_str("2=01").unwrap().decimal, 201);
        assert_eq!(Snafu::from_str("111").unwrap().decimal, 31);
        assert_eq!(Snafu::from_str("20012").unwrap().decimal, 1257);
        assert_eq!(Snafu::from_str("112").unwrap().decimal, 32);
        assert_eq!(Snafu::from_str("1=-1=").unwrap().decimal, 353);
        assert_eq!(Snafu::from_str("1-12").unwrap().decimal, 107);
        assert_eq!(Snafu::from_str("12").unwrap().decimal, 7);
        assert_eq!(Snafu::from_str("1=").unwrap().decimal, 3);
        assert_eq!(Snafu::from_str("122").unwrap().decimal, 37);
    }

    #[test]
    fn from_decimal() {
        assert_eq!(Snafu::from(1747).snafu, "1=-0-2");
        assert_eq!(Snafu::from(906).snafu, "12111");
        assert_eq!(Snafu::from(198).snafu, "2=0=");
        assert_eq!(Snafu::from(11).snafu, "21");
        assert_eq!(Snafu::from(201).snafu, "2=01");
        assert_eq!(Snafu::from(31).snafu, "111");
        assert_eq!(Snafu::from(1257).snafu, "20012");
        assert_eq!(Snafu::from(32).snafu, "112");
        assert_eq!(Snafu::from(353).snafu, "1=-1=");
        assert_eq!(Snafu::from(107).snafu, "1-12");
        assert_eq!(Snafu::from(7).snafu, "12");
        assert_eq!(Snafu::from(3).snafu, "1=");
        assert_eq!(Snafu::from(37).snafu, "122");
    }
}
