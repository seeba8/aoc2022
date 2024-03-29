use std::{
    fmt::{Debug, Display},
    ops::{BitAnd, BitOrAssign},
    str::FromStr,
};

use itertools::Itertools;

use crate::util::{read_input, Direction};

pub fn solve() -> color_eyre::Result<()> {
    let input = read_input("day17.txt");
    let mut chamber = Chamber::new(Jet::from_str(&input)?);
    chamber.drop_rocks(2022);
    println!("Day 17 part 1: {}", chamber.get_highest_occupied_row());
    Ok(())
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Rock {
    shape: Vec<Row>,
}

impl Rock {
    fn new(shape: Vec<impl Into<Row>>) -> Self {
        Self {
            shape: shape.into_iter().map(std::convert::Into::into).collect(),
        }
    }

    fn height(&self) -> usize {
        self.shape.len()
    }

    fn shr(&self) -> Self {
        if self.shape.iter().all(|row| row.0.trailing_zeros() > 0) {
            Self {
                shape: self
                    .shape
                    .iter()
                    .copied()
                    .map(|row| Row::from(row.0 >> 1))
                    .collect(),
            }
        } else {
            self.clone()
        }
    }

    fn shl(&self) -> Self {
        if self.shape.iter().all(|row| row.0.leading_zeros() > 0) {
            Self {
                shape: self
                    .shape
                    .iter()
                    .copied()
                    .map(|row| Row::from(row.0 << 1))
                    .collect(),
            }
        } else {
            self.clone()
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Row(u8);

impl From<u8> for Row {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{:08b}", self.0)
                .replace('1', "#")
                .replace('0', ".")
        )
    }
}

impl BitAnd for Row {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0.bitand(rhs.0))
    }
}

impl BitOrAssign for Row {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0);
    }
}

impl Row {
    const fn is_empty(self) -> bool {
        self.0 == 1
    }
}

impl Debug for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_width: usize = self
            .shape
            .iter()
            .map(|row| 8 - row.0.trailing_zeros())
            .max()
            .unwrap() as usize;
        for row in self.shape.iter().rev() {
            writeln!(f, "{}", &format!("{row:?}")[0..max_width])?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Jet {
    directions: Vec<Direction>,
    index: usize,
}

impl Iterator for Jet {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let o = self.directions[self.index];
        self.index += 1;
        if self.index == self.directions.len() {
            self.index = 0;
        }
        Some(o)
    }
}

impl FromStr for Jet {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            directions: s
                .trim()
                .chars()
                .map(|c| format!("{c}").parse())
                .collect::<color_eyre::Result<Vec<_>>>()?,
            index: 0,
        })
    }
}

#[derive(Clone, Default, Debug)]
pub struct Rocks {
    rocks: Vec<Rock>,
    index: usize,
}

impl Iterator for Rocks {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let o = &self.rocks[self.index];
        self.index += 1;
        if self.index == self.rocks.len() {
            self.index = 0;
        }
        Some(o.clone())
    }
}

#[derive(Default, Debug)]
pub struct Chamber {
    rocks: Rocks,
    jet: Jet,
    grid: Vec<Row>,
    current: Option<(Rock, usize)>,
}

impl Chamber {
    fn new(jet: Jet) -> Self {
        Self {
            rocks: Rocks {
                index: 0,
                rocks: vec![
                    Rock::new(vec![0b0011_1100]),
                    Rock::new(vec![0b0001_0000, 0b0011_1000, 0b0001_0000]),
                    Rock::new(vec![0b0011_1000, 0b0000_1000, 0b0000_1000]),
                    Rock::new(vec![0b0010_0000, 0b0010_0000, 0b0010_0000, 0b0010_0000]),
                    Rock::new(vec![0b0011_0000, 0b0011_0000]),
                ],
            },
            jet,
            grid: vec![0xff.into()],
            ..Default::default()
        }
    }

    #[allow(unused)]
    fn drop_many_rocks(&mut self, amount: usize) {
        let original_rocks = self.rocks.clone();
        let cycle_length = self.rocks.rocks.len() * self.jet.directions.len();
        let mut i = 0;
        let mut previous_cycle_start = 1;
        let mut the_one_before_that = 1;
        loop {
            if i == amount {
                return;
            }
            // if i >= cycle_length && amount - i > cycle_length {
            //     if i == cycle_length {
            //         let new_rock: Vec<Row> = self.grid[1..]
            //             .iter()
            //             .take_while(|row| !row.is_empty())
            //             .map(|row| Row::from(row.0 & 0b1111_1110))
            //             .collect();
            //         self.rocks.rocks = vec![Rock::new(new_rock)];
            //         //println!("{:?}", self.rocks.rocks[0]);
            //         self.rocks.index = 0;

            //     }
            //     previous_cycle_start = self.get_highest_occupied_row() + 1;
            //     self.spawn();
            //     while self.current.is_some() {
            //         self.gravity();
            //     }
            //     i += cycle_length;
                
            //     if i % 1_000_000 == 0 {
            //         println!("{i}");
            //     }
            // } else {
                if self.rocks.rocks.len() == 1 {
                    self.rocks = original_rocks.clone();
                }
                self.drop_rocks(1);
                i += 1;
                if i % cycle_length == 0 && i > 0 && i < amount - cycle_length {
                    // println!("Check for cycles in round {i} out of {amount}");
                    let new_rows: Vec<_> = self.grid[previous_cycle_start..=self.get_highest_occupied_row()].to_vec();
                    if self.grid[1..previous_cycle_start].ends_with(&new_rows) {
                        println!("Found cycle after {i} rounds. From: {previous_cycle_start} to including {}", self.get_highest_occupied_row());
                        println!("{:?}", self.grid[previous_cycle_start]);
                        println!("{:?}", self.grid[self.get_highest_occupied_row()]);
                        
                        while self.get_highest_occupied_row() > self.grid.len() - 1{
                            self.grid.pop();
                        }
                        while i < amount - cycle_length {
                            self.grid.append(&mut new_rows.clone());
                            i += cycle_length;
                        }
                    }
                    if the_one_before_that > 1 && previous_cycle_start > 1 {
                        let needle = &self.grid[the_one_before_that..=previous_cycle_start];
                        // if let Some(u) = self.grid[1..the_one_before_that].windows(needle.len()).position(|window| window == needle) {
                        if self.grid[..(the_one_before_that-11)].ends_with(needle) {
                            // dbg!(u);
                            dbg!(needle);
                            std::fs::write("cycle.txt", format!("{:?}", needle.iter().rev().collect_vec())).unwrap();
                            std::fs::write("chamber.txt", format!("{self}")).unwrap();
                            return;
                        }
                    }
                    // dbg!(previous_cycle_start, self.get_highest_occupied_row());
                    the_one_before_that = previous_cycle_start;
                    previous_cycle_start = self.get_highest_occupied_row();
                }
            // }
        }
    }

    fn drop_rocks(&mut self, amount: usize) {
        let mut amount = amount;
        loop {
            if self.current.is_none() {
                if amount > 0 {
                    self.spawn();
                    amount -= 1;
                } else {
                    return;
                }
            }
            self.tick();
        }
    }

    fn tick(&mut self) {
        if self.current.is_none() {
            self.spawn();
        }
        self.jet();
        self.gravity();
    }

    fn get_highest_occupied_row(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .rev()
            .find(|(_, x)| !x.is_empty())
            .unwrap_or((0, &Row::default()))
            .0
    }

    fn jet(&mut self) {
        let (rock, y) = self.current.as_mut().expect("Cannot be empty");
        let potential_new_rock = match self.jet.next().unwrap() {
            Direction::Right => rock.shr(),
            Direction::Left => rock.shl(),
            _ => unreachable!(),
        };
        if (*y..(*y + rock.height()))
            .all(|idx| self.grid[idx] & potential_new_rock.shape[idx - *y] == Row(0))
        {
            *rock = potential_new_rock;
        }
    }

    fn gravity(&mut self) {
        let (rock, y) = self.current.as_mut().expect("Cannot be empty");
        let new_y = *y - 1;
        if (new_y..(new_y + rock.height()))
            .all(|idx| self.grid[idx] & rock.shape[idx - new_y] == Row(0))
        {
            *y = new_y;
        } else {
            for (idx, row) in rock.shape.iter().enumerate() {
                self.grid[*y + idx] |= *row;
            }
            self.current = None;
        }
    }

    fn spawn(&mut self) {
        let shape = self.rocks.next().unwrap();
        let highest_occupied = self.get_highest_occupied_row();
        for _ in self.grid.len()..(highest_occupied + 4 + shape.height()) {
            self.grid.push(1.into());
        }
        self.current = Some((shape, highest_occupied + 4));
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // if let Some((rock, _)) = self.current.as_ref() {
        //     write!(f, "{rock:?}")?;
        // }
        for (i, row) in self.grid.iter().enumerate().skip(1).rev() {
            let mut static_row = format!("{row:?}");
            if let Some((rock, y)) = self.current.as_ref() && i >= *y && i < *y + rock.height() {
                for (char_index, c) in format!("{:?}", rock.shape[i - *y]).char_indices() {
                    if c == '#' && char_index < 7 {
                        static_row.replace_range(char_index..=char_index, "@");
                    }
                }
            }
            writeln!(f, "|{}|", &static_row[..7])?;
        }
        writeln!(f, "+-------+")?;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;

    #[test]
    fn it_displays_rock() {
        let rock = Rock::new(vec![0b1111_0000]);
        assert_eq!("####\n", format!("{rock:?}"));

        let rock = Rock::new(vec![0b0100_0000, 0b1110_0000, 0b0100_0000]);
        assert_eq!(".#.\n###\n.#.\n", format!("{rock:?}"));

        let rock = Rock::new(vec![0b1110_0000, 0b0010_0000, 0b0010_0000]);
        assert_eq!("..#\n..#\n###\n", format!("{rock:?}"));
    }

    #[test]
    fn it_streams_directions() {
        let mut jet = Jet {
            directions: vec![Direction::Left, Direction::Right],
            ..Default::default()
        };
        assert_eq!(Some(Direction::Left), jet.next());
        assert_eq!(Some(Direction::Right), jet.next());
        assert_eq!(Some(Direction::Left), jet.next());
    }

    #[test]
    fn it_parses_input() {
        let expected = Jet {
            directions: vec![Direction::Left, Direction::Right],
            ..Default::default()
        };
        let input = "<>";
        assert_eq!(expected, Jet::from_str(input).unwrap());
    }

    #[test]
    fn it_spawns_rock() {
        let mut chamber = Chamber::new(Jet::from_str(&read_example("day17.txt")).unwrap());
        chamber.spawn();
        assert_eq!(4, chamber.current.unwrap().1);
    }

    #[test]
    fn it_ticks_once() {
        let mut chamber = Chamber::new(Jet::from_str(&read_example("day17.txt")).unwrap());
        chamber.spawn();
        chamber.jet();
        assert_eq!(
            &(Rock::new(vec![0b0001_1110]), 4),
            chamber.current.as_ref().unwrap()
        );
        chamber.gravity();
        assert_eq!(
            &(Rock::new(vec![0b0001_1110]), 3),
            chamber.current.as_ref().unwrap()
        );
    }
    #[test]
    fn it_simulates_one_piece() {
        let mut chamber = Chamber::new(Jet::from_str(&read_example("day17.txt")).unwrap());
        chamber.tick();
        assert_eq!(
            &(Rock::new(vec![0b0001_1110]), 3),
            chamber.current.as_ref().unwrap()
        );
        chamber.tick();
        assert_eq!(
            &(Rock::new(vec![0b0001_1110]), 2),
            chamber.current.as_ref().unwrap()
        );
        chamber.tick();
        assert_eq!(
            &(Rock::new(vec![0b0001_1110]), 1),
            chamber.current.as_ref().unwrap()
        );
        chamber.tick();
        assert!(chamber.current.is_none());
        assert_eq!(Row::from(0b0011_1101), chamber.grid[1]);
        println!("{chamber}");
    }

    #[test]
    fn it_drops_rocks() {
        let mut chamber = Chamber::new(Jet::from_str(&read_example("day17.txt")).unwrap());
        chamber.drop_rocks(2022);
        
        assert_eq!(3068, chamber.get_highest_occupied_row());
    }

    #[ignore = "Doesn't work yet"]
    #[test]
    fn it_drops_more_rocks() {
        let mut chamber = Chamber::new(Jet::from_str(&read_example("day17.txt")).unwrap());
        chamber.drop_many_rocks(20220);
        std::fs::write("test.txt",format!("{chamber}")).unwrap();
        assert_eq!(30624, chamber.get_highest_occupied_row());
    }

    #[test]
    fn it_drops_ten_rocks() {
        let mut chamber = Chamber::new(Jet::from_str(&read_example("day17.txt")).unwrap());
        chamber.drop_rocks(10);
        chamber.spawn();
        assert_eq!(
            "|.......|
|..@@@@.|
|.......|
|.......|
|.......|
|....#..|
|....#..|
|....##.|
|##..##.|
|######.|
|.###...|
|..#....|
|.####..|
|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+",
            format!("{chamber}").trim()
        );
    }
}
