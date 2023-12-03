use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use itertools::Itertools;
use crate::util::{Direction, read_input};

pub fn solve() {
    let input = read_input("day24.txt");
    let mut valley: Valley = input.parse().unwrap();
    println!("Day 24 part 1: {}", valley.find_fastest_path());
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: u8,
    y: u8,
}

impl TryFrom<char> for Direction {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            _ => Err(color_eyre::eyre::eyre!("Illegal direction: {value}"))
        }
    }
}

impl Point {
    fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }
}

#[derive(Default)]
struct Valley {
    blizzards: Vec<Blizzard>,
    width: u8,
    height: u8,
}

impl Valley {

    fn find_fastest_path(&mut self) -> usize {
        let mut ticks = 0;
        let mut expeditions = HashSet::new();
        expeditions.insert(Point::new(1, 0));
        loop {
            self.tick_blizzards();
            expeditions = self.get_expedition_movements(&expeditions);
            ticks += 1;
            dbg!(&expeditions.len());
            if expeditions.iter().any(|expedition| expedition.x == self.width - 2 && expedition.y == self.height - 1) {
                return ticks;
            }
        }
    }
    fn contains_blizzard(&self, point: &Point) -> bool {
        self.blizzards.iter().any(|b| b.position == *point)
    }
    fn get_expedition_movements(&self, expeditions: &HashSet<Point>) -> HashSet<Point> {
        let mut new_expeditions = HashSet::new();
        for expedition in expeditions {
            if !self.contains_blizzard(expedition) {
                // wait
                new_expeditions.insert(*expedition);
            }
            if expedition.y > 1 || expedition.y == 1 && expedition.x == 0 {
                // up
                let target = Point::new(expedition.x, expedition.y - 1);
                if !self.contains_blizzard(&target) {
                    new_expeditions.insert(target);
                }
            }
            if expedition.y < self.height - 2 || expedition.y == self.height - 2 && expedition.x == self.width - 2 {
                // down
                let target = Point::new(expedition.x, expedition.y + 1);
                if !self.contains_blizzard(&target) {
                    new_expeditions.insert(target);
                }
            }
            if expedition.x > 1 {
                // left
                let target = Point::new(expedition.x - 1, expedition.y);
                if !self.contains_blizzard(&target) {
                    new_expeditions.insert(target);
                }
            }
            if expedition.x < self.width - 2 && expedition.y > 0 {
                // right
                let target = Point::new(expedition.x + 1, expedition.y);
                if !self.contains_blizzard(&target) {
                    new_expeditions.insert(target);
                }
            }
        }
        new_expeditions
    }
    fn tick_blizzards(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            match blizzard.direction {
                Direction::Up => {
                    blizzard.position.y -= 1;
                    if blizzard.position.y == 0 {
                        blizzard.position.y = self.height - 2;
                    }
                }
                Direction::Right => {
                    blizzard.position.x += 1;
                    if blizzard.position.x == self.width - 1 {
                        blizzard.position.x = 1;
                    }
                }
                Direction::Down => {
                    blizzard.position.y += 1;
                    if blizzard.position.y == self.height - 1 {
                        blizzard.position.y = 1;
                    }
                }
                Direction::Left => {
                    blizzard.position.x -= 1;
                    if blizzard.position.x == 0 {
                        blizzard.position.x = self.width - 2;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Blizzard {
    direction: Direction,
    position: Point,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Right => write!(f, ">"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
        }
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let right_wall = self.width - 1;
        let bottom_wall = self.height - 1;
        for y in 0..self.height {
            for x in 0..self.width {
                match (x, y) {
                    (0, _) => write!(f, "#")?,
                    (x, _) if x == right_wall => write!(f, "#")?,
                    (x, 0) if x != 1 => write!(f, "#")?,
                    (x, y) if y == bottom_wall && x != right_wall - 1 => write!(f, "#")?,
                    (x, y) => {
                        let p = Point { x, y };

                        let blizzards = self.blizzards.iter().filter_map(|b| if b.position == p { Some(b.direction) } else { None }).collect_vec();
                        match blizzards.len() {
                            0 => write!(f, "."),
                            1 => write!(f, "{}", blizzards[0]),
                            x => write!(f, "{x}")
                        }?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Valley {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.trim().lines().count();
        let width = s.trim().lines().next().ok_or_else(|| color_eyre::eyre::eyre!("Empty input"))?.len();
        let mut valley = Valley::default();
        valley.width = width.try_into()?;
        valley.height = height.try_into()?;
        for (y, line) in s.trim().lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                if ['^', '>', 'v', '<'].contains(&c) {
                    valley.blizzards.push(Blizzard { direction: c.try_into()?, position: Point::new(u8::try_from(x)?, u8::try_from(y)?) });
                }
            }
        }
        Ok(valley)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::read_example;
    use super::*;

    #[test]
    fn it_parses_valley() {
        let input = read_example("day24_1.txt");
        let valley: Valley = input.parse().unwrap();
        assert_eq!(2, valley.blizzards.len());
    }

    #[test]
    fn it_displays_valley() {
        let mut input = read_example("day24_1.txt");
        let valley: Valley = input.parse().unwrap();
        input.replace_range(1..=1, "E");
        assert_eq!(valley.to_string().trim(), input.trim());
    }

    #[test]
    fn it_ticks_blizzards() {
        let mut input = read_example("day24_1.txt");
        let mut valley: Valley = input.parse().unwrap();
        input.replace_range(1..=1, "E");
        valley.tick_blizzards();
        valley.tick_blizzards();
        let expected = r"#.#####
#...v.#
#..>..#
#.....#
#.....#
#.....#
#####.#".replace('\r', "");
        assert_eq!(valley.to_string().trim(), expected.trim());
        valley.tick_blizzards();
        let expected = r"#.#####
#.....#
#...2.#
#.....#
#.....#
#.....#
#####.#".replace('\r', "");
        assert_eq!(valley.to_string().trim(), expected.trim());
    }

    #[test]
    fn it_finds_fastest_path() {
        let input = read_example("day24_2.txt");
        let mut valley: Valley = input.parse().unwrap();
        assert_eq!(valley.find_fastest_path(), 18);
    }
}