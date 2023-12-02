use std::collections::{HashMap, HashSet};
use itertools::{Itertools, MinMaxResult};
use std::fmt::Write;
use crate::util::read_input;

pub fn solve() {
    let input = read_input("day23.txt");
    let elves = get_elves(&input);
    let (_, empty_tiles) = tick_and_count(elves, 10);
    println!("Day 23 part 1: {empty_tiles}");
    let elves = get_elves(&input);
    let (_, rounds) = tick_until_static(elves);
    println!("Day 23 part 2: {rounds}");
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Elf {
    x: isize,
    y: isize,
}

impl Elf {
    const fn offset(&self, x: isize, y: isize) -> Self {
        Self { x: self.x + x, y: self.y + y }
    }
    const fn north(&self) -> [Self; 3] {
        [self.offset(0, -1), self.offset(1, -1), self.offset(-1, -1)]
    }
    const fn south(&self) -> [Self; 3] {
        [self.offset(0, 1), self.offset(1, 1), self.offset(-1, 1)]
    }
    const fn west(&self) -> [Self; 3] {
        [self.offset(-1, 0), self.offset(-1, 1), self.offset(-1, -1)]
    }

    const fn east(&self) -> [Self; 3] {
        [self.offset(1, 0), self.offset(1, 1), self.offset(1, -1)]
    }
}


fn tick(elves: &HashSet<Elf>, iteration: usize) -> (HashSet<Elf>, bool) {
    let mut targets: HashMap<Elf, Vec<usize>> = HashMap::new();
    let elves_list = elves.iter().collect_vec();
    for (i, elf) in elves_list.iter().enumerate() {
        let mut alone = true;
        for (x, y) in (-1..=1).cartesian_product(-1..=1) {
            if x == 0 && y == 0 {
                continue;
            }
            if elves_list.contains(&&Elf { x: elf.x + x, y: elf.y + y }) {
                alone = false;
                break;
            }
        }
        if alone {
            targets.try_insert(**elf, vec![i]).unwrap();
        } else {
            let mut target_found = false;
            for check in 0..4 {
                match (iteration + check) % 4 {
                    0 => {
                        if elf.north().iter().all(|e| !elves_list.contains(&e)) {
                            targets.entry(Elf { x: elf.x, y: elf.y - 1 }).and_modify(|e| e.push(i)).or_insert_with(||vec![i]);
                            target_found = true;
                            break;
                        }
                    }
                    1 => {
                        if elf.south().iter().all(|e| !elves_list.contains(&e)) {
                            targets.entry(Elf { x: elf.x, y: elf.y + 1 }).and_modify(|e| e.push(i)).or_insert_with(||vec![i]);
                            target_found = true;
                            break;
                        }
                    }
                    2 => {
                        if elf.west().iter().all(|e| !elves_list.contains(&e)) {
                            targets.entry(Elf { x: elf.x - 1, y: elf.y }).and_modify(|e| e.push(i)).or_insert_with(||vec![i]);
                            target_found = true;
                            break;
                        }
                    }
                    3 => {
                        if elf.east().iter().all(|e| !elves_list.contains(&e)) {
                            targets.entry(Elf { x: elf.x + 1, y: elf.y }).and_modify(|e| e.push(i)).or_insert_with(||vec![i]);
                            target_found = true;
                            break;
                        }
                    }
                    _ => unreachable!()
                }
            }
            if !target_found {
                targets.try_insert(**elf, vec![i]).unwrap();
            }
        }
    }
    let mut result = HashSet::new();
    for (elf, origins) in targets {
        if origins.len() == 1 {
            result.insert(elf);
        } else {
            for u in origins {
                result.insert(*elves_list[u]);
            }
        }
    }
    assert_eq!(result.len(), elves_list.len());
    let same = result == *elves;
    (result, same)
}

fn tick_and_count(mut elves: HashSet<Elf>, rounds: usize) -> (HashSet<Elf>, usize) {
    // println!("{}", to_string(&elves));
    for i in 0..rounds {
        (elves, _) = tick(&elves, i);
        //println!("{}", to_string(&elves));
    }
    let (MinMaxResult::MinMax(x_min, x_max), MinMaxResult::MinMax(y_min, y_max)) = get_dimensions(&elves) else { panic!() };
    let v = (x_max.abs_diff(x_min) + 1) * (y_max.abs_diff(y_min) + 1) - elves.len();
    (elves, v)
}

fn tick_until_static(mut elves: HashSet<Elf>) -> (HashSet<Elf>, usize) {
    let mut i = 0;
    let mut same: bool;
    loop {
        (elves, same) = tick(&elves, i);
        i += 1;
        if same {
            return (elves, i);
        }
    }
}

fn get_dimensions(elves: &HashSet<Elf>) -> (MinMaxResult<isize>, MinMaxResult<isize>) {
    let x = elves.iter().map(|e| e.x).minmax();
    let y = elves.iter().map(|e| e.y).minmax();
    (x, y)
}

#[allow(unused)]
fn to_string(elves: &HashSet<Elf>) -> String {
    let mut result = String::new();
    let (MinMaxResult::MinMax(x_min, x_max), MinMaxResult::MinMax(y_min, y_max)) = get_dimensions(elves) else { panic!() };
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if elves.contains(&Elf { x, y }) {
                write!(result, "#").unwrap();
            } else {
                write!(result, ".").unwrap();
            }
        }
        writeln!(result).unwrap();
    }
    result
}

fn get_elves(input: &str) -> HashSet<Elf> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)|
            line
                .trim()
                .chars()
                .enumerate()
                .filter_map(move |(x, e)| {
                    if e == '#' {
                        Some(Elf { x: isize::try_from(x).unwrap(), y: isize::try_from(y).unwrap() })
                    } else { None }
                }))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::util::read_example;
    use super::*;

    #[test]
    fn it_prints_grid() {
        let expected = r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        let elves = get_elves(&expected);
        assert_eq!(expected, to_string(&elves).trim());
    }

    #[test]
    fn it_ticks() {
        let input = r".....
..##.
..#..
.....
..##.
.....";
        let expected = r"##
..
#.
.#
#.";
        let elves = get_elves(input);
        let (elves, _) = tick(&elves, 0);
        assert_eq!(to_string(&elves).trim_end(), expected);
    }

    #[test]
    fn it_counts_empty_tiles_after_rounds() {
        let input = read_example("day23.txt");
        let elves = get_elves(&input);
        let (elves, empty_tiles) = tick_and_count(elves, 10);
        println!("{}", to_string(&elves));
        assert_eq!(110, empty_tiles);
    }

    #[test]
    fn it_runs_until_it_doesnt() {
        let input = read_example("day23.txt");
        let elves = get_elves(&input);
        let (elves, count) = tick_until_static(elves);
        println!("{}", to_string(&elves));
        assert_eq!(20, count);
    }
}