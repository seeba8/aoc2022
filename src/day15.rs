use std::{collections::HashSet, fmt::Display, ops::Range};

use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    sequence::{pair, preceded, separated_pair},
    Finish, IResult,
};

use crate::util::read_input;

pub fn solve() {
    let input = read_input("day15.txt");
    let sensors: Vec<Sensor> = input.lines().map(Sensor::parse).collect();
    println!("Day 15 part 1: {}", coverage(2_000_000, &sensors));
}

#[derive(Debug, Clone)]
pub struct Sensor {
    position: Point,
    beacon: Point,
}

impl Sensor {
    pub const fn range(&self) -> usize {
        self.position.manhattan_distance(&self.beacon)
    }
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::range_plus_one)]
    pub const fn range_at(&self, y: isize) -> Range<isize> {
        let y_offset = self.position.y.abs_diff(y);
        let Some(range_at_y) = self.range().checked_sub(y_offset) else {return 0..1;};

        (self.position.x - range_at_y as isize)..(self.position.x + range_at_y as isize + 1)
    }

    pub fn parse(input: &str) -> Self {
        all_consuming(map(
            pair(
                preceded(tag("Sensor at "), Point::parse_point),
                preceded(tag(": closest beacon is at "), Point::parse_point),
            ),
            |(position, beacon)| Self { position, beacon },
        ))(input)
        .finish()
        .unwrap()
        .1
    }
}

pub fn coverage(y: isize, sensors: &[Sensor]) -> usize {
    let beacon_positions: HashSet<isize> = sensors
        .iter()
        .filter(|s| s.beacon.y == y)
        .map(|s| s.beacon.x)
        .collect();
    let coverage: HashSet<isize> = sensors.iter().flat_map(|s| s.range_at(y)).collect();
    coverage.difference(&beacon_positions).count()
}

impl Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sensor at {}: closest beacon is at {}",
            self.position, self.beacon
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn parse_point(input: &str) -> IResult<&str, Point> {
        map(
            separated_pair(
                preceded(tag("x="), nom::character::complete::i32),
                tag(", "),
                preceded(tag("y="), nom::character::complete::i32),
            ),
            |(x, y)| (x as isize, y as isize).into(),
        )(input)
    }

    pub const fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x={}, y={}", self.x, self.y)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    #[test]
    fn it_prints_sensor() {
        let expected = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let sensor = Sensor {
            position: (2, 18).into(),
            beacon: (-2, 15).into(),
        };
        assert_eq!(expected, sensor.to_string());
    }

    #[test]
    fn it_parses_sensor() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let sensor = Sensor::parse(input);
        assert_eq!(input, sensor.to_string());
    }

    #[test]
    fn it_counts_coverage() {
        let input = read_example("day15.txt");
        let sensors: Vec<Sensor> = input.lines().map(Sensor::parse).collect();
        assert_eq!(26, coverage(10, &sensors));
    }
}
