use anyhow::Result;
use std::fs;
use regex::Regex;
use std::cmp::max;

#[derive(Debug)]
struct Blueprint {
    number: u16,
    ore: u16,
    clay: u16,
    obsidian: (u16, u16),
    geode: (u16, u16),
}

impl Blueprint {
    fn max_ore(&self) -> u16 {
        max(max(self.ore, self.clay), max(self.obsidian.0, self.geode.0))
    }

    fn max_clay(&self) -> u16 {
        self.obsidian.1
    }

    fn max_obsidian(&self) -> u16 {
        self.geode.1
    }
}

fn main() -> Result<()> {
    let blueprints: Vec<Blueprint> = parse_input(fs::read_to_string("day19.input")?.as_str());
    println!("{}", part1(&blueprints));
    println!("{}", part2(&blueprints));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input.split("\n").map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Blueprint {
    let re = Regex::new(r"Blueprint (?P<number1>\d+): Each ore robot costs (?P<number2>\d+) ore. Each clay robot costs (?P<number3>\d+) ore. Each obsidian robot costs (?P<number4>\d+) ore and (?P<number5>\d+) clay. Each geode robot costs (?P<number6>\d+) ore and (?P<number7>\d+) obsidian.")
    .expect("invalid regex");
    let caps = re.captures(line).expect("parse {line}");
    Blueprint {
        number: caps["number1"].parse::<u16>().unwrap(),
        ore: caps["number2"].parse::<u16>().unwrap(),
        clay: caps["number3"].parse::<u16>().unwrap(),
        obsidian: (caps["number4"].parse::<u16>().unwrap(), caps["number5"].parse::<u16>().unwrap()),
        geode: (caps["number6"].parse::<u16>().unwrap(), caps["number7"].parse::<u16>().unwrap()),
    }
}

fn part1(blueprints: &Vec<Blueprint>) -> u16 {
    blueprints.iter()
        .map(|blueprint| blueprint.number * dfs(&blueprint, (0,0,0), (1,0,0), (false, false, false), 23))
        .sum()
}

fn dfs(blueprint: &Blueprint, resource: (u16, u16, u16), robots: (u16, u16, u16), skip: (bool, bool, bool), time: u16) -> u16 {
    if time == 0 {
        return 0;
    }
    let mut max_geode = 0;
    let mut current_skip = skip.clone();
    if resource.0 >= blueprint.geode.0 && resource.2 >= blueprint.geode.1 {
        max_geode = max(max_geode, time + dfs(
            blueprint,
            (resource.0 + robots.0 - blueprint.geode.0, resource.1 + robots.1 , resource.2 + robots.2 - blueprint.geode.1),
            (robots.0, robots.1, robots.2),
            (false, false, false),
            time - 1));
    }
    if !skip.0 && robots.2 < blueprint.max_obsidian() && resource.0 >= blueprint.obsidian.0 && resource.1 >= blueprint.obsidian.1 {
        max_geode = max(max_geode, dfs(
            blueprint,
            (resource.0 + robots.0 - blueprint.obsidian.0, resource.1 + robots.1 - blueprint.obsidian.1, resource.2 + robots.2),
            (robots.0, robots.1, robots.2 + 1),
            (false, false, false),
            time - 1));
        current_skip.0 = true;
    }
    if !skip.1 && robots.1 < blueprint.max_clay() && resource.0 >= blueprint.clay {
        max_geode = max(max_geode, dfs(
            blueprint,
            (resource.0 + robots.0 - blueprint.clay, resource.1 + robots.1, resource.2 + robots.2),
            (robots.0, robots.1 + 1, robots.2),
            (false, false, false),
            time - 1));
        current_skip.1 = true;
    }
    if !skip.2 && robots.0 < blueprint.max_ore() && resource.0 >= blueprint.ore {
        max_geode = max(max_geode, dfs(
            blueprint,
            (resource.0 + robots.0 - blueprint.ore, resource.1 + robots.1, resource.2 + robots.2),
            (robots.0 + 1, robots.1, robots.2),
            (false, false, false),
            time - 1));
        current_skip.2 = true;
    }
    max_geode = max(max_geode, dfs(
        blueprint, 
        (resource.0 + robots.0, resource.1 + robots.1, resource.2 + robots.2), 
        robots, 
        current_skip, 
        time - 1));
    max_geode
}

fn part2(blueprints: &Vec<Blueprint>) -> u16 {
    let answer1 = dfs(&blueprints[0], (0,0,0), (1,0,0), (false, false, false), 31);
    let answer2 = dfs(&blueprints[1], (0,0,0), (1,0,0), (false, false, false), 31);
    let answer3 = dfs(&blueprints[2], (0,0,0), (1,0,0), (false, false, false), 31);
    println!("{answer1} {answer2} {answer3}");
    answer1 * answer2 * answer3
}

// cargo test --bin day19 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, 33);
    }

    const INPUT: &str ="Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\n\
                        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
}