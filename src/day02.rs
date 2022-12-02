use std::{cmp::Ordering, str::FromStr};

use crate::util::read_input;

pub fn solve() -> anyhow::Result<()> {
    let input = read_input("day02.txt");
    println!("Day 02 part 1: {}", play_part1(&input));
    println!("Day 02 part 2: {}", play_part2(&input));
    Ok(())
}

pub fn play_part1(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        // the s in (s)elf is silent
        let (other, elf): (Shape, Shape) = line.trim().split_once(' ').map(|(other, elf)| (other.parse().unwrap(), elf.parse().unwrap())).unwrap();
        score += elf.play(&other);
    }
    score
}

pub fn play_part2(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        // the s in (s)elf is silent
        let (other, target): (Shape, Target) = line.trim().split_once(' ').map(|(other, target)| (other.parse().unwrap(), target.parse().unwrap())).unwrap();
        score += other.target(&target).play(&other);
    }
    score
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Lose,
    Draw,
    Win,
}

impl FromStr for Target{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Target::Lose),
            "Y" => Ok(Target::Draw),
            "Z" => Ok(Target::Win),
            x => Err(format!("Illegal target: '{x}'"))
        }
    }
}



impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissor),
            x => Err(format!("Illegal character: '{x}'"))
        }
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => Some(Ordering::Equal),
            (Shape::Rock, Shape::Paper) => Some(Ordering::Less),
            (Shape::Rock, Shape::Scissor) => Some(Ordering::Greater),
            (Shape::Paper, Shape::Rock) => Some(Ordering::Greater),
            (Shape::Paper, Shape::Paper) => Some(Ordering::Equal),
            (Shape::Paper, Shape::Scissor) => Some(Ordering::Less),
            (Shape::Scissor, Shape::Rock) => Some(Ordering::Less),
            (Shape::Scissor, Shape::Paper) => Some(Ordering::Greater),
            (Shape::Scissor, Shape::Scissor) => Some(Ordering::Equal),
        }
    }
}

impl Shape {

    pub fn value(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
    pub fn play(&self, other: &Self) -> usize {
        match self.cmp(other) {
            Ordering::Less => self.value(),
            Ordering::Equal => self.value() + 3,
            Ordering::Greater => self.value() + 6,
        }
    }

    pub fn target(&self, target: &Target) -> Shape {
        match (target, self) {
            (Target::Lose, Shape::Rock) => Shape::Scissor,
            (Target::Lose, Shape::Paper) => Shape::Rock,
            (Target::Lose, Shape::Scissor) => Shape::Paper,
            (Target::Draw, Shape::Rock) => Shape::Rock,
            (Target::Draw, Shape::Paper) => Shape::Paper,
            (Target::Draw, Shape::Scissor) => Shape::Scissor,
            (Target::Win, Shape::Rock) => Shape::Paper,
            (Target::Win, Shape::Paper) => Shape::Scissor,
            (Target::Win, Shape::Scissor) => Shape::Rock,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{day02::{Shape, play_part2}, util::read_example};

    use super::play_part1;

    #[test]
    fn it_finds_winner() {
        assert!(Shape::Rock > Shape::Scissor);
        assert!(Shape::Paper > Shape::Rock);
        assert_eq!(Shape::Paper, Shape::Paper);
    }

    #[test]
    fn it_scores_correctly() {
        assert_eq!(8, Shape::Paper.play(&Shape::Rock));
        assert_eq!(1, Shape::Rock.play(&Shape::Paper));
        assert_eq!(6, Shape::Scissor.play(&Shape::Scissor));
    }

    #[test]
    fn it_scores_example_part1() {
        assert_eq!(15, play_part1(&read_example("day02.txt")));
    }

    #[test]
    fn it_scores_example_part2() {
        assert_eq!(12, play_part2(&read_example("day02.txt")));
    }
}