use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Debug)]
enum Job {
    Num(i64),
    Operation {
        lhs: String,
        rhs: String,
        op: Operator,
    },
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job,
}

impl FromStr for Job {
    type Err = Error;

    fn from_str(job: &str) -> Result<Self> {
        if let Ok(num) = job.parse().with_context(|| format!("invalid job '{job}'")) {
            Ok(Self::Num(num))
        } else {
            let parts = job.split_ascii_whitespace().collect::<Vec<_>>();
            let &[lhs, op, rhs] = parts.as_slice() else {
                bail!("invalid job '{job}'");
            };

            Ok(Self::Operation {
                lhs: lhs.to_string(),
                rhs: rhs.to_string(),
                op: op
                    .chars()
                    .next()
                    .with_context(|| format!("invalid job '{job}'"))
                    .and_then(Operator::try_from)?,
            })
        }
    }
}

impl TryFrom<char> for Operator {
    type Error = Error;

    fn try_from(operator: char) -> Result<Self> {
        match operator {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            _ => bail!("invalid operator '{operator}'"),
        }
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(monkey: &str) -> Result<Self> {
        let (name, job) = monkey
            .split_once(": ")
            .ok_or_else(|| anyhow!("invalid monkey '{monkey}'"))?;

        Ok(Self {
            name: name.to_string(),
            job: job
                .parse()
                .with_context(|| format!("invalid monkey '{monkey}'"))?,
        })
    }
}

impl Operator {
    const fn eval(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
        }
    }
}

fn part1(jobs: &HashMap<String, Job>) -> i64 {
    fn dfs<'a>(
        jobs: &'a HashMap<String, Job>,
        monkey: &'a str,
        cache: &mut HashMap<&'a str, i64>,
    ) -> i64 {
        if let Some(&num) = cache.get(monkey) {
            return num;
        }

        let num = match &jobs[monkey] {
            &Job::Num(num) => num,
            Job::Operation { lhs, rhs, op } => {
                op.eval(dfs(jobs, lhs, cache), dfs(jobs, rhs, cache))
            }
        };

        cache.insert(monkey, num);
        num
    }

    let mut cache = HashMap::new();
    dfs(jobs, "root", &mut cache)
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let jobs = fs::read_to_string("in/day21.txt")?
        .lines()
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|monkey| (monkey.name, monkey.job))
        .collect();

    {
        let start = Instant::now();
        let part1 = self::part1(&jobs);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 309_248_622_142_100);
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
