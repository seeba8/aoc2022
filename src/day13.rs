use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, opt},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
    Finish, IResult,
};

use crate::util::read_input;

pub fn solve() {
    let input = read_input("day13.txt");
    println!("Day 13 part 1: {}", part1(&input));
    println!("Day 13 part 2: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    let pairs = all_consuming(Packet::parse_input)(input.trim())
        .finish()
        .unwrap()
        .1;
    pairs
        .iter()
        .enumerate()
        .map(|(index, (a, b))| if a < b { index + 1 } else { 0 })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let pairs: Vec<_> = all_consuming(Packet::parse_input)(input.trim())
        .finish()
        .unwrap()
        .1;
    let mut packets: Vec<_> = pairs.iter().map(|(a, _)| a).cloned().collect();
    let second_part: Vec<_> = pairs.iter().map(|(_, a)| a).cloned().collect();
    let markers = Packet::parse_pair(
        "[[2]]
[[6]]",
    )
    .unwrap()
    .1;
    packets.extend(second_part);
    packets.push(markers.0.clone());
    packets.push(markers.1.clone());
    packets.sort_unstable();
    (packets.binary_search(&markers.0).unwrap() + 1)
        * (packets.binary_search(&markers.1).unwrap() + 1)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u8),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Number(first), Self::Number(second)) => first.cmp(second),
            (Self::List(first), Self::List(second)) => {
                let mut i = 0;
                loop {
                    match (first.get(i), second.get(i)) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(first), Some(second)) => {
                            let res = first.cmp(second);
                            if res != Ordering::Equal {
                                return res;
                            }
                        }
                    }
                    i += 1;
                }
            }
            (Self::List(first), Self::Number(second)) => {
                let second = vec![Self::Number(*second)];
                first.cmp(&second)
            }
            (Self::Number(first), Self::List(second)) => {
                let first = vec![Self::Number(*first)];
                first.cmp(second)
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        fn parse_list(input: &str) -> IResult<&str, Packet> {
            delimited(
                tag("["),
                map(
                    separated_list0(tag(","), alt((parse_list, parse_integer))),
                    Packet::List,
                ),
                tag("]"),
            )(input)
        }

        fn parse_integer(input: &str) -> IResult<&str, Packet> {
            map(nom::character::complete::u8, Packet::Number)(input)
        }

        parse_list(input)
    }

    pub fn parse_pair(input: &str) -> IResult<&str, (Self, Self)> {
        pair(
            Self::parse,
            preceded(tuple((opt(tag("\r")), tag("\n"))), Self::parse),
        )(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, Vec<(Self, Self)>> {
        separated_list1(many1(tuple((opt(tag("\r")), tag("\n")))), Self::parse_pair)(input)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    use nom::{combinator::all_consuming, Finish};

    #[test]
    fn it_parses_single_item_list() {
        let list = all_consuming(Packet::parse)("[1]").finish().unwrap().1;
        assert_eq!(Packet::List(vec![Packet::Number(1),]), list);
    }

    #[test]
    fn it_parses_flat_list() {
        let list = all_consuming(Packet::parse)("[1,1,3,1,1]")
            .finish()
            .unwrap()
            .1;
        assert_eq!(
            Packet::List(vec![
                Packet::Number(1),
                Packet::Number(1),
                Packet::Number(3),
                Packet::Number(1),
                Packet::Number(1)
            ]),
            list
        );
    }
    #[test]
    fn it_parses_nested_list() {
        let list = all_consuming(Packet::parse)("[[1],4]").finish().unwrap().1;
        assert_eq!(
            Packet::List(vec![
                Packet::List(vec![Packet::Number(1)]),
                Packet::Number(4)
            ]),
            list
        );
        let list = all_consuming(Packet::parse)("[[8,7,6]]")
            .finish()
            .unwrap()
            .1;
        assert_eq!(
            Packet::List(vec![Packet::List(vec![
                Packet::Number(8),
                Packet::Number(7),
                Packet::Number(6)
            ]),]),
            list
        );
    }
    #[test]
    fn it_parses_empty_list() {
        let list = all_consuming(Packet::parse)("[]").finish().unwrap().1;
        assert_eq!(Packet::List(vec![]), list);
    }

    #[test]
    fn it_parses_pair() {
        let list = all_consuming(Packet::parse_pair)(
            "[]
[3]",
        )
        .finish()
        .unwrap()
        .1;
        assert_eq!(
            (Packet::List(vec![]), Packet::List(vec![Packet::Number(3)])),
            list
        );
    }

    #[test]
    fn it_compares_pairs() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]";
        let list = all_consuming(Packet::parse_pair)(input).finish().unwrap().1;
        assert!(list.0 < list.1);
    }
    #[test]
    fn it_solves_part1() {
        let input = read_example("day13.txt");
        assert_eq!(13, part1(&input));
    }

    #[test]
    fn it_solves_part2() {
        let input = read_example("day13.txt");
        assert_eq!(140, part2(&input));
    }
}
