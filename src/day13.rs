use std::cmp::Ordering;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Debug, PartialEq, Eq)]
enum Val {
    Int(u8),
    List(Vec<Val>),
}

#[derive(Debug)]
struct Pair(Val, Val);

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Int(int) => write!(f, "{int}"),
            Self::List(vals) => {
                write!(f, "[")?;
                for (i, val) in vals.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }

                    write!(f, "{val}")?;
                }
                write!(f, "]")
            }
        }
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", &self.0, &self.1)
    }
}

impl FromStr for Val {
    type Err = Error;

    fn from_str(val: &str) -> Result<Self> {
        fn parse(input: &[u8], idx: &mut usize) -> Result<Val> {
            match input.get(*idx) {
                Some(b'[') => {
                    *idx += 1;

                    let mut list = Vec::new();
                    while input[*idx] != b']' {
                        if input[*idx] == b',' {
                            *idx += 1;
                        }

                        list.push(parse(input, idx)?);
                    }

                    *idx += 1;

                    Ok(Val::List(list))
                }
                Some(&c) if c.is_ascii_digit() => {
                    let start = *idx;
                    while input.get(*idx).is_some_and(|&c| c.is_ascii_digit()) {
                        *idx += 1;
                    }

                    Ok(Val::Int(str::from_utf8(&input[start..*idx])?.parse()?))
                }
                _ => bail!(
                    "invalid character at position {idx} in '{}'",
                    str::from_utf8(input)?,
                ),
            }
        }

        parse(val.as_bytes(), &mut 0)
    }
}

impl FromStr for Pair {
    type Err = Error;

    fn from_str(pair: &str) -> Result<Self> {
        let (lhs, rhs) = pair
            .split_once('\n')
            .ok_or_else(|| anyhow!("invalid pair '{pair}'"))?;
        let (lhs_val, rhs_val) = (lhs.parse()?, rhs.parse()?);

        if !matches!(lhs_val, Val::List(_)) {
            bail!("invalid packet '{lhs}'");
        }

        if !matches!(rhs_val, Val::List(_)) {
            bail!("invalid packet '{rhs}'");
        }

        Ok(Self(lhs_val, rhs_val))
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Self::Int(a), &Self::Int(b)) => a.cmp(&b),
            (&Self::Int(int), Self::List(_)) => Self::List(vec![Self::Int(int)]).cmp(other),
            (Self::List(_), &Self::Int(int)) => self.cmp(&Self::List(vec![Self::Int(int)])),
            (Self::List(lhs), Self::List(rhs)) => {
                for (a, b) in lhs.iter().zip(rhs.iter()) {
                    let ord = a.cmp(b);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }

                lhs.len().cmp(&rhs.len())
            }
        }
    }
}

impl Pair {
    fn is_order_correct(&self) -> bool {
        self.0 <= self.1
    }
}

fn part1(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| (idx + 1, pair))
        .filter_map(|(idx, pair)| pair.is_order_correct().then_some(idx))
        .sum()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let pairs = fs::read_to_string("in/day13.txt")?
        .split("\n\n")
        .map(Pair::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&pairs);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 5_625);
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
