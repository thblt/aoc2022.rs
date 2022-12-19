use lib::*;
use sscanf::sscanf;
use std::{collections::HashSet, fmt::Debug};

type Ore = usize;
type Clay = usize;
type Obsidian = usize;

type Futures = HashSet<Factory>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Log {
    geodes: usize,
    history: Vec<Factory>,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Exclusions {
    no_ore_robot: bool,
    no_clay_robot: bool,
    no_obsidian_robot: bool,
    no_geode_robot: bool,
}

impl Default for Exclusions {
    fn default() -> Self {
        Self {
            no_ore_robot: false,
            no_clay_robot: false,
            no_obsidian_robot: false,
            no_geode_robot: false,
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Factory {
    ore_store: Ore,
    clay_store: Clay,
    obsidian_store: Obsidian,

    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,

    geodes: usize,

    exclusions: Exclusions,
}

impl Factory {
    fn new() -> Factory {
        Factory {
            ore_store: 0,
            clay_store: 0,
            obsidian_store: 0,
            geodes: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,

            exclusions: Exclusions::default(),
        }
    }

    fn make_robots(&self, blueprint: &Blueprint) -> Futures {
        let mut ret = Futures::new();

        // Determine what can of robots we can make.

        let ore_p = self.ore_store >= blueprint.ore_cost;
        let clay_p = self.ore_store >= blueprint.clay_cost;
        let obsidian_p = self.ore_store >= blueprint.obsidian_cost.0
            && self.clay_store >= blueprint.obsidian_cost.1;
        let geode_p = self.ore_store >= blueprint.geode_cost.0
            && self.obsidian_store >= blueprint.geode_cost.1;

        if !(ore_p && clay_p && obsidian_p && geode_p) {
            let mut new = self.clone();
            new.exclusions.no_ore_robot |= ore_p;
            new.exclusions.no_clay_robot |= clay_p;
            new.exclusions.no_obsidian_robot |= obsidian_p;
            new.exclusions.no_geode_robot |= geode_p;
            ret.insert(new);
        }

        // Then we create as many new factories as we can, each
        // building a possible robot (a factory can create at most one
        // robot per minute)

        if ore_p && !self.exclusions.no_ore_robot {
            let mut new = self.clone();
            new.ore_store -= blueprint.ore_cost;
            new.ore_robots += 1;
            new.exclusions = Exclusions::default();
            ret.insert(new);
        }
        if clay_p && !self.exclusions.no_clay_robot {
            let mut new = self.clone();
            new.ore_store -= blueprint.clay_cost;
            new.clay_robots += 1;
            new.exclusions = Exclusions::default();
            ret.insert(new);
        }
        if obsidian_p && !self.exclusions.no_obsidian_robot {
            let mut new = self.clone();
            new.ore_store -= blueprint.obsidian_cost.0;
            new.clay_store -= blueprint.obsidian_cost.1;
            new.obsidian_robots += 1;
            new.exclusions = Exclusions::default();
            ret.insert(new);
        }
        if geode_p && !self.exclusions.no_geode_robot {
            let mut new = self.clone();
            new.ore_store -= blueprint.geode_cost.0;
            new.obsidian_store -= blueprint.geode_cost.1;
            new.geode_robots += 1;
            new.exclusions = Exclusions::default();
            ret.insert(new);
        }
        ret
    }

    fn make_future_states(&self, blueprint: &Blueprint) -> Futures {
        // Mine materials
        let new_ore = self.ore_robots;
        let new_clay = self.clay_robots;
        let new_obsidian = self.obsidian_robots;
        let new_geodes = self.geode_robots;

        // Build robots
        let mut futures = self.make_robots(blueprint);

        futures = futures
            .into_iter()
            .map(|mut f| {
                f.ore_store += new_ore;
                f.clay_store += new_clay;
                f.obsidian_store += new_obsidian;
                f.geodes += new_geodes;
                f
            })
            .collect::<HashSet<Factory>>();
        futures
    }

    fn break_geodes(&self, blueprint: &Blueprint, time: usize) -> Option<Log> {
        if time == 0 {
            return Some(Log {
                geodes: self.geodes,
                history: vec![*self],
            });
        }

        let mut max = 0;
        let mut best: Option<Log> = None;

        let mut futures = self.make_future_states(blueprint);
        for f in futures.drain() {
            let log = f.break_geodes(blueprint, time - 1);
            if let Some(mut log) = log {
                if log.geodes > max {
                    log.history.push(*self);
                    max = log.geodes;
                    best = Some(log);
                }
            }
        }
        best
    }
}

impl Debug for Factory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Store: ore={:2} c={:2} obs={:2} g={:?}) (Robots: ore={:?} c={:?} obs={:?} g={:?})",
            self.ore_store,
            self.clay_store,
            self.obsidian_store,
            self.geodes,
            self.ore_robots,
            self.clay_robots,
            self.obsidian_robots,
            self.geode_robots,
        )
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Blueprint {
    id: usize,
    ore_cost: Ore,
    clay_cost: Ore,
    obsidian_cost: (Ore, Clay),
    geode_cost: (Ore, Obsidian),
}

fn read_input(s: &str) -> Vec<Blueprint> {
    let mut ret = vec![];
    for line in read_lines(s) {
        let line = line.unwrap();
        let
            (id, ore_cost, clay_cost, obsidian_cost_ore, obsidian_cost_clay, geode_cost_ore, geode_cost_obsidian) =
            sscanf!(line,
                    "Blueprint {usize}: Each ore robot costs {usize} ore. Each clay robot costs {usize} ore. Each obsidian robot costs {usize} ore and {usize} clay. Each geode robot costs {usize} ore and {usize} obsidian.").unwrap();
        ret.push(Blueprint {
            id,
            ore_cost,
            clay_cost,
            obsidian_cost: (obsidian_cost_ore, obsidian_cost_clay),
            geode_cost: (geode_cost_ore, geode_cost_obsidian),
        })
    }
    ret
}

fn main() {
    let time = 24;
    let mut result = 0;
    for blueprint in read_input("inputs/19.txt").into_iter().skip(1) {
        println!("Blueprint {0}", blueprint.id);
        if let Some(score) = Factory::new().break_geodes(&blueprint, time) {
            println!(" - Geodes: {:?}", score.geodes);
            result += blueprint.id * score.geodes;
            for (minute, state) in score.history.into_iter().enumerate() {
                println!("{:2} {:?}", time - minute, state);
            }
        } else {
            println!(" - NOTHING");
        }

    }
    println!("Result: {}", result);
}
