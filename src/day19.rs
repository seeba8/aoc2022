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
    let mut prod = 1;
    for (idx, blueprint) in blueprints.iter().take(3).enumerate() {
        dbg!(idx);
        prod *= Swarm::new().max_geodes(32, blueprint) as usize;
    }
    println!("Day 19 part 2: {prod}");
}

type Robot = u16;

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
    pub const fn new() -> Self {
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

    pub fn tick(mut self, blueprint: &Blueprint) -> Vec<Self> {
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
        swarms
    }

    #[allow(unused)]
    fn get_projected_resources(&self, remaining_ticks: Robot) -> [Robot; 4] {
        let mut projected_resources = self.resources;
        for (i, res) in projected_resources.iter_mut().enumerate() {
            *res += self.robots[i] * remaining_ticks;
        }
        projected_resources
    }

    pub fn max_geodes(self, minutes: Robot, blueprint: &Blueprint) -> Robot {
        let mut swarms: HashSet<Self> = HashSet::new();
        swarms.insert(self);
        for _ in 0..minutes {
            // dbg!(tick_number);

            let most_robots = swarms.iter().fold([0,0,0,0], |acc, s|  {
                let mut acc = acc;
                for (i, x) in acc.iter_mut().enumerate() {
                    *x = (*x).max(s.robots[i]);
                }
                acc
            });
            let mut new_swarms: HashSet<Self> = HashSet::new();
            for s in swarms {
                let mut is_worse = true;
                for (i, r) in most_robots.iter().enumerate() {
                    if s.robots[i] >= *r {
                        is_worse = false;
                    }
                }
                if is_worse {
                    continue;
                }
                // tick the swarm
                new_swarms.extend(s.tick(blueprint));
            }
            swarms = new_swarms;
            // dbg!(swarms.len());
            // dbg!(swarms.iter().map(|s| s.resources[3]).max());
        }
        // let best_swarm = swarms.iter().max_by(|a, b| a.resources[3].cmp(&b.resources[3])).unwrap();
        // dbg!(best_swarm);
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

    #[test]
    fn it_can_do_32_minutes() {
        let swarm = Swarm {
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
        };
        let blueprint: Blueprint = [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]];
        assert_eq!(56, swarm.max_geodes(32, &blueprint));
    }
}
