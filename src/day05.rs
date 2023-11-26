use color_eyre::eyre;
use std::fmt::Write;
use std::{fmt::Display, str::FromStr};

use crate::util::read_input;
pub fn solve() -> color_eyre::Result<()> {
    let input = read_input("day05.txt");
    println!("Day 05 part 1: {}", part1(&input)?);
    println!("Day 05 part 1: {}", part2(&input)?);
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Crate(char);
impl From<char> for Crate {
    fn from(value: char) -> Self {
        Self(value)
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stack(Vec<Crate>);

impl Stack {
    pub fn push(&mut self, c: Crate) {
        self.0.push(c);
    }

    pub fn pop(&mut self) -> Option<Crate> {
        self.0.pop()
    }

    pub fn last(&self) -> Option<&Crate> {
        self.0.last()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Instruction {
    quantity: u8,
    from: u8,
    to: u8,
}

impl FromStr for Instruction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // pattern: "move 1 from 2 to 1"
        // positions are 1-based
        let mut segments = s.split_ascii_whitespace().skip(1).step_by(2);
        Ok(Self {
            quantity: segments
                .next()
                .ok_or(eyre::eyre!("Cannot get quantity"))?
                .parse()?,
            from: segments
                .next()
                .ok_or(eyre::eyre!("Cannot get from"))?
                .parse()?,
            to: segments
                .next()
                .ok_or(eyre::eyre!("Cannot get to"))?
                .parse()?,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct Ship {
    pub stacks: Vec<Stack>,
}

#[derive(Debug, Clone, Copy)]
pub enum Crane {
    CrateMover9000,
    CrateMover9001,
}

impl FromStr for Ship {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ship = Self::default();
        let mut is_first_row = true;
        for line in s.lines().rev() {
            for (idx, c) in line.chars().skip(1).step_by(4).enumerate() {
                if is_first_row {
                    ship.stacks.push(Stack::default());
                } else if c.is_alphabetic() {
                    ship.stacks[idx].push(c.into());
                }
            }
            is_first_row = false;
        }

        Ok(ship)
    }
}

impl Ship {
    fn execute_instructions(
        &mut self,
        instructions: &[Instruction],
        crane: Crane,
    ) -> color_eyre::Result<()> {
        match crane {
            Crane::CrateMover9000 => {
                for i in instructions {
                    self.execute_9000(i)?;
                }
            }
            Crane::CrateMover9001 => {
                for i in instructions {
                    self.execute_9001(i);
                }
            }
        }

        Ok(())
    }

    fn execute_9000(&mut self, instruction: &Instruction) -> color_eyre::Result<()> {
        for _ in 0..instruction.quantity {
            let c = self.stacks[instruction.from as usize - 1]
                .pop()
                .ok_or_else(|| eyre::eyre!("Cannot pop from stack {}", instruction.from))?;
            self.stacks[instruction.to as usize - 1].push(c);
        }
        Ok(())
    }

    fn execute_9001(&mut self, instruction: &Instruction) {
        let source_len = self.stacks[instruction.from as usize - 1].0.len();

        let moved_stack: Vec<Crate> = self.stacks[instruction.from as usize - 1]
            .0
            .drain((source_len - instruction.quantity as usize)..)
            .collect();
        self.stacks[instruction.to as usize - 1]
            .0
            .extend(moved_stack);
    }

    fn get_tops(&self) -> String {
        self.stacks.iter().fold(String::new(), |mut output, s| {
            let _ = write!(output, "{}", *s.last().unwrap());
            output
        })
    }
}

fn part1(input: &str) -> color_eyre::Result<String> {
    _both_parts(input, Crane::CrateMover9000)
}

fn part2(input: &str) -> color_eyre::Result<String> {
    _both_parts(input, Crane::CrateMover9001)
}

fn _both_parts(input: &str, crane: Crane) -> color_eyre::Result<String> {
    let binding = input.replace("\r\n", "\n");
    let (ship, instructions) = binding
        .split_once("\n\n")
        .ok_or_else(|| color_eyre::eyre::eyre!("Cannot split between stacks and instructions"))?;

    let mut ship: Ship = ship.parse().unwrap();
    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(str::parse)
        .collect::<color_eyre::Result<Vec<Instruction>>>()?;
    ship.execute_instructions(&instructions, crane)?;
    Ok(ship.get_tops())
}
#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    #[test]
    fn it_parses_row_of_stacks_with_spaces() {
        let input = r"[Q] [G]     [V]     [S]         [V]";
        let ship = Ship::from_str(input).unwrap();
        assert_eq!(9, ship.stacks.len());
    }

    #[test]
    fn it_parses_stacks() {
        let input = "[T] [L] [D] [G] [P] [P] [V] [N] [R]
 1   2   3   4   5   6   7   8   9 ";
        let mut ship = Ship::from_str(input).unwrap();
        assert_eq!(9, ship.stacks.len());
        assert_eq!(Crate::from('T'), ship.stacks[0].pop().unwrap());
        assert_eq!(Crate::from('R'), ship.stacks[8].pop().unwrap());
    }

    #[test]
    fn it_parses_stacks_of_example1() {
        let input = read_example("day05.txt");

        let binding = input.replace("\r\n", "\n");
        let (ship, _) = binding
            .split_once("\n\n")
            .ok_or_else(|| color_eyre::eyre::eyre!("Cannot split between stacks and instructions"))
            .unwrap();

        let ship: Ship = ship.parse().unwrap();
        assert_eq!(3, ship.stacks.len());
        assert_eq!(Stack(vec![Crate('Z'), Crate('N')]), ship.stacks[0]);
        assert_eq!(
            Stack(vec![Crate('M'), Crate('C'), Crate('D')]),
            ship.stacks[1]
        );
        assert_eq!(Stack(vec![Crate('P')]), ship.stacks[2]);
    }

    #[test]
    fn it_parses_instruction() {
        let input = "move 1 from 2 to 1";
        let expected = Instruction {
            quantity: 1,
            from: 2,
            to: 1,
        };
        assert_eq!(expected, Instruction::from_str(input).unwrap());
    }

    #[test]
    fn it_solves_example1() {
        let input = read_example("day05.txt");
        assert_eq!("CMZ".to_owned(), part1(&input).unwrap());
    }

    #[test]
    fn it_solves_example2() {
        let input = read_example("day05.txt");
        assert_eq!("MCD".to_owned(), part2(&input).unwrap());
    }
}
