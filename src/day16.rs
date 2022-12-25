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

    fn get_best_pressure_release_with_elephant(&self, remaining_time: usize) -> usize {
        let mut best = 0;
        let dist = self.get_distances();
        let start: Name = ('A', 'A').into();
        self._get_best_pressure_release_with_elephant(
            &dist,
            (start, Action::Open(start)),
            (start, Action::Open(start)),
            &mut HashSet::default(),
            remaining_time,
            0,
            &mut best,
        );
        best
    }
    #[allow(clippy::too_many_arguments)]
    fn _get_best_pressure_release_with_elephant(
        &self,
        dist: &HashMap<Tunnel, (usize, Name)>,
        me: (Name, Action),
        elephant: (Name, Action),
        opened_valves: &mut HashSet<Valve>,
        remaining_time: usize,
        current: usize,
        best_flow: &mut usize,
    ) {
        
        if remaining_time == 0 {
            return;
        }
        if let Action::Open(name) = me.1 {
            opened_valves.insert(*self.valves.iter().find(|v| v.name == name).unwrap());
        }
        if let Action::Open(name) = elephant.1 {
            opened_valves.insert(*self.valves.iter().find(|v| v.name == name).unwrap());
        }
        // dbg!(&opened_valves);
        let current = current
            + opened_valves
                .iter()
                .map(|v| v.flow_rate as usize)
                .sum::<usize>();
        if current > *best_flow {
            *best_flow = current;
        }
       
        // if let Action::WalkTowards(name) = me.1 && opened_valves.contains(self.valves.iter().find(|v| v.name == name).unwrap()) {
        //     return;
        // }
        // if let Action::WalkTowards(name) = elephant.1 && opened_valves.contains(self.valves.iter().find(|v| v.name == name).unwrap()) {
        //     return;
        // }

        
        dbg!(&current);
        let me_actions = self._get_actions(dist, me, opened_valves, remaining_time);
        dbg!(&me_actions);
        let elephant_actions = self._get_actions(dist, elephant, opened_valves, remaining_time);
        dbg!(&elephant_actions);
        if me_actions.is_empty() && elephant_actions.is_empty() {
            dbg!(&remaining_time);
            let current = current + (remaining_time - 1) * opened_valves.iter().map(|v|v.flow_rate as usize).sum::<usize>();
            if current > *best_flow {
                *best_flow = current;
            }
            return;
        }
        for me_action in &me_actions {
            for elephant_action in &elephant_actions {
                if me_action.1 == elephant_action.1 && me_action.1 != Action::Noop {
                    continue;
                }
                dbg!(me_action);
                dbg!(elephant_action);
                self._get_best_pressure_release_with_elephant(
                    dist,
                    *me_action,
                    *elephant_action,
                    opened_valves,
                    remaining_time - 1,
                    current,
                    best_flow,
                );
                if let Action::Open(name) = me_action.1 {
                    opened_valves.remove(self.valves.iter().find(|v| v.name == name).unwrap());
                }
                if let Action::Open(name) = elephant_action.1 {
                    opened_valves.remove(self.valves.iter().find(|v| v.name == name).unwrap());
                }
            }
        }
    }

    fn _get_actions(
        &self,
        dist: &HashMap<Tunnel, (usize, Name)>,
        person: (Name, Action),
        opened_valves: &HashSet<Valve>,
        remaining_time: usize,
    ) -> Vec<(Name, Action)> {
        #[allow(clippy::single_match_else)]
        match person.1 {
            Action::WalkTowards(target) => {
                if person.0 == target {
                    vec![(person.0, Action::Open(target))]
                } else {
                    let path_to_target = dist.get(&Tunnel::new(person.0, target)).unwrap().1;
                    vec![(path_to_target, Action::WalkTowards(target))]
                }
            }
            _ => {
                let position = person.0;
                let mut remaining_valves: Vec<Valve> = self
                    .valves
                    .iter()
                    .filter(|v| {
                        v.flow_rate > 0
                            && !opened_valves.contains(v)
                            && dist.get(&Tunnel::new(position, v.name)).unwrap().0 < remaining_time
                    })
                    .copied()
                    .collect();
                remaining_valves.sort_unstable_by_key(|v| {
                    v.flow_rate as usize
                        * (remaining_time - 1 - dist.get(&Tunnel::new(position, v.name)).unwrap().0)
                });
                let mut return_values = vec![];
                for remaining_valve in remaining_valves.into_iter().rev() {
                    let first_step = dist
                        .get(&Tunnel::new(position, remaining_valve.name))
                        .unwrap()
                        .1;
                    return_values.push((first_step, Action::WalkTowards(remaining_valve.name)));
                }
                if return_values.is_empty() {
                    return_values.push((person.0, Action::Noop));
                }
                return_values
            }
        }
    }

    fn get_best_pressure_release(&self, remaining_time: usize) -> usize {
        let mut best = 0;
        let dist = self.get_distances();
        self._get_best_pressure_release(
            &dist,
            *self
                .valves
                .iter()
                .find(|v| v.name == Name::from(('A', 'A')))
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
        dist: &HashMap<Tunnel, (usize, Name)>,
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
                    && dist.get(&Tunnel::new(position.name, v.name)).unwrap().0 < remaining_time
            })
            .copied()
            .collect();
        remaining_valves.sort_unstable_by_key(|v| {
            v.flow_rate as usize
                * (remaining_time - 1 - dist.get(&Tunnel::new(position.name, v.name)).unwrap().0)
        });
        while let Some(target) = remaining_valves.pop() {
            let remaining_time = remaining_time
                - 1
                - dist
                    .get(&Tunnel::new(position.name, target.name))
                    .unwrap()
                    .0;
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Action {
    WalkTowards(Name),
    Open(Name),
    Noop,
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
        assert_eq!(1651, vulcano.get_best_pressure_release(30));
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
        assert_eq!(1707, vulcano.get_best_pressure_release_with_elephant(26));
    }
}
