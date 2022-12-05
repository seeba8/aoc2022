#![feature(iter_next_chunk)]
#![feature(try_trait_v2)]
#![feature(iter_collect_into)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
mod day01;
mod day02;
pub mod util;
mod day03;
mod day04;
mod day05;

fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;
    day01::solve();
    day02::solve();
    day03::solve()?;
    day04::solve()?;
    day05::solve()?;
    Ok(())
}