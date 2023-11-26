use std::fmt::Write;
use std::str::FromStr;

use crate::util::read_input;

pub fn solve() {
    const ENCRYPTION_KEY: isize = 811_589_153;
    let input = read_input("day20.txt");
    let mut f = File::from_str(&input).unwrap();
    f.mix();
    println!("Day 20 part 1: {}", f.get_sum_of_grove_coordinates());
    let mut f = File::from_str(&input).unwrap();
    for v in &mut f.0 {
        v.value *= ENCRYPTION_KEY;
    }
    for _ in 0..10 {
        f.mix();
    }
    println!("Day 20 part 2: {}", f.get_sum_of_grove_coordinates());
}

#[derive(Clone, PartialEq, Eq)]
pub struct Number {
    prev: usize,
    next: usize,
    value: isize,
    is_head_for_print: bool,
}

pub struct File(Vec<Number>);

impl FromStr for File {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.trim().lines().count();
        let mut numbers: Vec<Number> = s
            .trim()
            .lines()
            .enumerate()
            .map(|(index, value)| Number {
                prev: index.saturating_sub(1),
                next: index.saturating_add(1),
                value: value.parse().unwrap(),
                is_head_for_print: false,
            })
            .collect();
        numbers[0].prev = length - 1;
        numbers[length - 1].next = 0;
        numbers[0].is_head_for_print = true;
        Ok(Self(numbers))
    }
}

impl ToString for File {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut idx = self
            .0
            .iter()
            .enumerate()
            .find(|(_, p)| p.is_head_for_print)
            .unwrap()
            .0;
        let start = idx;
        write!(s, "{}", self.0[idx].value).unwrap();
        idx = self.0[idx].next;
        while idx != start {
            write!(s, ", {}", self.0[idx].value).unwrap();
            idx = self.0[idx].next;
        }
        s
    }
}

impl File {
    fn step(&self, from_index: usize, num_steps: isize, skip_self: bool) -> usize {
        let mut num_steps = num_steps;
        if num_steps == 0 {
            return from_index;
        }
        let is_negative = num_steps < 0;
        if is_negative {
            num_steps *= -1;
        }
        // we should be able to do some modulo
        let num_steps = if skip_self {
            num_steps % (isize::try_from(self.0.len()).unwrap() - 1)
        } else {
            num_steps % isize::try_from(self.0.len()).unwrap()
        };
        if num_steps == 0 {
            return self.0[from_index].prev;
        }
        let mut res = from_index;
        if is_negative {
            for _ in 0..num_steps {
                res = self.0[res].prev;
                if skip_self && res == from_index {
                    res = self.0[res].prev;
                }
            }
        } else {
            for _ in 0..num_steps {
                res = self.0[res].next;
                if skip_self && res == from_index {
                    res = self.0[res].next;
                }
            }
        }
        res
    }

    pub fn mix(&mut self) {
        // println!("{}",self.to_string());
        for i in 0..self.0.len() {
            self.move_number(i);
        }
        // println!("{}",self.to_string());
    }

    fn move_number(&mut self, index: usize) {
        let mut future_index = self.step(index, self.0[index].value, true);
        if self.0[index].value < 0 {
            // fix off-by-one
            future_index = self.0[future_index].prev;
        }
        if self.0[index].value == 0 {
            return;
        }
        let my_prev = self.0[index].prev;
        let my_next = self.0[index].next;
        self.0[my_prev].next = self.0[index].next;
        self.0[my_next].prev = self.0[index].prev;
        if self.0[index].is_head_for_print {
            self.0[my_next].is_head_for_print = true;
            self.0[index].is_head_for_print = false;
        }

        let future_next = self.0[future_index].next;
        self.0[index].next = future_next;
        self.0[future_next].prev = index;
        self.0[index].prev = future_index;
        self.0[future_index].next = index;
    }

    pub fn get_sum_of_grove_coordinates(&self) -> isize {
        // get index of value 0:
        let idx_zero = self
            .0
            .iter()
            .enumerate()
            .find(|(_, p)| p.value == 0)
            .unwrap()
            .0;
        self.0[self.step(idx_zero, 1000, false)].value
            + self.0[self.step(idx_zero, 2000, false)].value
            + self.0[self.step(idx_zero, 3000, false)].value
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::{read_example, read_input};

    use super::*;

    #[test]
    fn it_mixes_example() {
        let input = read_example("day20.txt");
        let mut f = File::from_str(&input).unwrap();
        f.mix();
        assert_eq!(String::from("1, 2, -3, 4, 0, 3, -2"), f.to_string());
    }

    #[test]
    fn it_finds_grove_coordinates() {
        let input = read_example("day20.txt");
        let mut f = File::from_str(&input).unwrap();
        f.mix();
        assert_eq!(3, f.get_sum_of_grove_coordinates());
    }

    #[test]
    fn it_gets_nth_value() {
        let input = read_example("day20.txt");
        let mut f = File::from_str(&input).unwrap();
        f.mix();
        let idx_zero =
            f.0.iter()
                .enumerate()
                .find(|(_, p)| p.value == 0)
                .unwrap()
                .0;
        assert_eq!(4, f.0[f.step(idx_zero, 1000, false)].value);
        assert_eq!(-3, f.0[f.step(idx_zero, 2000, false)].value);
        assert_eq!(2, f.0[f.step(idx_zero, 3000, false)].value);
    }

    #[test]
    fn it_solves_part1() {
        let input = read_input("day20.txt");
        let mut f = File::from_str(&input).unwrap();
        f.mix();
        assert_eq!(13883, f.get_sum_of_grove_coordinates());
    }

    #[test]
    fn it_can_do_ten_times_too() {
        const ENCRYPTION_KEY: isize = 811589153;
        let input = read_example("day20.txt");
        let mut f = File::from_str(&input).unwrap();
        for v in &mut f.0 {
            v.value *= ENCRYPTION_KEY;
        }
        for _ in 0..10 {
            f.mix();
        }
        let idx_zero =
            f.0.iter()
                .enumerate()
                .find(|(_, p)| p.value == 0)
                .unwrap()
                .0;
        assert_eq!(811589153, f.0[f.step(idx_zero, 1000, false)].value);
        assert_eq!(2434767459, f.0[f.step(idx_zero, 2000, false)].value);
        assert_eq!(-1623178306, f.0[f.step(idx_zero, 3000, false)].value);
        assert_eq!(1623178306, f.get_sum_of_grove_coordinates());
    }
    #[test]
    fn it_skips_step_positive() {
        let input = read_example("day20.txt");
        let f = File::from_str(&input).unwrap();
        assert_eq!(4, f.step(0, 10, true));
        assert_eq!(4, f.step(0, 16, true));
        assert_eq!(4, f.step(0, 22, true));
    }

    #[test]
    fn it_can_do_ten_times_too2() {
        let input = read_input("day20.txt");
        let mut f = File::from_str(&input).unwrap();
        const ENCRYPTION_KEY: isize = 811589153;
        for v in &mut f.0 {
            v.value *= ENCRYPTION_KEY;
        }
        for _ in 0..10 {
            f.mix();
        }
        assert_eq!(19185967576920, f.get_sum_of_grove_coordinates());
    }
}
