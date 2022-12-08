use std::collections::HashSet;

use crate::util::read_input;

pub fn solve() -> color_eyre::Result<()>{
    let input = read_input("day06.txt");
    println!("Day 06 part 1: {}", find_marker(&input, 4)?);
    println!("Day 06 part 2: {}", find_marker(&input, 14)?);
    Ok(())
}

fn find_marker(input: &str, length: usize) -> color_eyre::Result<usize> {
    // Rust can't window over chars for some reason without Itertools.
    // Therefore we iterate over the naked bytes since we know the input is just ascii.
    for (idx, window) in input.as_bytes().windows(length).enumerate() {
        if window.iter().copied().collect::<HashSet<_>>().len() == length {
            return Ok(idx + length);
        }
    }
    Err(color_eyre::eyre::eyre!("Did not find marker"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_finds_marker() {
        assert_eq!(7, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4).unwrap());
        assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap());
        assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap());
        assert_eq!(10, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap());
        assert_eq!(11, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap());
    }

    #[test]
    fn it_finds_long_marker() {
        assert_eq!(19, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap());
        assert_eq!(23, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap());
        assert_eq!(23, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap());
        assert_eq!(29, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap());
        assert_eq!(26, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap());
    }
}