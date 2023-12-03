extern crate aoc;

use anyhow::Result;
use aoc::*;

fn main() -> Result<()> {
    let sol1 = aoc1::solve1();
    let sol2 = aoc1::solve2();

    println!("AOC 1.1 - {}", sol1);
    println!("AOC 1.2 - {}", sol2);

    let sol2_1 = aoc2::solve1()?;
    let sol2_2 = aoc2::solve2()?;

    println!("AOC 2.1 - {}", sol2_1);
    println!("AOC 2.2 - {}", sol2_2);
    Ok(())
}
