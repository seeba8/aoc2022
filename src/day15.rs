use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::Display,
    ops::{AddAssign, Range},
};

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
    println!("Day 15 part 2: {}", uncovered_spot2(4_000_000, &sensors));
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

pub fn uncovered_spot2(max: isize, sensors: &[Sensor]) -> isize {
    fn is_covered(point: &Point, sensors: &[Sensor]) -> bool {
        sensors
            .iter()
            .any(|s| s.position.manhattan_distance(point) <= s.range())
    }

    const fn is_in_range(point: &Point, max: isize) -> bool {
        point.x >= 0 && point.x <= max && point.y >= 0 && point.y <= max
    }

    for sensor in sensors {
        let starting_point = Point {
            x: sensor.position.x,
            y: sensor.position.y
                - std::convert::TryInto::<isize>::try_into(sensor.range()).unwrap()
                - 1,
        };
        let mut point = starting_point;
        let mut direction = Point { x: 1, y: 1 };
        loop {
            point += direction;
            if !is_covered(&point, sensors) && is_in_range(&point, max) {
                return point.x * 4_000_000 + point.y;
            };
            if point == starting_point {
                break;
            }
            match (
                point.x.cmp(&sensor.position.x),
                point.y.cmp(&sensor.position.y),
            ) {
                (Ordering::Greater, Ordering::Equal) => {
                    // We are at the right tip of the circumference
                    direction = Point { x: -1, y: 1 };
                }
                (Ordering::Equal, Ordering::Greater) => {
                    // We are at the bottom tip of the circumference
                    direction = Point { x: -1, y: -1 };
                }
                (Ordering::Less, Ordering::Equal) => {
                    // We are at the left tip of the circumference
                    direction = Point { x: 1, y: -1 };
                }
                _ => {}
            }
        }
    }
    todo!()
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
    pub fn parse_point(input: &str) -> IResult<&str, Self> {
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

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

    #[test]
    fn it_finds_uncovered_spot() {
        let input = read_example("day15.txt");
        let sensors: Vec<Sensor> = input.lines().map(Sensor::parse).collect();
        assert_eq!(56_000_011, uncovered_spot2(20, &sensors));
    }
}
