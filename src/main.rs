#![feature(iter_next_chunk)]
#![feature(try_trait_v2)]
#![feature(iter_collect_into)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![feature(let_chains)]
#![feature(map_try_insert)]

mod day01;
mod day02;
pub mod util;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;
    day24::solve();
    day01::solve();
    day02::solve();
    day03::solve()?;
    day04::solve()?;
    day05::solve()?;
    day06::solve()?;
    day07::solve()?;
    day08::solve()?;
    day09::solve()?;
    day10::solve()?;
    day11::solve();
    day12::solve()?;
    day13::solve();
    day14::solve();
    day15::solve();
    day16::solve();
    println!("Day 16 part 2 is skipped because my code is too slow to just run it everytime");
    day17::solve()?;
    day18::solve();
    day19::solve(); 
    day20::solve();
    day21::solve();
    day22::solve();
    day23::solve();

    Ok(())
}
