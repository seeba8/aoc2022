use anyhow::Ok;

mod day01;
mod day02;
pub mod util;

fn main() -> anyhow::Result<()>{
    day01::solve()?;
    day02::solve()?;
    Ok(())
}
