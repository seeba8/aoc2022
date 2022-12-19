use std::fmt::Display;

use itertools::{Itertools, MinMaxResult};
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map, opt},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    Finish, IResult,
};

use crate::util::read_input;

pub fn solve() {
    let input = read_input("day14.txt");
    let mut cave = Cave::parse(&input, (500, 0), None);
    println!("Day 14 part 1: {}", cave.count_resting_sand());
    let mut cave = Cave::parse(&input, (500, 0), Some(2));
    println!("Day 14 part 2: {}", cave.count_resting_sand());
}

pub struct Cave {
    // index = x * height + y
    // That means it's the opposite direction since we are normally doing column operations
    grid: Vec<Cell>,
    height: usize,
    width: usize,
    drop_position: Coordinate,
}

impl Cave {
    pub fn set_cell(&mut self, coordinate: impl Into<Coordinate>, cell: Cell) {
        let coord = self.get_index(coordinate);
        self.grid[coord] = cell;
    }

    pub fn get_cell(&self, coordinate: impl Into<Coordinate>) -> Cell {
        self.grid[self.get_index(coordinate)]
    }

    pub fn count_resting_sand(&mut self) -> usize {
        let mut c = 0;
        while self.tick().is_some() {
            c += 1;
        }
        c
    }

    pub fn tick(&mut self) -> Option<Coordinate> {
        self.drop_sand(self.drop_position)
    }

    fn drop_sand(&mut self, coordinate: impl Into<Coordinate>) -> Option<Coordinate> {
        if self.get_cell(self.drop_position) == Cell::Sand {
            return None;
        }
        let mut coordinate: Coordinate = coordinate.into();
        if coordinate.x >= self.width {
            return None;
        }
        while Cell::Air == self.get_cell(coordinate) {
            coordinate.y += 1;
        }
        // back up 1
        coordinate.y -= 1;

        // try left and right first. If they are already filled, stay in the centre.
        if Cell::Air == self.get_cell((coordinate.x.checked_sub(1)?, coordinate.y + 1)) {
            self.drop_sand((coordinate.x.checked_sub(1)?, coordinate.y + 1))
        } else if Cell::Air == self.get_cell((coordinate.x + 1, coordinate.y + 1)) {
            self.drop_sand((coordinate.x + 1, coordinate.y + 1))
        } else {
            self.set_cell(coordinate, Cell::Sand);
            Some(coordinate)
        }
    }

    pub fn get_index(&self, coordinate: impl Into<Coordinate>) -> usize {
        let coordinate: Coordinate = coordinate.into();
        coordinate.x * self.height + coordinate.y
    }

    fn _parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
        map(
            separated_pair(
                nom::character::complete::u16,
                tag(","),
                nom::character::complete::u16,
            ),
            |(a, b)| (a as usize, b as usize).into(),
        )(input)
    }

    fn _parse_line(input: &str) -> IResult<&str, Vec<Coordinate>> {
        separated_list1(tag(" -> "), Self::_parse_coordinate)(input)
    }
    fn _parse_input(input: &str) -> IResult<&str, Vec<Vec<Coordinate>>> {
        separated_list1(many1(tuple((opt(tag("\r")), tag("\n")))), Self::_parse_line)(input)
    }

    fn draw_rock_line(&mut self, rocks: &[Coordinate]) {
        for (from, to) in rocks.iter().tuple_windows() {
            for y in from.y.min(to.y)..=from.y.max(to.y) {
                for x in from.x.min(to.x)..=from.x.max(to.x) {
                    self.set_cell((x, y), Cell::Rock);
                }
            }
        }
    }

    pub fn parse(
        input: &str,
        drop_position: impl Into<Coordinate>,
        floor_offset: Option<usize>,
    ) -> Self {
        let mut drop_position: Coordinate = drop_position.into();
        let mut lines = all_consuming(Self::_parse_input)(input.trim())
            .finish()
            .unwrap()
            .1;
        let MinMaxResult::MinMax(min, max) = lines.iter().flatten().map(|x| x.x).minmax() else {
            panic!()
        };
        let minmax_x = (min.min(drop_position.x), max.max(drop_position.x));
        let MinMaxResult::MinMax(min, max) = lines.iter().flatten().map(|y| y.y).minmax() else {
            panic!()
        };
        let mut minmax_y = (min.min(drop_position.y), max.max(drop_position.y));

        let mut width = minmax_x.1 - minmax_x.0 + 1;
        let mut height = minmax_y.1 - minmax_y.0 + 1;
        let mut add_x_offset = 0;
        if let Some(floor_offset) = floor_offset {
            minmax_y.1 += floor_offset;
            height += floor_offset;
            add_x_offset = ((2 * height + 2) - width) / 2;
            width += add_x_offset * 2;
            lines.push(vec![
                (minmax_x.0 - add_x_offset, minmax_y.1).into(),
                (minmax_x.1 + add_x_offset, minmax_y.1).into(),
            ]);
        }

        drop_position.x = drop_position.x + add_x_offset - minmax_x.0;
        drop_position.y -= minmax_y.0;
        let grid: Vec<Cell> = vec![Cell::Air; width * height];

        let mut cave = Self {
            grid,
            height,
            width,
            drop_position,
        };
        // shift input over to be properly zeroed
        for line in &mut lines {
            for c in line {
                c.x = c.x + add_x_offset - minmax_x.0;
                c.y -= minmax_y.0;
            }
        }
        for formation in lines {
            cave.draw_rock_line(&formation);
        }
        cave
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.drop_position == (x, y).into() {
                    write!(f, "+")?;
                } else {
                    write!(f, "{}", self.get_cell((x, y)))?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Air,
    Rock,
    Sand,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    #[test]
    fn it_parses_rocks() {
        let input = read_example("day14.txt");
        let cave = Cave::parse(&input, (500, 0), None);
        let expected = r#"
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########."#
            .trim();
        assert_eq!(expected, cave.to_string().trim());
    }

    #[test]
    fn it_drops_single_sand() {
        let input = read_example("day14.txt");
        let mut cave = Cave::parse(&input, (500, 0), None);
        let position = cave.tick();
        assert_eq!(Some((6, 8).into()), position);
    }

    #[test]
    fn it_drops_two_sands() {
        let input = read_example("day14.txt");
        let mut cave = Cave::parse(&input, (500, 0), None);
        cave.tick();
        let position = cave.tick();
        assert_eq!(Some((5, 8).into()), position);
    }

    #[test]
    fn it_drops_saaand() {
        let input = read_example("day14.txt");
        let mut cave = Cave::parse(&input, (500, 0), None);
        for _ in 0..22 {
            cave.tick();
        }
        let expected = r"
......+...
..........
......o...
.....ooo..
....#ooo##
....#ooo#.
..###ooo#.
....oooo#.
...ooooo#.
#########."
            .trim();
        let actual = cave.to_string();
        let actual = actual.trim();
        assert_eq!(expected, actual, "expected:\n{expected}, actual:\n{actual}");
    }
    #[test]
    fn it_drops_saaaaaaaaaaaaaaand() {
        let input = read_example("day14.txt");
        let mut cave = Cave::parse(&input, (500, 0), None);
        for _ in 0..24 {
            assert!(cave.tick().is_some());
        }
        assert!(cave.tick().is_none());
    }

    #[test]
    fn it_solves_part1() {
        let input = read_example("day14.txt");
        let mut cave = Cave::parse(&input, (500, 0), None);
        assert_eq!(24, cave.count_resting_sand());
    }

    #[test]
    fn it_adds_floor() {
        let input = read_example("day14.txt");
        let cave = Cave::parse(&input, (500, 0), Some(2));
        let expected = r"
..............+...........
..........................
..........................
..........................
............#...##........
............#...#.........
..........###...#.........
................#.........
................#.........
........#########.........
..........................
##########################"
            .trim();
        let actual = cave.to_string();
        let actual = actual.trim();
        assert_eq!(expected, actual, "expected:\n{expected}, actual:\n{actual}");
    }

    #[test]
    fn it_drops_on_floor() {
        let input = read_example("day14.txt");
        let mut cave = Cave::parse(&input, (500, 0), Some(2));
        let res = cave.count_resting_sand();
        println!("{}", &cave);
        assert_eq!(93, res);
    }
}
