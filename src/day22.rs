use std::fmt::{Display, Formatter};
use nom::branch::alt;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{all_consuming, map};
use nom::{Finish, IResult};
use crate::util::{Direction, read_input};

pub fn solve() {
    let input = read_input("day22.txt");
    let (map,mut me) = init(&input);
    me.follow_instructions(&map);
    println!("Day 22 part 1: {}", me.get_password());
}

pub fn init(input: &str) -> (Map, Me) {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let map = Map::parse(map);
    let mut me = Me::parse(instructions);
    me.set_starting_point(&map);
    (map, me)
}

pub struct Map {
    grid: Vec<Tile>,
    width: isize,
    height: isize,
}

impl Map {
    fn parse(s: &str) -> Self {
        let width = isize::try_from(s.lines().map(str::len).max().unwrap()).unwrap();
        let height = isize::try_from(s.lines().count()).unwrap();
        let grid = vec![Tile::Void; usize::try_from(width * height).unwrap()];
        let mut map = Self { grid, width, height };
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.set(isize::try_from(x).unwrap(), isize::try_from(y).unwrap(), match c {
                    ' ' => Tile::Void,
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    _ => unreachable!()
                });
            }
        }
        map
    }

    fn set(&mut self, x: isize, y: isize, tile: Tile) {
        self.grid[usize::try_from(y * self.width + x).unwrap()] = tile;
    }

    fn get(&self, x: isize, y: isize) -> Tile {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            Tile::Void
        } else {
            self.grid[usize::try_from(y * self.width + x).unwrap()]
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Me {
    instructions: Vec<Instruction>,
    position: (isize, isize),
    direction: Direction,
}


impl Me {
    fn _digit_or_direction(input: &str) -> IResult<&str, &str> {
        alt((digit1, alpha1))(input)
    }
    fn parse(s: &str) -> Self {
        let instructions: Vec<Instruction> = all_consuming(
            map(nom::multi::many1(
                Self::_digit_or_direction),
                |v| v.into_iter().map(Instruction::from).collect::<Vec<Instruction>>())
        )(s.trim()).finish().unwrap().1;
        Self { instructions, position: (0, 0), direction: Direction::Right }
    }

    pub fn set_starting_point(&mut self, map: &Map) {
        for x in 0..map.width {
            if map.get(x, 0) == Tile::Open {
                self.position = (x, 0);
                return;
            }
        }
    }

    fn follow_instructions(&mut self, map: &Map) {
        let instructions = self.instructions.clone();
        for instruction in instructions {
            self.follow_instruction(map, instruction);
        }
    }
    fn try_move_up(&mut self, map: &Map) {
        match map.get(self.position.0, self.position.1 - 1) {
            Tile::Void => {
                for i in (0..map.height).rev() {
                    match map.get(self.position.0, i) {
                        Tile::Void => {}
                        Tile::Open => {
                            self.position.1 = i;
                            return;
                        }
                        Tile::Wall => {
                            return;
                        }
                    }
                }
            }
            Tile::Open => { self.position.1 -= 1; }
            Tile::Wall => {}
        }
    }
    fn try_move_down(&mut self, map: &Map) {
        match map.get(self.position.0, self.position.1 + 1) {
            Tile::Void => {
                for i in 0..map.height {
                    match map.get(self.position.0, i) {
                        Tile::Void => {}
                        Tile::Open => {
                            self.position.1 = i;
                            return;
                        }
                        Tile::Wall => {
                            return;
                        }
                    }
                }
            }
            Tile::Open => { self.position.1 += 1; }
            Tile::Wall => {}
        }
    }
    fn try_move_left(&mut self, map: &Map) {
        match map.get(self.position.0 - 1, self.position.1) {
            Tile::Void => {
                for i in (0..map.width).rev() {
                    match map.get(i, self.position.1) {
                        Tile::Void => {}
                        Tile::Open => {
                            self.position.0 = i;
                            return;
                        }
                        Tile::Wall => {
                            return;
                        }
                    }
                }
            }
            Tile::Open => { self.position.0 -= 1; }
            Tile::Wall => {}
        }
    }
    fn try_move_right(&mut self, map: &Map) {
        match map.get(self.position.0 + 1, self.position.1) {
            Tile::Void => {
                for i in 0..map.width {
                    match map.get(i, self.position.1) {
                        Tile::Void => {}
                        Tile::Open => {
                            self.position.0 = i;
                            return;
                        }
                        Tile::Wall => {
                            return;
                        }
                    }
                }
            }
            Tile::Open => { self.position.0 += 1; }
            Tile::Wall => {}
        }
    }
    fn follow_instruction(&mut self, map: &Map, instruction: Instruction) {
        match instruction {
            Instruction::TurnLeft => self.direction = self.direction.turn(true),
            Instruction::TurnRight => self.direction = self.direction.turn(false),
            Instruction::Move(num_steps) => {
                for _ in 0..num_steps {
                    match self.direction {
                        Direction::Up => { self.try_move_up(map) }
                        Direction::Right => { self.try_move_right(map) }
                        Direction::Down => { self.try_move_down(map) }
                        Direction::Left => { self.try_move_left(map) }
                    }
                }
            }
        }
    }

    const fn get_password(&self) -> isize {
        (self.position.1 + 1) * 1_000 + (self.position.0 + 1) * 4 + match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub enum Tile {
    #[default]
    Void,
    Open,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Void => ' ',
            Self::Open => '.',
            Self::Wall => '#',
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Instruction {
    Move(u8),
    TurnLeft,
    TurnRight,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Self::TurnRight,
            "L" => Self::TurnLeft,
            _ => Self::Move(value.parse().unwrap_or(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::util::read_example;
    use super::*;

    #[test]
    fn it_parses_instructions() {
        let expected = vec![Instruction::Move(10), Instruction::TurnRight, Instruction::Move(5), Instruction::TurnLeft];
        assert_eq!(Me::parse("10R5L"), Me { instructions: expected, position: (0, 0), direction: Direction::Right });
    }

    #[test]
    fn it_parses_map() {
        let input = read_example("day22.txt");
        let (map, instructions) = input.split_once("\n\n").unwrap();
        let map = Map::parse(map);
        let expected = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.";
        let actual: String = map.to_string().lines().map(|line| line.trim_end()).join("\n");
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_follows_instructions() {
        let input = read_example("day22.txt");
        let (map, instructions) = input.split_once("\n\n").unwrap();
        let map = Map::parse(map);
        let mut me = Me::parse(instructions);
        me.set_starting_point(&map);
        assert_eq!((8, 0), me.position);
        me.follow_instruction(&map, me.instructions[0]);
        assert_eq!((10,0), me.position);
        assert_eq!(Direction::Right, me.direction);
        me.follow_instruction(&map, me.instructions[1]);
        assert_eq!((10,0), me.position);
        assert_eq!(Direction::Down, me.direction);
        me.follow_instruction(&map, me.instructions[2]);
        assert_eq!((10,5), me.position);
        assert_eq!(Direction::Down, me.direction);
        me.follow_instruction(&map, me.instructions[3]);
        assert_eq!((10,5), me.position);
        assert_eq!(Direction::Right, me.direction);
        me.follow_instruction(&map, me.instructions[4]);
        assert_eq!((3,5), me.position);
        assert_eq!(Direction::Right, me.direction);
    }
    #[test]
    fn it_follows_instructions_to_the_end() {
        let input = read_example("day22.txt");
        let (map, mut me): (Map, Me) = init(&input);
        me.follow_instructions(&map);
        assert_eq!((7,5), me.position);
        assert_eq!(Direction::Right, me.direction);
        assert_eq!(6_032, me.get_password());
    }
}