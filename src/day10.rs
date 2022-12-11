use std::str::FromStr;

use itertools::Itertools;

use crate::util::read_input;

const SIGNAL_OFFSET: usize = 20;
const SIGNAL_INTERVAL: usize = 40;
const SCREEN_WIDTH: usize = 40;

pub fn solve() -> color_eyre::Result<()> {
    let input = read_input("day10.txt");
    println!("Day 10 part 1: {}", part1(&input)?);
    println!("Day 10 part 1:\n{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> color_eyre::Result<isize> {
    let mut video_system = VideoSystem::default();
    let instructions: Vec<Instruction> = input.lines().map(str::parse).collect::<color_eyre::Result<Vec<_>>>()?;
    for instruction in &instructions {
        video_system.apply(instruction);
    }
    Ok(video_system.signal_strengths.iter().sum())
}

fn part2(input: &str) -> color_eyre::Result<String> {
    let mut video_system = VideoSystem::default();
    let instructions: Vec<Instruction> = input.lines().map(str::parse).collect::<color_eyre::Result<Vec<_>>>()?;
    for instruction in &instructions {
        video_system.apply(instruction);
    }
    Ok(video_system.draw())
}

#[derive(Debug, Clone)]
pub struct VideoSystem {
    signal_strengths: Vec<isize>,
    cycle: usize,
    register: isize,
    screen: Vec<u64>
}

impl Default for VideoSystem {
    fn default() -> Self {
        Self { signal_strengths: Vec::default(), cycle: Default::default(), register: 1, screen: Vec::default() }
    }
}

impl VideoSystem {
    fn apply(&mut self, instruction: &Instruction) {
        let old_cycle = self.cycle;
        self.cycle += instruction.cost();
        for cycle in (old_cycle+1)..=self.cycle {
            if cycle % SIGNAL_INTERVAL == SIGNAL_OFFSET {
                #[allow(clippy::cast_possible_wrap)]
                self.signal_strengths.push(cycle as isize * self.register);
            }

            let screen_position = (cycle - 1) % SCREEN_WIDTH;
            if screen_position == 0 {
                self.screen.push(u64::MAX);
            }
            if self.register.abs_diff(screen_position.try_into().expect("Cannot fail since screen position is between 0 and 39 thanks to modulo")) > 1{
                *self.screen.last_mut().expect("Screen row not initialized") &= !(1 << screen_position);
            }
        }
        if let Instruction::Addx(value) = instruction {
            self.register += value;
        }
        
    }

    fn draw(&self) -> String {
        self.screen.iter().map(|row| format!("{row:b}").replace('1', "#").replace('0', ".")[24..].chars().rev().collect::<String>()).join("\n")
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Instruction {
    Addx(isize),
    #[default]
    Noop
}

impl Instruction {
    pub const fn cost(&self) -> usize {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }
}

impl FromStr for Instruction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else if let Some((_, value)) = s.split_once(' ') {
            Ok(Self::Addx(value.parse()?))
        } else {
            Err(color_eyre::eyre::eyre!("Cannot parse instruction '{s}"))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    #[test]
    fn it_applies_instructions() {
        let input = read_example("day10.txt");
        let mut video_system = VideoSystem::default();
        let instructions: Vec<Instruction> = input.lines().map(str::parse).collect::<color_eyre::Result<Vec<_>>>().unwrap();
        video_system.apply(&instructions[0]);
        assert_eq!(1, video_system.register);
        assert_eq!(1, video_system.cycle);
        video_system.apply(&instructions[1]);
        assert_eq!(4, video_system.register);
        assert_eq!(3, video_system.cycle);
        video_system.apply(&instructions[2]);
        assert_eq!(-1, video_system.register);
        assert_eq!(5, video_system.cycle);
    }

    #[test]
    fn it_solves_part1() {
        let input = read_example("day10_2.txt");
        assert_eq!(13_140, part1(&input).unwrap());
    }

    #[test]
    fn it_draws_example() {
        let input = read_example("day10_2.txt");
        assert_eq!(read_example("day10_expected.txt").replace("\r\n", "\n"),part2(&input).unwrap());
    }
}