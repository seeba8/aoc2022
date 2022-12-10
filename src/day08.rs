use itertools::Itertools;
use std::str::FromStr;

use crate::util::{read_input, Direction};

pub fn solve() -> color_eyre::Result<()> {
    let input = read_input("day08.txt");
    println!("Day 08 part 1: {}", part1(&input)?);
    println!("Day 08 part 2: {}", part2(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> color_eyre::Result<usize> {
    let mut forest: Forest = input.parse()?;
    forest.calculate_visibility();
    Ok(forest.trees.iter().filter(|tree| tree.visible).count())
}

pub fn part2(input: &str) -> color_eyre::Result<usize> {
    let mut forest: Forest = input.parse()?;
    forest.calculate_scenic_score();
    forest
        .trees
        .iter()
        .map(|tree| tree.scenic_score)
        .max()
        .ok_or_else(|| color_eyre::eyre::eyre!("Cannot get maximum scenic score"))
}

#[derive(Debug, Clone, Default)]
pub struct Forest {
    pub trees: Vec<Tree>,
    width: usize,
    #[allow(dead_code)]
    height: usize,
}

impl FromStr for Forest {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(std::convert::TryInto::try_into)
            .collect::<color_eyre::Result<Vec<_>>>()?;
        Ok(Self {
            trees,
            width: s
                .lines()
                .next()
                .ok_or_else(|| color_eyre::eyre::eyre!("Cannot split input into lines"))?
                .len(),
            height: s.lines().count(),
        })
    }
}

impl Forest {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            trees: Vec::with_capacity(width * height),
        }
    }

    fn calculate_scenic_score(&mut self) {
        for index in 0..self.trees.len() {
            self.trees[index].scenic_score = [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
            .into_iter()
            .map(|direction| self.scenic_score_in_direction(index, direction))
            .product();
        }
    }

    fn scenic_score_in_direction(&self, index: usize, direction: Direction) -> usize {
        let (x, y) = self.get_xy(index);
        let height = self.trees[index].height;
        match direction {
            Direction::Up => {
                let mut s = 0;
                for t in self.trees[x..=index]
                    .iter()
                    .rev()
                    .skip(self.width)
                    .step_by(self.width)
                {
                    s += 1;
                    if t.height >= height {
                        break;
                    }
                }
                s
            }
            Direction::Right => {
                let mut s = 0;
                for t in &self.trees[(index + 1)..((y + 1) * self.width)] {
                    s += 1;
                    if t.height >= height {
                        break;
                    }
                }
                s
            }
            Direction::Down => {
                let mut s = 0;
                for t in self.trees[(index)..]
                    .iter()
                    .skip(self.width)
                    .step_by(self.width)
                {
                    s += 1;
                    if t.height >= height {
                        break;
                    }
                }
                s
            }
            Direction::Left => {
                let mut s = 0;
                for t in self.trees[(y * self.width)..index].iter().rev() {
                    s += 1;
                    if t.height >= height {
                        break;
                    }
                }
                s
            }
        }
    }

    fn calculate_visibility(&mut self) {
        for index in 0..self.trees.len() {
            if [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
            .into_iter()
            .any(|d| self.is_visible_looking_from(index, d))
            {
                self.trees[index].visible = true;
            }
        }
    }

    fn is_visible_looking_from(&self, index: usize, direction: Direction) -> bool {
        let (x, y) = self.get_xy(index);
        let height = self.trees[index].height;
        match direction {
            Direction::Up => self.trees[x..index]
                .iter()
                .step_by(self.width)
                .all(|t| t.height < height),
            Direction::Right => self.trees[(index + 1)..((y + 1) * self.width)]
                .iter()
                .all(|t| t.height < height),
            Direction::Down => self.trees[(index)..]
                .iter()
                .skip(self.width)
                .step_by(self.width)
                .all(|t| t.height < height),
            Direction::Left => self.trees[(y * self.width)..index]
                .iter()
                .all(|t| t.height < height),
        }
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap, unused)]
    fn get_neighbours(&self, index: usize, include_diagonals: bool) -> Vec<usize> {
        let (x, y) = self.get_xy(index);
        (-1..=1_isize)
            .cartesian_product(-1..=1)
            .filter(|(x_offset, y_offset)| include_diagonals || *x_offset == 0 || *y_offset == 0)
            .filter(|(x_offset, y_offset)| *x_offset != 0 || *y_offset != 0)
            .map(|(x_offset, y_offset)| (x as isize + x_offset, y as isize + y_offset))
            .filter(|(x, y)| self.is_in_bounds(*x, *y))
            .map(|(x, y)| y as usize * self.width + x as usize)
            .collect()
    }

    #[allow(clippy::cast_possible_wrap)]
    const fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    const fn get_xy(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree {
    height: u8,
    visible: bool,
    scenic_score: usize,
}

impl TryFrom<char> for Tree {
    type Error = color_eyre::Report;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
            height: value
                .to_digit(10)
                .ok_or_else(|| color_eyre::eyre::eyre!("Cannot parse tree '{value}'"))?
                as u8,
            ..Default::default()
        })
    }
}

impl Tree {
    #[allow(dead_code)]
    fn from_height(height: u8) -> Self {
        Self {
            height,
            ..Default::default()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;

    #[test]
    fn it_parses_forest() {
        let input = read_example("day08.txt");
        let forest: Forest = input.parse().unwrap();
        assert_eq!(25, forest.trees.len());
        assert_eq!(5, forest.height);
        assert_eq!(5, forest.width);
    }

    #[test]
    fn it_finds_neighbours() {
        let forest: Forest = Forest {
            trees: (0..25).map(Tree::from_height).collect(),
            width: 5,
            height: 5,
        };
        let mut neighbours = forest.get_neighbours(6, false);
        neighbours.sort_unstable();
        assert_eq!(vec![1, 5, 7, 11], neighbours);

        let mut neighbours = forest.get_neighbours(5, false);
        neighbours.sort_unstable();
        assert_eq!(vec![0, 6, 10], neighbours);
    }

    #[test]
    fn it_solves_part1() {
        let input = read_example("day08.txt");
        assert_eq!(21, part1(&input).unwrap());
    }

    #[test]
    fn it_gets_the_scenic_score() {
        let input = read_example("day08.txt");
        let mut forest = Forest::from_str(&input).unwrap();
        assert_eq!(
            2,
            forest.scenic_score_in_direction(17, Direction::Up),
            "Upwards"
        );
        assert_eq!(
            2,
            forest.scenic_score_in_direction(17, Direction::Left),
            "Leftwards"
        );
        assert_eq!(
            1,
            forest.scenic_score_in_direction(17, Direction::Down),
            "Downwards"
        );
        assert_eq!(
            2,
            forest.scenic_score_in_direction(17, Direction::Right),
            "Rightwards"
        );

        forest.calculate_scenic_score();
        assert_eq!(
            1,
            forest.scenic_score_in_direction(7, Direction::Up),
            "Upwards"
        );
        assert_eq!(
            1,
            forest.scenic_score_in_direction(7, Direction::Left),
            "Leftwards"
        );
        assert_eq!(
            2,
            forest.scenic_score_in_direction(7, Direction::Down),
            "Downwards"
        );
        assert_eq!(
            2,
            forest.scenic_score_in_direction(7, Direction::Right),
            "Rightwards"
        );
    }

    #[test]
    fn it_solves_part2() {
        let input = read_example("day08.txt");
        assert_eq!(8, part2(&input).unwrap());
    }

    #[test]
    fn test_iterator_rev() {
        println!("{:?}", (0..5).rev().collect_vec());
        println!("{:?}", (0..5).rev().skip(4).collect_vec());
    }
}
