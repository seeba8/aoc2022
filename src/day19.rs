use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

use crate::util::read_input;

pub fn solve() {
    let input = read_input("day19.txt");
    let blueprints = parse_blueprints(&input);
    let mut sum: usize = 0;
    for (idx, blueprint) in blueprints.iter().enumerate() {
        dbg!(idx);
        sum += Swarm::new().max_geodes(24, blueprint) as usize * (idx + 1);
    }
    println!("Day 19 part 1: {sum}");
}

type Robot = u8;

type Blueprint = [[Robot; 4]; 4];

pub fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(
        r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.",
    ).unwrap();
    let mut blueprints: Vec<Blueprint> = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
            let c: Vec<Robot> = captures
                .iter()
                .skip(1)
                .filter_map(|v| v.and_then(|v| v.as_str().parse().ok()))
                .collect_vec();
            blueprints.push([
                [c[1], 0, 0, 0],
                [c[2], 0, 0, 0],
                [c[3], c[4], 0, 0],
                [c[5], 0, c[6], 0],
            ]);
        }
    }
    blueprints
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Swarm {
    robots: [Robot; 4],
    resources: [Robot; 4], // 4 since geodes are considered resources, here
}

impl Swarm {
    pub fn new() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
        }
    }
    fn collect(&mut self) {
        for (idx, count) in self.robots.iter().enumerate() {
            self.resources[idx] += count;
        }
    }

    pub fn tick(mut self, blueprint: &Blueprint) -> Vec<Swarm> {
        let mut swarms = vec![];
        for (robot_type, &r) in blueprint.iter().enumerate() {
            if self.resources.iter().zip(r).all(|(l, r)| *l >= r) {
                let mut fork = self.clone();

                for (i, cost) in r.iter().enumerate() {
                    fork.resources[i] -= cost;
                }
                fork.collect();
                fork.robots[robot_type] += 1;
                swarms.push(fork);
            }
        }
        self.collect();

        swarms.push(self);
        return swarms;
    }

    #[allow(unused)]
    fn get_projected_resources(&self, remaining_ticks: Robot) -> [Robot; 4] {
        let mut projected_resources = self.resources.clone();
        for i in 0..4 {
            projected_resources[i] += self.robots[i] * remaining_ticks;
        }
        return projected_resources;
    }

    pub fn max_geodes(self, minutes: Robot, blueprint: &Blueprint) -> Robot {
        let mut swarms: HashSet<Swarm> = HashSet::new();
        swarms.insert(self);
        for tick_number in 0..minutes {
            dbg!(tick_number);
            // get swarm with most geode-producing robots.
            let most_geode_robots = swarms.iter().map(|s| s.robots[3]).max().unwrap();

            let most_geodes = swarms.iter().map(|s| s.resources[3]).max().unwrap();
            // let all_projected_resources: Vec<_> = swarms
            //     .iter()
            //     .map(|swarm| swarm.get_projected_resources(minutes - tick_number))
            //     .collect();
            // let most_robots = swarms
            //     .iter()
            //     .map(|s| s.robots.iter().sum::<u8>())
            //     .max()
            //     .unwrap();
            let mut new_swarms: HashSet<Swarm> = HashSet::new();
            for s in swarms.into_iter() {
                // trim strictly worse swarms
                if s.robots[3] < most_geode_robots {
                    continue;
                }

                if s.resources[3] < most_geodes {
                    continue;
                    // can we do this?
                }
                /*if most_robots > 1 && s.robots.iter().sum::<u8>() == 1 {
                    continue;
                }
                if tick_number > minutes / 2 {
                    if s.robots[1] == 0 && s.robots[2] == 0 && s.robots[3] == 0 {
                        continue;
                    }
                }*/
                // let projected_resources = s.get_projected_resources(minutes - tick_number);
                // if all_projected_resources
                //     .iter()
                //     .any(|r| r.iter().zip(projected_resources).all(|(l, r)| *l > r))
                // {
                //     continue;
                // }
                // tick the swarm
                new_swarms.extend(s.tick(blueprint));
            }
            swarms = new_swarms;
            dbg!(swarms.len());
            dbg!(swarms.iter().map(|s| s.resources[3]).max());
        }
        return swarms.iter().map(|s| s.resources[3]).max().unwrap();
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    #[test]
    fn it_collects() {
        let mut swarm = Swarm {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
        };
        swarm.collect();
        assert_eq!([1, 0, 0, 0], swarm.resources);
        swarm.collect();
        assert_eq!([2, 0, 0, 0], swarm.resources);
    }

    #[test]
    fn it_parses_input() {
        let input = read_example("day19.txt");
        let blueprints = parse_blueprints(&input);
        assert_eq!(
            blueprints[0],
            [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]]
        );
        assert_eq!(
            blueprints[1],
            [[2, 0, 0, 0], [3, 0, 0, 0], [3, 8, 0, 0], [3, 0, 12, 0]]
        );
    }

    #[test]
    fn it_gets_max_geodes() {
        let swarm = Swarm {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
        };
        let blueprint: Blueprint = [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]];
        assert_eq!(9, swarm.max_geodes(24, &blueprint));
        let swarm = Swarm {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
        };
        let blueprint: Blueprint = [[2, 0, 0, 0], [3, 0, 0, 0], [3, 8, 0, 0], [3, 0, 12, 0]];
        assert_eq!(12, swarm.max_geodes(24, &blueprint));
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn it_solves_part1() {
        let input = read_input("day19.txt");
        let blueprints = parse_blueprints(&input);
        let mut sum = 0;
        for (idx, blueprint) in blueprints.iter().enumerate() {
            dbg!(idx);
            sum += Swarm::new().max_geodes(24, blueprint) as usize * (idx + 1);
        }
        assert_eq!(1404, sum);
    }
}
