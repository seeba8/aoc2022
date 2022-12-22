use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    Finish, IResult,
};

use crate::util::read_input;

pub fn solve() {
    let input = read_input("day16.txt");
    let vulcano = Vulcano::parse(&input);
    println!("Day 16 part 1: {}", vulcano.get_best_pressure_release(30));
}

#[derive(Debug)]
pub struct Vulcano {
    valves: Vec<Valve>,
    tunnels: Vec<Tunnel>,
}
///
/// Idea: first get the shortest distance from/to every valve
/// Then multiply the distance * the flow rate as a measure of where to go
/// So basically, first do a minimizing Dijkstra
/// And then a maximising one
impl Vulcano {
    fn parse(input: &str) -> Self {
        let mut v = vec![];
        let mut tunnels: Vec<Tunnel> = vec![];
        for line in input.trim().lines() {
            let (valve, targets) = Valve::parse(line);
            v.push(valve);
            for target in &targets {
                tunnels.push(Tunnel {
                    from: valve.name,
                    to: *target,
                });
            }
        }
        Self { valves: v, tunnels }
    }

    fn get_distances(&self) -> HashMap<Tunnel, usize> {
        let mut distances: HashMap<Tunnel, usize> = HashMap::default();
        for tunnel in &self.tunnels {
            distances.insert(*tunnel, 1);
        }
        for valve in &self.valves {
            distances.insert(
                Tunnel {
                    from: valve.name,
                    to: valve.name,
                },
                0,
            );
        }
        for k in 0..self.valves.len() {
            for i in 0..self.valves.len() {
                for j in 0..self.valves.len() {
                    let ik = *distances
                        .get(&Tunnel {
                            from: self.valves[i].name,
                            to: self.valves[k].name,
                        })
                        .unwrap_or(&usize::MAX);
                    let kj = *distances
                        .get(&Tunnel {
                            from: self.valves[k].name,
                            to: self.valves[j].name,
                        })
                        .unwrap_or(&usize::MAX);
                    let ij = distances
                        .entry(Tunnel {
                            from: self.valves[i].name,
                            to: self.valves[j].name,
                        })
                        .or_insert(usize::MAX);
                    if *ij > ik.saturating_add(kj) {
                        *ij = ik + kj;
                    }
                }
            }
        }
        distances
    }

    fn get_best_pressure_release(&self, remaining_time: usize) -> usize {
        let mut best = 0;
        let dist = self.get_distances();
        self._get_best_pressure_release(
            &dist,
            *self
                .valves
                .iter()
                .find(|v| v.name == Name::from("AA"))
                .unwrap(),
            &mut HashSet::new(),
            remaining_time,
            0,
            &mut best,
        );
        best
    }

    fn _get_best_pressure_release(
        &self,
        dist: &HashMap<Tunnel, usize>,
        position: Valve,
        opened_valves: &mut HashSet<Valve>,
        remaining_time: usize,
        current: usize,
        best_flow: &mut usize,
    ) {
        if current > *best_flow {
            *best_flow = current;
        }
        if remaining_time == 0 {
            return;
        }
        let mut remaining_valves: Vec<Valve> = self
            .valves
            .iter()
            .filter(|v| {
                v.flow_rate > 0
                    && !opened_valves.contains(v)
                    && *dist.get(&Tunnel::new(position.name, v.name)).unwrap() < remaining_time
            })
            .copied()
            .collect();
        remaining_valves.sort_unstable_by_key(|v| {
            v.flow_rate as usize
                * (remaining_time - 1 - dist.get(&Tunnel::new(position.name, v.name)).unwrap())
        });
        while let Some(target) = remaining_valves.pop() {
            let remaining_time =
                remaining_time - 1 - dist.get(&Tunnel::new(position.name, target.name)).unwrap();
            let current = current + remaining_time * target.flow_rate as usize;
            opened_valves.insert(target);
            self._get_best_pressure_release(
                dist,
                target,
                opened_valves,
                remaining_time,
                current,
                best_flow,
            );
            opened_valves.remove(&target);
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub struct Name(([char; 2]));

impl From<(char, char)> for Name {
    fn from((a, b): (char, char)) -> Self {
        Self([a, b])
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        Self([chars.next().unwrap(), chars.next().unwrap()])
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Valve {
    name: Name,
    flow_rate: u8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Tunnel {
    from: Name,
    to: Name,
}

impl Tunnel {
    pub fn new(from: impl Into<Name>, to: impl Into<Name>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

impl Valve {
    fn parse(input: &str) -> (Self, Vec<Name>) {
        fn _name_parser(input: &str) -> IResult<&str, Name> {
            map(
                pair(
                    nom::character::complete::anychar,
                    nom::character::complete::anychar,
                ),
                Name::from,
            )(input)
        }

        fn _parse(input: &str) -> IResult<&str, (Name, u8, Vec<Name>)> {
            tuple((
                preceded(tag("Valve "), _name_parser),
                preceded(tag(" has flow rate="), nom::character::complete::u8),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), _name_parser),
                ),
            ))(input)
        }

        let (name, flow_rate, tunnels) = all_consuming(_parse)(input).finish().unwrap().1;
        (Self { name, flow_rate }, tunnels)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    #[test]
    fn it_parses_valve() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let (valve, tunnels) = Valve::parse(input);
        assert_eq!(
            Valve {
                name: "AA".into(),
                flow_rate: 0
            },
            valve
        );
        assert_eq!(vec![Name::from("DD"), "II".into(), "BB".into()], tunnels);
    }

    #[test]
    fn it_parses_vulcano() {
        let input = read_example("day16.txt");
        let vulcano = Vulcano::parse(&input);
        assert_eq!(10, vulcano.valves.len());
    }

    #[test]
    fn it_finds_shortest_paths() {
        let input = read_example("day16.txt");
        let vulcano = Vulcano::parse(&input);
        let distances = vulcano.get_distances();
        assert_eq!(*distances.get(&Tunnel::new("JJ", "EE")).unwrap(), 4);
        assert_eq!(*distances.get(&Tunnel::new("JJ", "HH")).unwrap(), 7);
    }

    #[test]
    fn it_finds_optimal_pressure_release() {
        let input = read_example("day16.txt");
        let vulcano = Vulcano::parse(&input);
        assert_eq!(1651, vulcano.get_best_pressure_release(30));
    }
}
