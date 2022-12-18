use std::{collections::HashMap, str::FromStr};

use crate::util::read_input;

pub fn solve() -> color_eyre::Result<()>{
    let input = read_input("day12.txt");
    println!("Day 12 part 1: {}", part1(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> color_eyre::Result<usize> {
    let graph = Graph::from_str(input)?;
    let (_, prev) = graph.dijkstra().ok_or_else(|| color_eyre::eyre::eyre!("Error applying dijkstra"))?;
    let mut target = &graph.end;
    let mut c = 0;
    while let Some(predecessor) = prev.get(target) {
        target = predecessor;
        c += 1;
    }
    Ok(c)
}

#[derive(Debug, Default)]
pub struct Graph {
    edges: HashMap<Node, Vec<Edge>>,
    start: Node,
    end: Node,
}

impl Graph {
    fn dijkstra(&self) -> Option<(HashMap<Node, isize>, HashMap<Node, Node>)> {
        let mut dist: HashMap<Node, isize> = HashMap::default();
        let mut prev: HashMap<Node, Node> = HashMap::default();
        let mut q: Vec<Node> = self.edges.keys().copied().collect();
        dist.insert(self.start, 0);
        while q.contains(&self.end) {
            // find vertex with minimum distance
            let (idx, u) = q
                .iter()
                .enumerate()
                .min_by_key(|(_, v)| dist.get(v).unwrap_or(&isize::MAX))?;
            // if u == &self.end { 
            //     return Some((dist, prev));
            // }
            let dist_to_u = *dist.get(u)?;
            let u = q.remove(idx);
            for edge in self.edges.get(&u)?.iter().filter(|e| q.contains(&e.target)) {
                let alt = dist_to_u + edge.cost;
                if alt < *dist.get(&edge.target).unwrap_or(&isize::MAX) {
                    dist.insert(edge.target, alt);
                    prev.insert(edge.target, u);
                }
            }
        }
        Some((dist, prev))
    }
}

impl FromStr for Graph {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<Vec<char>> = s
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let mut graph = Self::default();
        let width = chars[0].len();
        let get_value = |c: char| match c{
            'S' => 'a',
            'E' => 'z',
            e => e,
        };
        let add_edge = |(value, nx, ny, edges): (char, usize, usize, &mut Vec<Edge>)| {
            let neighbour = get_value(chars[ny][nx]);
            let difference = (neighbour as i16) - (value as i16); // we can go downwards as far as we want, but upwards only 1 step
            if difference < 2 {
                edges.push(Edge {
                    cost: 26 - difference as isize,
                    target: Node {
                        index: nx + ny * width,
                        value: neighbour,
                    },
                });
            }
        };

        for y in 0..chars.len() {
            for x in 0..width {
                let value = get_value( chars[y][x] );
                let node = Node {
                    index: x + y * width,
                    value,
                };
                if chars[y][x] == 'S' {
                    graph.start = node;
                }
                if chars[y][x] == 'E' {
                    graph.end = node;
                }

                let mut edges: Vec<Edge> = Vec::default();
                

                if x > 0 {
                    add_edge((value, x - 1, y, &mut edges));
                }
                if x < width - 1 {
                    add_edge((value, x + 1, y, &mut edges));
                }
                if y > 0 {
                    add_edge((value, x, y - 1, &mut edges));
                }
                if y < chars.len() - 1 {
                    add_edge((value, x, y + 1, &mut edges));
                }

                graph.edges.insert(node, edges);
            }
        }
        Ok(graph)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Node {
    index: usize,
    value: char,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Edge {
    cost: isize,
    target: Node,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;
    use super::*;
    #[test]
    fn it_parses_graph() {
        let input = read_example("day12.txt");
        let graph = Graph::from_str(&input).unwrap();
        assert_eq!(Node { index: 0, value: 'a' }, graph.start);
        assert_eq!(Node { index: 21, value: 'z' }, graph.end);
        assert_eq!(2, graph.edges.get(&graph.start).unwrap().len());
        assert_eq!(&vec![Edge {cost: 27, target: Node {index: 1, value: 'a'}}, Edge { cost: 25, target: Node { index: 10, value: 'c' }}], graph.edges.get(&Node { index: 2, value: 'b'}).unwrap());
    }

    #[test]
    fn it_finds_path() {
        let input = read_example("day12.txt");
        
        assert_eq!(31, part1(&input).unwrap());
    }
}