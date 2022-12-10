use std::{collections::HashSet, str::FromStr};

use color_eyre::eyre;

use crate::util::{Direction, read_input};

pub fn solve() -> color_eyre::Result<()>{
    let input = read_input("day09.txt");
    println!("Day 09 part 1: {}", part1(&input)?);
    Ok(())
}

fn part1(input: &str) -> color_eyre::Result<usize> {
    let mut area = Area::new();
    let instructions: Vec<Instruction> = input
            .lines()
            .map(str::parse)
            .collect::<color_eyre::Result<Vec<_>>>()?;
        for instruction in &instructions {
            area.apply(instruction);
        }
    Ok(area.tail_visited.len())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Instruction {
    direction: Direction,
    distance: usize,
}

impl FromStr for Instruction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s
            .split_once(' ')
            .ok_or_else(|| eyre::eyre!("Cannot parse instruction '{s}"))?;
        let direction = direction.parse()?;
        let distance = distance.parse()?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Area {
    head: Point,
    tail: Point,
    tail_visited: HashSet<Point>,
}

impl Area {
    pub fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert(Point::default());
        Self {
            head: Point::default(),
            tail: Point::default(),
            tail_visited: visited,
        }
    }

    pub fn apply(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.distance {
            self.move_head(instruction.direction);
        }
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.head.y += 1,
            Direction::Right => self.head.x += 1,
            Direction::Down => self.head.y -= 1,
            Direction::Left => self.head.x -= 1,
        }
        self.follow_tail();
    }

    fn follow_tail(&mut self) {
        if self.head.x.abs_diff(self.tail.x) <= 1 && self.head.y.abs_diff(self.tail.y) <= 1 {
            // They are touching
            return;
        }

        if self.head.x == self.tail.x {
            // same column
            self.tail.y += (self.head.y - self.tail.y).signum();
        } else if self.head.y == self.tail.y {
            // same row
            self.tail.x += (self.head.x - self.tail.x).signum();
        } else {
            // diagonal
            self.tail.y += (self.head.y - self.tail.y).signum();
            self.tail.x += (self.head.x - self.tail.x).signum();
        }
        self.tail_visited.insert(self.tail);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    #[test]
    fn it_follows() -> color_eyre::Result<()> {
        
        let input = read_example("day09.txt");
        
        assert_eq!(13, part1(&input)?);
        Ok(())
    }
}
