use std::{cmp::Ordering, str::FromStr};

use crate::util::read_input;

pub fn solve(){
    let input = read_input("day02.txt");
    println!("Day 02 part 1: {}", play_part1(&input));
    println!("Day 02 part 2: {}", play_part2(&input));
}

pub fn play_part1(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        // the s in (s)elf is silent
        let (other, elf): (Shape, Shape) = line.trim().split_once(' ').map(|(other, elf)| (other.parse().unwrap(), elf.parse().unwrap())).unwrap();
        score += elf.play(other);
    }
    score
}

pub fn play_part2(input: &str) -> usize {
    let mut score = 0;
    for line in input.lines() {
        // the s in (s)elf is silent
        let (other, target): (Shape, Target) = line.trim().split_once(' ').map(|(other, target)| (other.parse().unwrap(), target.parse().unwrap())).unwrap();
        score += other.target(target).play(other);
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
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            x => Err(format!("Illegal target: '{x}'"))
        }
    }
}



impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            x => Err(format!("Illegal character: '{x}'"))
        }
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Rock) | (Self::Paper, Self::Paper) | (Self::Scissor, Self::Scissor) => Ordering::Equal,
            (Self::Rock, Self::Paper) | (Self::Paper, Self::Scissor) | (Self::Scissor, Self::Rock) => Ordering::Less,
            (Self::Rock, Self::Scissor) |  (Self::Paper, Self::Rock) | (Self::Scissor, Self::Paper)=> Ordering::Greater,
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Shape {

    pub const fn value(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }
    pub fn play(self, other: Self) -> usize {
        match self.cmp(&other) {
            Ordering::Less => self.value(),
            Ordering::Equal => self.value() + 3,
            Ordering::Greater => self.value() + 6,
        }
    }

    pub const fn target(self, target: Target) -> Self {
        match (target, self) {
            (Target::Lose, Self::Paper) | (Target::Draw, Self::Rock) | (Target::Win, Self::Scissor) => Self::Rock,
            (Target::Lose, Self::Scissor) | (Target::Draw, Self::Paper) | (Target::Win, Self::Rock) => Self::Paper,
            (Target::Lose, Self::Rock) | (Target::Draw, Self::Scissor) | (Target::Win, Self::Paper) => Self::Scissor,
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
        assert_eq!(8, Shape::Paper.play(Shape::Rock));
        assert_eq!(1, Shape::Rock.play(Shape::Paper));
        assert_eq!(6, Shape::Scissor.play(Shape::Scissor));
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