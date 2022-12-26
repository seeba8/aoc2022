use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use itertools::Itertools;
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
    println!(
        "Day 16 part 1: {}",
        vulcano.get_best_pressure_release(1, 30)
    );
    println!(
        "Day 16 part 2: {}",
        vulcano.get_best_pressure_release(2, 26)
    );
}

#[derive(Debug)]
pub struct Vulcano {
    valves: Vec<Valve>,
    tunnels: Vec<Tunnel>,
    active_valves: Vec<Valve>,
}
///
/// Idea: first get the shortest distance from/to every valve
/// Then multiply the distance * the flow rate as a measure of where to go
/// So basically, first do a minimizing Dijkstra
/// And then a maximising one
impl Vulcano {
    fn parse(input: &str) -> Self {
        let mut valves = Vec::with_capacity(input.lines().count());
        let mut tunnels: Vec<Tunnel> = vec![];
        let mut active_valves = Vec::with_capacity(input.lines().count());
        for line in input.trim().lines() {
            let (valve, targets) = Valve::parse(line);
            valves.push(valve);
            if valve.flow_rate > 0 {
                active_valves.push(valve);
            }
            for target in &targets {
                tunnels.push(Tunnel {
                    from: valve.name,
                    to: *target,
                });
            }
        }

        Self {
            valves,
            tunnels,
            active_valves,
        }
    }

    fn get_distances(&self) -> HashMap<Tunnel, (usize, Name)> {
        let mut distances: HashMap<Tunnel, (usize, Name)> = HashMap::default();
        for tunnel in &self.tunnels {
            distances.insert(*tunnel, (1, tunnel.to));
        }
        for valve in &self.valves {
            distances.insert(
                Tunnel {
                    from: valve.name,
                    to: valve.name,
                },
                (0, valve.name),
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
                        .unwrap_or(&(usize::MAX, (' ', ' ').into()));
                    let kj = *distances
                        .get(&Tunnel {
                            from: self.valves[k].name,
                            to: self.valves[j].name,
                        })
                        .unwrap_or(&(usize::MAX, (' ', ' ').into()));
                    let ij = distances
                        .entry(Tunnel {
                            from: self.valves[i].name,
                            to: self.valves[j].name,
                        })
                        .or_insert((usize::MAX, (' ', ' ').into()));
                    if ij.0 > ik.0.saturating_add(kj.0) {
                        ij.0 = ik.0 + kj.0;
                        ij.1 = ik.1;
                    }
                }
            }
        }
        distances
    }

    fn get_best_pressure_release(&self, num_actors: usize, remaining_time: usize) -> usize {
        let mut best = 0;
        let dist = self.get_distances();
        let start: Valve = *self
            .valves
            .iter()
            .find(|v| v.name == ('A', 'A').into())
            .unwrap();
        self._get_best_pressure_release_with_elephant(
            &dist,
            &mut vec![
                Actor {
                    next_action: remaining_time,
                    position: start,
                };
                num_actors
            ],
            &mut HashMap::default(),
            remaining_time,
            &mut best,
        );
        best
    }
    #[allow(clippy::too_many_arguments)]
    fn _get_best_pressure_release_with_elephant(
        &self,
        dist: &HashMap<Tunnel, (usize, Name)>,
        actors: &mut [Actor],
        opened_valves: &mut HashMap<Valve, usize>,
        remaining_time: usize,
        best_flow: &mut usize,
    ) {
        // dbg!(&actors);
        if actors.len() > 2 {
            unimplemented!()
        }
        let sum = opened_valves
        .iter()
        .map(|(valve, ticks)| valve.flow_rate as usize * ticks)
        .sum();
        if remaining_time == 0 {
            if sum > *best_flow {
                *best_flow = sum;
                dbg!(&opened_valves);
                dbg!(best_flow);
            }
            return;
        }
        // Can we even reach the best value still?
        if sum + self.active_valves.iter().map(|v| v.flow_rate as usize * remaining_time).sum::<usize>() < *best_flow {
            return;
        }
        let new_valves: Vec<Valve> = actors
            .iter()
            .filter(|actor| actor.next_action == remaining_time && actor.position.flow_rate > 0)
            .map(|actor| actor.position)
            .collect();
        for v in &new_valves {
            opened_valves.insert(*v, remaining_time);
        }
        let next_actions: Vec<_> = actors
            .iter()
            .map(|actor| {
                if actor.next_action == remaining_time {
                    self._get_actions(dist, &actor.position, opened_valves, remaining_time)
                } else {
                    vec![*actor]
                }
            })
            .collect();
        // dbg!(&opened_valves);
        if actors.len() == 1 {
            for actors_new in &next_actions[0] {
                // dbg!(&actors_new);
                // dbg!(&next_iteration);
                self._get_best_pressure_release_with_elephant(
                    dist,
                    &mut [*actors_new],
                    opened_valves,
                    actors_new.next_action,
                    best_flow,
                );
            }
        } else {
            for mut actors_new in next_actions.into_iter().multi_cartesian_product() {
                if actors_new.iter().map(|actor| actor.position).all_equal()
                    && actors_new[0].position.flow_rate != 0
                {
                    continue;
                }
                // dbg!(&actors_new);
                let next_iteration = actors_new
                    .iter()
                    .map(|actor| actor.next_action)
                    .max()
                    .unwrap();
                // dbg!(&next_iteration);
                self._get_best_pressure_release_with_elephant(
                    dist,
                    &mut actors_new,
                    opened_valves,
                    next_iteration,
                    best_flow,
                );
            }
        }

        for v in &new_valves {
            opened_valves.remove(v);
        }
    }

    fn _get_actions(
        &self,
        dist: &HashMap<Tunnel, (usize, Name)>,
        current_position: &Valve,
        opened_valves: &HashMap<Valve, usize>,
        remaining_time: usize,
    ) -> Vec<Actor> {
        let mut remaining_valves: Vec<_> = self
            .active_valves
            .iter()
            .filter(|v| {
                !opened_valves.contains_key(v)
                && dist.get(&Tunnel::new(current_position.name, v.name)).unwrap().0 < remaining_time // ???
            })
            .copied()
            .map(|v| Actor {
                next_action: remaining_time.saturating_sub(
                    1_usize
                        + dist
                            .get(&Tunnel::new(current_position.name, v.name))
                            .unwrap()
                            .0,
                ),
                position: v,
            })
            .collect();
        remaining_valves.push(Actor {
            position: *self
                .valves
                .iter()
                .find(|v| v.name == ('A', 'A').into())
                .unwrap(),
            next_action: 0,
        });
        remaining_valves
            .sort_unstable_by_key(|v| usize::MAX - (v.position.flow_rate as usize * v.next_action));
        remaining_valves
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub struct Name(([char; 2]));

impl From<(char, char)> for Name {
    fn from((a, b): (char, char)) -> Self {
        Self([a, b])
    }
}

impl TryFrom<&str> for Name {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        Ok(Self([
            chars
                .next()
                .ok_or_else(|| color_eyre::eyre::eyre!("Cannot get first character of name"))?,
            chars
                .next()
                .ok_or_else(|| color_eyre::eyre::eyre!("Cannot get second character of name"))?,
        ]))
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
    flow_rate: u16,
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
        (
            Self {
                name,
                flow_rate: u16::from(flow_rate),
            },
            tunnels,
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Actor {
    next_action: usize,
    position: Valve,
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
                name: ('A', 'A').into(),
                flow_rate: 0
            },
            valve
        );
        assert_eq!(
            vec![
                Name::try_from("DD").unwrap(),
                "II".try_into().unwrap(),
                "BB".try_into().unwrap()
            ],
            tunnels
        );
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
        assert_eq!(
            distances
                .get(&Tunnel::new(('J', 'J'), ('E', 'E')))
                .unwrap()
                .0,
            4
        );
        assert_eq!(
            distances
                .get(&Tunnel::new(('J', 'J'), ('H', 'H')))
                .unwrap()
                .0,
            7
        );
    }

    #[test]
    fn it_finds_optimal_pressure_release() {
        let input = read_example("day16.txt");
        let vulcano = Vulcano::parse(&input);
        assert_eq!(1651, vulcano.get_best_pressure_release(1, 30));
    }

    #[test]
    fn it_gets_next_step_for_shortest_path() {
        let input = read_example("day16.txt");
        let vulcano = Vulcano::parse(&input);
        let distances = vulcano.get_distances();
        assert_eq!(
            distances.get(&Tunnel::new(('J', 'J'), ('E', 'E'))).unwrap(),
            &(4, ('I', 'I').into())
        );
        assert_eq!(
            distances.get(&Tunnel::new(('J', 'J'), ('H', 'H'))).unwrap(),
            &(7, ('I', 'I').into())
        );
    }

    #[test]
    fn it_finds_optimal_pressure_release_with_elephant() {
        let input = read_example("day16.txt");
        let vulcano = Vulcano::parse(&input);
        assert_eq!(1707, vulcano.get_best_pressure_release(2, 26));
    }
}
