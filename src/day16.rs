use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<String>,
}

#[derive(Debug)]
struct Cave {
    start: usize,
    rates: Box<[u32]>,
    dist: Box<[Box<[u32]>]>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(valve: &str) -> Result<Self> {
        let err = || anyhow!("invalid report '{valve}'");

        let valve = valve.strip_prefix("Valve ").ok_or_else(err)?;
        let (name, valve) = valve.split_once(" has flow rate=").ok_or_else(err)?;
        let (rate, valve) = valve.split_once("; ").ok_or_else(err)?;
        let tunnels = valve
            .strip_prefix("tunnel leads to valve ")
            .or_else(|| valve.strip_prefix("tunnels lead to valves "))
            .ok_or_else(err)?;

        Ok(Self {
            name: name.to_string(),
            rate: rate.parse()?,
            tunnels: tunnels.split(", ").map(str::to_string).collect(),
        })
    }
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(report: &str) -> Result<Self> {
        let valves = report
            .lines()
            .map(Valve::from_str)
            .collect::<Result<Box<_>>>()?;

        Self::try_from(valves.as_ref())
    }
}

impl TryFrom<&[Valve]> for Cave {
    type Error = Error;

    fn try_from(valves: &[Valve]) -> Result<Self> {
        let adj = {
            let idxs = valves
                .iter()
                .enumerate()
                .map(|(i, v)| (v.name.as_str(), i))
                .collect::<HashMap<_, _>>();

            valves
                .iter()
                .map(|v| v.tunnels.iter().map(|t| idxs[t.as_str()]))
                .map(Iterator::collect::<Vec<_>>)
                .collect::<Vec<_>>()
        };

        let interesting = valves
            .iter()
            .enumerate()
            .filter(|&(_, valve)| valve.name == "AA" || valve.rate > 0)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let start = interesting
            .iter()
            .position(|&i| valves[i].name == "AA")
            .ok_or_else(|| anyhow!("missing valve 'AA'"))?;

        let rates = interesting.iter().map(|&i| valves[i].rate).collect();

        let dist = interesting
            .iter()
            .map(|&src| {
                let mut dist = vec![u32::MAX; valves.len()];
                let mut queue = VecDeque::new();

                dist[src] = 0;
                queue.push_back(src);

                while let Some(cur) = queue.pop_front() {
                    for &next in &adj[cur] {
                        if dist[next] == u32::MAX {
                            dist[next] = dist[cur] + 1;
                            queue.push_back(next);
                        }
                    }
                }

                interesting.iter().map(|&dst| dist[dst]).collect()
            })
            .collect();

        Ok(Self { start, rates, dist })
    }
}

impl Cave {
    fn dfs(&self, time: u32, cur: usize, opened: u32, pressure: u32, best: &mut HashMap<u32, u32>) {
        let entry = best.entry(opened).or_default();
        *entry = (*entry).max(pressure);

        for next in 0..self.rates.len() {
            if opened & (1 << next) != 0 || self.rates[next] == 0 {
                continue;
            }

            let time_to_next = self.dist[cur][next] + 1;
            if time_to_next >= time {
                continue;
            }

            let time_remaining = time - time_to_next;
            let released = self.rates[next] * time_remaining;
            let opened = opened | (1 << next);

            self.dfs(time_remaining, next, opened, pressure + released, best);
        }
    }

    fn max_pressure(&self, time: u32) -> u32 {
        let mut best = HashMap::new();
        self.dfs(time, self.start, 0, 0, &mut best);
        best.into_values().max().unwrap_or_default()
    }

    fn max_pressure_with_elephant(&self, time: u32) -> u32 {
        let mut best = HashMap::new();
        self.dfs(time, self.start, 0, 0, &mut best);

        let mut max = 0;
        for (&mask_a, &score_a) in &best {
            for (&mask_b, &score_b) in &best {
                if mask_a & mask_b == 0 {
                    max = max.max(score_a + score_b);
                }
            }
        }

        max
    }
}

fn part1(cave: &Cave) -> u32 {
    const TIME: u32 = 30;
    cave.max_pressure(TIME)
}

fn part2(cave: &Cave) -> u32 {
    const TIME: u32 = 26;
    cave.max_pressure_with_elephant(TIME)
}

fn main() -> Result<()> {
    let cave = Cave::from_str(&fs::read_to_string("in/day16.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&cave);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_880);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&cave);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 2_520);
    };

    Ok(())
}
