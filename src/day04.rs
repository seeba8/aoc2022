use std::str::FromStr;

use crate::util::read_input;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pair(Elf, Elf);
impl FromStr for Pair {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .split_once(',')
            .ok_or(color_eyre::eyre::eyre!("Cannot parse pair: '{s}'"))?;
        Ok(Self(x.parse()?, y.parse()?))
    }
}

impl Pair {
    pub const fn fully_contains(&self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }

    pub const fn overlap(&self) -> bool {
        self.0.contains(self.1.start) || self.1.contains(self.0.start)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Elf {
    pub start: usize,
    pub end: usize,
}

impl FromStr for Elf {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((Ok(start), Ok(end))) = s
            .trim()
            .split_once('-')
            .map(|(start, end)| (start.parse(), end.parse()))
        {
            Ok(Self { start, end })
        } else {
            Err(color_eyre::eyre::eyre!("Cannot parse elf {s}"))
        }
    }
}

impl Elf {
    pub const fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub const fn contains(&self, value: usize) -> bool {
        self.start <= value && value <= self.end
    }
}

pub fn solve() -> color_eyre::Result<()> {
    let input = read_input("day04.txt");
    println!("Day 04 part 1: {}", part1(&input)?);
    println!("Day 04 part 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> color_eyre::Result<usize> {
    let pairs: Vec<Pair> = input
        .lines()
        .map(Pair::from_str)
        .collect::<color_eyre::Result<Vec<_>>>()?;
    Ok(pairs.iter().filter(|pair| pair.fully_contains()).count())
}

fn part2(input: &str) -> color_eyre::Result<usize> {
    let pairs: Vec<Pair> = input
        .lines()
        .map(Pair::from_str)
        .collect::<color_eyre::Result<Vec<_>>>()?;
    Ok(pairs.iter().filter(|p| p.overlap()).count())
}

#[cfg(test)]
pub mod tests {

    use crate::util;

    use super::*;

    #[test]
    fn it_parses_elf() {
        assert_eq!(Elf { start: 2, end: 4 }, "2-4".parse().unwrap());
        assert_eq!(Elf { start: 2, end: 44 }, "2-44".parse().unwrap());
    }

    #[test]
    fn it_parses_pair() {
        assert_eq!(
            Pair(Elf { start: 2, end: 4 }, Elf { start: 6, end: 8 }),
            "2-4,6-8".parse().unwrap()
        );
    }

    #[test]
    fn it_finds_fully_contains() {
        let input = util::read_example("day04.txt");
        assert_eq!(2, part1(&input).unwrap());
    }

    #[test]
    fn it_finds_overlaps() {
        assert!(Pair::from_str("5-7,7-9").unwrap().overlap());
        assert!(Pair::from_str("2-8,3-7").unwrap().overlap());
        assert!(!Pair::from_str("2-4,6-8").unwrap().overlap());
    }

    #[test]
    fn it_finds_all_overlaps() {
        let input = util::read_example("day04.txt");
        assert_eq!(4, part2(&input).unwrap());
    }
}
