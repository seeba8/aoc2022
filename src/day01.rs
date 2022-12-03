use crate::util::read_input;

pub fn solve() {
    let input = read_input("day01.txt");
    println!("Day 01 part 1: {:?}", part1(&input));
    println!("Day 01 part 2: {:?}", part2(&input));
}

fn get_elves(input: &str) -> Vec<usize> {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .collect()
}

fn part1(input: &str) -> usize {
    get_elves(input).into_iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut elves = get_elves(input);
    elves.sort_unstable_by(|a, b| b.cmp(a)); // reverse sort
    elves.into_iter().take(3).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::{part1, part2};

    #[test]
    fn it_finds_the_elf_with_most_calories() {
        let input = read_example("day01.txt");
        assert_eq!(24000, part1(&input));
    }

    #[test]
    fn it_finds_the_top_3_elves() {
        let input = read_example("day01.txt");
        assert_eq!(45000, part2(&input));
    }
}
