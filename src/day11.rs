use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Old,
    Val(u64),
}

#[derive(Clone, Debug)]
struct Operation {
    lhs: Operand,
    rhs: Operand,
    op: Operator,
}

#[derive(Clone, Debug)]
struct Test {
    divisible_by: u64,
    pass: usize,
    fail: usize,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Clone, Debug)]
struct Game {
    monkeys: Vec<Monkey>,
    inspections: Vec<usize>,
    divisible_by_lcm: u64,
}

impl TryFrom<char> for Operator {
    type Error = Error;

    fn try_from(operator: char) -> Result<Self> {
        match operator {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mul),
            _ => bail!("invalid operator '{operator}'"),
        }
    }
}

impl FromStr for Operand {
    type Err = Error;

    fn from_str(operand: &str) -> Result<Self> {
        match operand {
            "old" => Ok(Self::Old),
            _ => Ok(Self::Val(operand.parse()?)),
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(operation: &str) -> Result<Self> {
        let mut operation = operation
            .trim_ascii_start()
            .strip_prefix("Operation: new = ")
            .ok_or_else(|| anyhow!("missing prefix 'Operation: new = '"))?
            .split_ascii_whitespace();

        let lhs = operation
            .next()
            .ok_or_else(|| anyhow!("missing lhs in operation"))?
            .parse()?;
        let op = operation
            .next()
            .and_then(|operator| operator.chars().next())
            .map(Operator::try_from)
            .ok_or_else(|| anyhow!("missing op in operation"))??;
        let rhs = operation
            .next()
            .ok_or_else(|| anyhow!("missing rhs in operation"))?
            .parse()?;

        Ok(Self { lhs, rhs, op })
    }
}

impl FromStr for Test {
    type Err = Error;

    fn from_str(test: &str) -> Result<Self> {
        let mut lines = test.lines().map(str::trim_ascii_start);

        let divisible_by = lines
            .next()
            .ok_or_else(|| anyhow!("missing first line in test"))?
            .strip_prefix("Test: divisible by ")
            .ok_or_else(|| anyhow!("missing prefix 'Test: divisible by '"))?
            .parse()?;
        let pass = lines
            .next()
            .ok_or_else(|| anyhow!("missing second line in test"))?
            .strip_prefix("If true: throw to monkey ")
            .ok_or_else(|| anyhow!("missing prefix 'If true: throw to monkey '"))?
            .parse()?;
        let fail = lines
            .next()
            .ok_or_else(|| anyhow!("missing third line in test"))?
            .strip_prefix("If false: throw to monkey ")
            .ok_or_else(|| anyhow!("missing prefix 'If false: throw to monkey '"))?
            .parse()?;

        Ok(Self {
            divisible_by,
            pass,
            fail,
        })
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(monkey: &str) -> Result<Self> {
        let mut lines = monkey.lines();

        let _id = lines
            .next()
            .ok_or_else(|| anyhow!("missing first line in monkey info"))?
            .strip_prefix("Monkey ")
            .and_then(|line| line.strip_suffix(':'))
            .ok_or_else(|| anyhow!("missing prefix 'Monkey ' and suffix ':'"))?
            .parse::<usize>()?;
        let items = lines
            .next()
            .ok_or_else(|| anyhow!("missing second line in monkey info"))?
            .trim_ascii_start()
            .strip_prefix("Starting items: ")
            .ok_or_else(|| anyhow!("missing prefix 'Starting items: '"))?
            .split(", ")
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let operation = lines
            .next()
            .ok_or_else(|| anyhow!("missing third line in monkey info"))
            .map(Operation::from_str)??;
        let test = Test::from_str(&lines.collect::<Vec<_>>().join("\n"))?;

        Ok(Self {
            items,
            operation,
            test,
        })
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let monkeys = input
            .split("\n\n")
            .map(Monkey::from_str)
            .collect::<Result<Vec<_>>>()?;
        let inspections = vec![0; monkeys.len()];

        // Note: all `divisible_by` values are prime, so no need to compute LCM
        let divisible_by_lcm = monkeys
            .iter()
            .map(|monkey| monkey.test.divisible_by)
            .product();

        Ok(Self {
            monkeys,
            inspections,
            divisible_by_lcm,
        })
    }
}

impl Operator {
    const fn apply(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
        }
    }
}

impl Operation {
    const fn apply(&self, old: u64) -> u64 {
        let (lhs, rhs) = match (self.lhs, self.rhs) {
            (Operand::Old, Operand::Old) => (old, old),
            (Operand::Old, Operand::Val(rhs)) => (old, rhs),
            (Operand::Val(lhs), Operand::Old) => (lhs, old),
            (Operand::Val(lhs), Operand::Val(rhs)) => (lhs, rhs),
        };

        self.op.apply(lhs, rhs)
    }
}

impl Monkey {
    const fn inspect(&self, item: u64, divide: bool, lcm: u64) -> u64 {
        let worry = self.operation.apply(item);
        if divide { worry / 3 } else { worry % lcm }
    }

    const fn test(&self, item: u64) -> usize {
        if item.is_multiple_of(self.test.divisible_by) {
            self.test.pass
        } else {
            self.test.fail
        }
    }
}

impl Game {
    fn play_round(mut self, divide: bool) -> Self {
        for i in 0..self.monkeys.len() {
            while let Some(item) = self.monkeys[i].items.pop_front() {
                let inspected = self.monkeys[i].inspect(item, divide, self.divisible_by_lcm);
                let j = self.monkeys[i].test(inspected);
                self.monkeys[j].items.push_back(inspected);
                self.inspections[i] += 1;
            }
        }

        self
    }

    fn monkey_business(&self) -> usize {
        const MOST_ACTIVE_MONKEYS: usize = 2;

        let mut inspections = BinaryHeap::with_capacity(MOST_ACTIVE_MONKEYS + 1);
        for &inspection in &self.inspections {
            inspections.push(Reverse(inspection));

            if inspections.len() > MOST_ACTIVE_MONKEYS {
                inspections.pop();
            }
        }

        inspections
            .into_iter()
            .map(|Reverse(inspection)| inspection)
            .product()
    }
}

fn simulate(mut game: Game, rounds: usize, divide: bool) -> usize {
    for _ in 0..rounds {
        game = game.play_round(divide);
    }

    game.monkey_business()
}

fn main() -> Result<()> {
    let game = Game::from_str(&fs::read_to_string("in/day11.txt")?)?;

    {
        let game = game.clone();
        let start = Instant::now();
        let part1 = self::simulate(game, 20, true);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 118_674);
    };

    {
        let start = Instant::now();
        let part2 = self::simulate(game, 10_000, false);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 32_333_418_600);
    };

    Ok(())
}
