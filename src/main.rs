extern crate aoc;

use anyhow::Result;
use aoc::*;

fn main() -> Result<()> {
    /*
    let sol1 = aoc01::solve1();
    let sol2 = aoc01::solve2();

    println!("AOC 1.1 - {}", sol1);
    println!("AOC 1.2 - {}", sol2);

    let sol2_1 = aoc02::solve1()?;
    let sol2_2 = aoc02::solve2()?;

    println!("AOC 2.1 - {}", sol2_1);
    println!("AOC 2.2 - {}", sol2_2);
    */
    let sol3_1 = aoc03::solve1();
    let sol3_2 = aoc03::solve2();

    println!("AOC 3.1 - {}", sol3_1);
    println!("AOC 2.2 - {}", sol3_2);
    Ok(())
}
