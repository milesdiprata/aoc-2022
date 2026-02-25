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

    const fn is_ore_buildable(&self, blueprint: &Blueprint) -> bool {
        self.ores >= blueprint.cost_ore_robot_ore as u16
    }

    const fn is_clay_buildable(&self, blueprint: &Blueprint) -> bool {
        self.ores >= blueprint.cost_clay_robot_ore as u16
    }

    const fn is_obsidian_buildable(&self, blueprint: &Blueprint) -> bool {
        self.ores >= blueprint.cost_obsidian_robot_ore as u16
            && self.clays >= blueprint.cost_obsidian_robot_clay as u16
    }

    const fn is_geode_buildable(&self, blueprint: &Blueprint) -> bool {
        self.ores >= blueprint.cost_geode_robot_ore as u16
            && self.obsidians >= blueprint.cost_geode_robot_obsidian as u16
    }

    const fn is_ore_underproduced(&self, blueprint: &Blueprint) -> bool {
        self.robots_ore < blueprint.cost_ore_max as u16
    }

    const fn is_clay_underproduced(&self, blueprint: &Blueprint) -> bool {
        self.robots_clay < blueprint.cost_obsidian_robot_clay as u16
    }

    const fn is_obsidian_underproduced(&self, blueprint: &Blueprint) -> bool {
        self.robots_obsidian < blueprint.cost_geode_robot_obsidian as u16
    }

    const fn collect(&self) -> Self {
        Self {
            minute: self.minute + 1,
            ores: self.ores + self.robots_ore,
            clays: self.clays + self.robots_clay,
            obsidians: self.obsidians + self.robots_obsidian,
            geodes: self.geodes + self.robots_geode,
            ..*self
        }
    }

    const fn build_ore(&self, blueprint: &Blueprint) -> Self {
        Self {
            ores: self.ores - blueprint.cost_ore_robot_ore as u16,
            robots_ore: self.robots_ore + 1,
            ..*self
        }
    }

    const fn build_clay(&self, blueprint: &Blueprint) -> Self {
        Self {
            ores: self.ores - blueprint.cost_clay_robot_ore as u16,
            robots_clay: self.robots_clay + 1,
            ..*self
        }
    }

    const fn build_obsidian(&self, blueprint: &Blueprint) -> Self {
        Self {
            ores: self.ores - blueprint.cost_obsidian_robot_ore as u16,
            clays: self.clays - blueprint.cost_obsidian_robot_clay as u16,
            robots_obsidian: self.robots_obsidian + 1,
            ..*self
        }
    }

    const fn build_geode(&self, blueprint: &Blueprint) -> Self {
        Self {
            ores: self.ores - blueprint.cost_geode_robot_ore as u16,
            obsidians: self.obsidians - blueprint.cost_geode_robot_obsidian as u16,
            robots_geode: self.robots_geode + 1,
            ..*self
        }
    }
}

fn part1(blueprints: &[Blueprint]) -> u16 {
    // Pruning strategies
    //   1. Upper bound: If an optimistic upper bound cannot be beat, then
    //   prune; assumes that we continue to make geode robots for the rest of
    //   the remaining turns (triangular number)
    //   2. Don't overproduce robots: do not build more robots of a type than
    //   the maximum cost of that resource across all recipes
    //   3. Always build a geode robot if possible: skip all other decision
    //   branches when it is possible to build a geode robot
    //   4. Prioritize building geode robots first: explore building geode
    //   robot decision branch first so that strategy #1 has impact earlier
    fn dfs(state: State, bp: &Blueprint, geodes_max: &mut u16) -> State {
        const TIME: u8 = 24;

        if state.minute >= TIME {
            return state;
        }

        let remaining = u16::from(TIME - state.minute);
        let upper_bound =
            state.geodes + (remaining * state.robots_geode) + ((remaining * (remaining - 1)) / 2);
        if *geodes_max >= upper_bound {
            return state;
        }

        let build_geode = state.is_geode_buildable(bp);
        let build_ore =
            !build_geode && state.is_ore_buildable(bp) && state.is_ore_underproduced(bp);
        let build_clay =
            !build_geode && state.is_clay_buildable(bp) && state.is_clay_underproduced(bp);
        let build_obsidian =
            !build_geode && state.is_obsidian_buildable(bp) && state.is_obsidian_underproduced(bp);

        let state = state.collect();
        let states = [
            build_geode.then(|| state.build_geode(bp)),
            build_ore.then(|| state.build_ore(bp)),
            build_clay.then(|| state.build_clay(bp)),
            build_obsidian.then(|| state.build_obsidian(bp)),
            (!build_geode).then_some(state),
        ];

        let best = states
            .into_iter()
            .flatten()
            .map(|state| dfs(state, bp, geodes_max))
            .max_by_key(|state| state.geodes)
            .unwrap();

        *geodes_max = (*geodes_max).max(best.geodes);

        best
    }

    blueprints
        .iter()
        .map(|blueprint| (blueprint.id, dfs(State::new(), blueprint, &mut 0).geodes))
        .map(|(id, geodes)| u16::from(id) * geodes)
        .sum()
}

fn part2() -> u64 {
    todo!()
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
        let part2 = self::part2();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 0);
    };

    Ok(())
}
