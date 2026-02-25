use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct Blueprint {
    id: u8,
    cost_ore_robot_ore: u8,
    cost_clay_robot_ore: u8,
    cost_obsidian_robot_ore: u8,
    cost_obsidian_robot_clay: u8,
    cost_geode_robot_ore: u8,
    cost_geode_robot_obsidian: u8,
    cost_ore_max: u8,
}

#[derive(Clone)]
struct State {
    minute: u8,
    ores: u16,
    clays: u16,
    obsidians: u16,
    geodes: u16,
    robots_ore: u16,
    robots_clay: u16,
    robots_obsidian: u16,
    robots_geode: u16,
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(blueprint: &str) -> Result<Self> {
        let mut nums = blueprint
            .split(|c: char| !c.is_ascii_digit())
            .flat_map(str::parse);

        Ok(Self::new(
            nums.next().ok_or_else(|| anyhow!("missing blueprint ID"))?,
            nums.next()
                .ok_or_else(|| anyhow!("missing ore robot cost"))?,
            nums.next()
                .ok_or_else(|| anyhow!("missing clay robot cost"))?,
            nums.next()
                .ok_or_else(|| anyhow!("missing obsidian robot ore cost"))?,
            nums.next()
                .ok_or_else(|| anyhow!("missing obsidian robot clay cost"))?,
            nums.next()
                .ok_or_else(|| anyhow!("missing geode robot ore cost"))?,
            nums.next()
                .ok_or_else(|| anyhow!("missing geode robot obsidian cost"))?,
        ))
    }
}

impl Blueprint {
    fn new(
        id: u8,
        cost_ore_robot_ore: u8,
        cost_clay_robot_ore: u8,
        cost_obsidian_robot_ore: u8,
        cost_obsidian_robot_clay: u8,
        cost_geode_robot_ore: u8,
        cost_geode_robot_obsidian: u8,
    ) -> Self {
        Self {
            id,
            cost_ore_robot_ore,
            cost_clay_robot_ore,
            cost_obsidian_robot_ore,
            cost_obsidian_robot_clay,
            cost_geode_robot_ore,
            cost_geode_robot_obsidian,
            cost_ore_max: cost_ore_robot_ore
                .max(cost_clay_robot_ore)
                .max(cost_obsidian_robot_ore)
                .max(cost_geode_robot_ore),
        }
    }
}

impl State {
    const fn new() -> Self {
        Self {
            minute: 0,
            ores: 0,
            clays: 0,
            obsidians: 0,
            geodes: 0,
            robots_ore: 1,
            robots_clay: 0,
            robots_obsidian: 0,
            robots_geode: 0,
        }
    }

    const fn build_robot_ore(self) -> Self {
        Self {
            robots_ore: self.robots_ore + 1,
            ..self
        }
    }

    const fn build_robot_clay(self) -> Self {
        Self {
            robots_clay: self.robots_clay + 1,
            ..self
        }
    }

    const fn build_robot_obsidian(self) -> Self {
        Self {
            robots_obsidian: self.robots_obsidian + 1,
            ..self
        }
    }

    const fn build_robot_geode(self) -> Self {
        Self {
            robots_geode: self.robots_geode + 1,
            ..self
        }
    }

    fn advance(&self, dt: u8, ore_cost: u8, clay_cost: u8, obsidian_cost: u8) -> Self {
        let minute = self.minute + dt;
        let dt = u16::from(dt);

        Self {
            minute,
            ores: self.ores + (self.robots_ore * dt) - u16::from(ore_cost),
            clays: self.clays + (self.robots_clay * dt) - u16::from(clay_cost),
            obsidians: self.obsidians + (self.robots_obsidian * dt) - u16::from(obsidian_cost),
            geodes: self.geodes + (self.robots_geode * dt),
            ..*self
        }
    }

    fn is_underproduced_ore(&self, bp: &Blueprint) -> bool {
        self.robots_ore < u16::from(bp.cost_ore_max)
    }

    fn is_underproduced_clay(&self, bp: &Blueprint) -> bool {
        self.robots_clay < u16::from(bp.cost_obsidian_robot_clay)
    }

    fn is_underproduced_obsidian(&self, bp: &Blueprint) -> bool {
        self.robots_obsidian < u16::from(bp.cost_geode_robot_obsidian)
    }

    fn time_until_affordable_ore(&self, bp: &Blueprint) -> Option<u8> {
        Self::time_until_affordable(self.ores, self.robots_ore, bp.cost_ore_robot_ore)
    }

    fn time_until_affordable_clay(&self, bp: &Blueprint) -> Option<u8> {
        Self::time_until_affordable(self.ores, self.robots_ore, bp.cost_clay_robot_ore)
    }

    fn time_until_affordable_obsidian(&self, bp: &Blueprint) -> Option<u8> {
        let ore =
            Self::time_until_affordable(self.ores, self.robots_ore, bp.cost_obsidian_robot_ore)?;
        let clay =
            Self::time_until_affordable(self.clays, self.robots_clay, bp.cost_obsidian_robot_clay)?;

        Some(ore.max(clay))
    }

    fn time_until_affordable_geode(&self, bp: &Blueprint) -> Option<u8> {
        let ore = Self::time_until_affordable(self.ores, self.robots_ore, bp.cost_geode_robot_ore)?;
        let obsidian = Self::time_until_affordable(
            self.obsidians,
            self.robots_obsidian,
            bp.cost_geode_robot_obsidian,
        )?;

        Some(ore.max(obsidian))
    }

    fn time_until_affordable(have: u16, production: u16, cost: u8) -> Option<u8> {
        let cost = u16::from(cost);

        if have >= cost {
            Some(0)
        } else if production == 0 {
            None
        } else {
            #[allow(clippy::cast_possible_truncation)]
            Some(((cost - have).div_ceil(production)) as u8)
        }
    }
}

fn dfs(state: &State, time: u8, bp: &Blueprint, best: &mut u16) -> u16 {
    let remaining = u16::from(time - state.minute);

    // If we build nothing else...
    let geodes_final = state.geodes + (remaining * state.robots_geode);
    *best = (*best).max(geodes_final);

    // Upper bound pruning
    let upper = geodes_final + ((remaining * (remaining - 1)) / 2);
    if *best >= upper {
        return geodes_final;
    }

    let mut geodes_max = geodes_final;

    if let Some(wait) = state.time_until_affordable_geode(bp) {
        let dt = wait + 1;
        if state.minute + dt < time {
            let next = state
                .advance(dt, bp.cost_geode_robot_ore, 0, bp.cost_geode_robot_obsidian)
                .build_robot_geode();
            geodes_max = geodes_max.max(self::dfs(&next, time, bp, best));
        }
    }

    if state.is_underproduced_obsidian(bp)
        && let Some(wait) = state.time_until_affordable_obsidian(bp)
    {
        let dt = wait + 1;
        if state.minute + dt < time {
            let next = state
                .advance(
                    dt,
                    bp.cost_obsidian_robot_ore,
                    bp.cost_obsidian_robot_clay,
                    0,
                )
                .build_robot_obsidian();
            geodes_max = geodes_max.max(self::dfs(&next, time, bp, best));
        }
    }

    if state.is_underproduced_clay(bp)
        && let Some(wait) = state.time_until_affordable_clay(bp)
    {
        let dt = wait + 1;
        if state.minute + dt < time {
            let next = state
                .advance(dt, bp.cost_clay_robot_ore, 0, 0)
                .build_robot_clay();
            geodes_max = geodes_max.max(dfs(&next, time, bp, best));
        }
    }

    if state.is_underproduced_ore(bp)
        && let Some(wait) = state.time_until_affordable_ore(bp)
    {
        let dt = wait + 1;
        if state.minute + dt < time {
            let next = state
                .advance(dt, bp.cost_ore_robot_ore, 0, 0)
                .build_robot_ore();
            geodes_max = geodes_max.max(dfs(&next, time, bp, best));
        }
    }

    geodes_max
}

fn part1(blueprints: &[Blueprint]) -> u16 {
    const TIME: u8 = 24;

    blueprints
        .iter()
        .map(|bp| (bp.id, self::dfs(&State::new(), TIME, bp, &mut 0)))
        .map(|(id, geodes)| u16::from(id) * geodes)
        .sum()
}

fn part2(blueprints: &[Blueprint]) -> u16 {
    const TIME: u8 = 32;

    blueprints
        .iter()
        .take(3)
        .map(|bp| self::dfs(&State::new(), TIME, bp, &mut 0))
        .product()
}

fn main() -> Result<()> {
    let blueprints = fs::read_to_string("in/day19.txt")?
        .lines()
        .map(Blueprint::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&blueprints);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_650);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&blueprints);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 5_824);
    };

    Ok(())
}
