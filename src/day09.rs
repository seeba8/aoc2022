use std::{collections::HashSet, str::FromStr};

use color_eyre::eyre;

use crate::util::{Direction, read_input};

pub fn solve() -> color_eyre::Result<()>{
    let input = read_input("day09.txt");
    println!("Day 09 part 1: {}", part1(&input)?);
    println!("Day 09 part 1: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> color_eyre::Result<usize> {
    let mut area = Area::new(2);
    let instructions: Vec<Instruction> = input
            .lines()
            .map(str::parse)
            .collect::<color_eyre::Result<Vec<_>>>()?;
        for instruction in &instructions {
            area.apply(instruction);
        }
    Ok(area.tail_visited.len())
}

fn part2(input: &str) -> color_eyre::Result<usize> {
    let mut area = Area::new(10);
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
    knots: Vec<Point>,
    tail_visited: HashSet<Point>,
}

impl Area {
    pub fn new(knots: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert(Point::default());
        Self {
            knots: vec![Point::default(); knots],
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
            Direction::Up => self.knots[0].y += 1,
            Direction::Right => self.knots[0].x += 1,
            Direction::Down => self.knots[0].y -= 1,
            Direction::Left => self.knots[0].x -= 1,
        }
        self.follow_tail();
    }

    fn follow_tail(&mut self) {
        for i in 1..self.knots.len() {
            if self.knots[i-1].x.abs_diff(self.knots[i].x) <= 1 && self.knots[i-1].y.abs_diff(self.knots[i].y) <= 1 {
                // They are touching
                return;
            }
    
            if self.knots[i-1].x == self.knots[i].x {
                // same column
                self.knots[i].y += (self.knots[i-1].y - self.knots[i].y).signum();
            } else if self.knots[i-1].y == self.knots[i].y {
                // same row
                self.knots[i].x += (self.knots[i-1].x - self.knots[i].x).signum();
            } else {
                // diagonal
                self.knots[i].y += (self.knots[i-1].y - self.knots[i].y).signum();
                self.knots[i].x += (self.knots[i-1].x - self.knots[i].x).signum();
            }
        }
        
        self.tail_visited.insert(*self.knots.last().expect("There must be at least 1 knot"));
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    #[test]
    fn it_follows_length2() -> color_eyre::Result<()> {
        let input = read_example("day09.txt");
        assert_eq!(13, part1(&input)?);
        Ok(())
    }

    #[test]
    fn it_follows_length10() -> color_eyre::Result<()> {
        assert_eq!(1, part2(&read_example("day09.txt"))?);
        assert_eq!(36, part2(&read_example("day09_2.txt"))?);
        Ok(())
    }
}
