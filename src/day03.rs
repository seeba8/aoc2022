use std::{collections::HashSet, str::FromStr};

use crate::util::read_input;

#[derive(Clone, Debug)]
pub struct Rucksack {
    compartments: [HashSet<char>; 2],
}

impl FromStr for Rucksack {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.trim().split_at(s.len() / 2);
        Ok(Self {
            compartments: [left.chars().collect(), right.chars().collect()],
        })
    }
}

impl Rucksack {
    pub fn get_duplicates(&self) -> Vec<&char> {
        self.compartments[0]
            .intersection(&self.compartments[1])
            .collect()
    }
    pub fn get_duplicates_priority(&self) -> usize {
        self.get_duplicates()
            .into_iter()
            .map(|c| priority(*c))
            .sum()
    }

    pub fn get_content(&self) -> HashSet<&char> {
        self.compartments[0].union(&self.compartments[1]).collect()
    }

    pub fn get_common_items(bags: &[Self]) -> HashSet<&char> {
        let mut common: Option<HashSet<_>> = None;
        for o in bags {
            common = common.map_or_else(
                || Some(o.get_content()),
                |c| Some(c.intersection(&o.get_content()).copied().collect()),
            );
        }
        common.unwrap_or_default()
    }
}

const fn priority(c: char) -> usize {
    if c.is_ascii_uppercase() {
        c as usize - 64 + 26
    } else {
        c as usize - 96
    }
}

pub fn solve() -> color_eyre::Result<()> {
    let input = read_input("day03.txt");
    println!("Day 03 part 1: {}", part1(&input)?);
    println!("Day 03 part 2: {}", part2(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> color_eyre::Result<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let rucksack: Rucksack = line.parse()?;
        sum += rucksack.get_duplicates_priority();
    }
    Ok(sum)
}

pub fn part2(input: &str) -> color_eyre::Result<usize> {
    let mut lines = input.lines();
    let mut sum = 0;
    while let Ok(bags) = lines.next_chunk::<3>() {
        sum += Rucksack::get_common_items(
            &bags
                .iter()
                .map(|b| Rucksack::from_str(b))
                .collect::<color_eyre::Result<Vec<_>>>()?,
        )
        .iter()
        .map(|x| priority(**x))
        .sum::<usize>();
    }
    Ok(sum)
}

#[cfg(test)]
pub mod tests {
    use std::str::FromStr;

    use crate::{
        day03::{part1, part2},
        util::read_example,
    };

    use super::Rucksack;

    #[test]
    fn it_finds_duplicate() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack: Rucksack = input.parse().unwrap();
        assert_eq!(vec![&'p'], rucksack.get_duplicates());
    }

    #[test]
    fn it_prioritises_duplicate() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack: Rucksack = input.parse().unwrap();
        assert_eq!(16, rucksack.get_duplicates_priority());

        let input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rucksack: Rucksack = input.parse().unwrap();
        assert_eq!(38, rucksack.get_duplicates_priority());

        let input = "PmmdzqPrVvPwwTWBwg";
        let rucksack: Rucksack = input.parse().unwrap();
        assert_eq!(42, rucksack.get_duplicates_priority());

        let input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let rucksack: Rucksack = input.parse().unwrap();
        assert_eq!(22, rucksack.get_duplicates_priority());
    }

    #[test]
    fn it_solves_part1() -> color_eyre::Result<()> {
        let input = read_example("day03.txt");
        assert_eq!(157, part1(&input)?);
        Ok(())
    }

    #[test]
    fn it_finds_common_items() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg";
        let rucksacks: Vec<_> = input
            .lines()
            .map(|line| Rucksack::from_str(line).unwrap())
            .collect();
        assert_eq!(
            vec![&&'r'],
            Rucksack::get_common_items(&rucksacks)
                .iter()
                .collect::<Vec<_>>()
        );

        let input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";
        let rucksacks: Vec<_> = input
            .lines()
            .map(|line| Rucksack::from_str(line).unwrap())
            .collect();
        assert_eq!(
            vec![&&'Z'],
            Rucksack::get_common_items(&rucksacks)
                .iter()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_solves_part2() -> color_eyre::Result<()> {
        let input = read_example("day03.txt");
        assert_eq!(70, part2(&input)?);
        Ok(())
    }
}
