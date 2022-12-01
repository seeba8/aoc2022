use anyhow::{Context, Result};

pub fn solve() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!()
            .split_once("::")
            .context("Cannot split filename")?
            .1
    ))?;
    println!("{:?}", part1(&input));
    println!("{:?}", part2(&input));
    Ok(())
}

fn get_elves(input: &str) -> Vec<usize> {
    input
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
    use super::{part1, part2};

    #[test]
    fn it_finds_the_elf_with_most_calories() {
        let input = std::fs::read_to_string("examples/day01.txt").unwrap();
        assert_eq!(24000, part1(&input));
    }

    #[test]
    fn it_finds_the_top_3_elves() {
        let input = std::fs::read_to_string("examples/day01.txt").unwrap();
        assert_eq!(45000, part2(&input));
    }
}
