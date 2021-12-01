use itertools::izip;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>  {
    println!("Day 1: Sonar Sweep");

    let input = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: {} <filename>", env::args().next().unwrap());
        std::process::exit(0);
    });

    let input = File::open(input)?;

    let depths = io::BufReader::new(input).lines();

    let depths = depths
                    .map(|d| d.unwrap().parse::<u32>().unwrap_or_default())
                    .collect::<Vec<u32>>();

    let increased = depths
                        .iter()
                        .zip(depths.iter().skip(1))
                        .filter(|(d1, d2)| d2.gt(d1))
                        .count();

    println!("Part 1: Depth measurements increased {} times", increased);

    let depths = izip!(depths.iter(), depths.iter().skip(1), depths.iter().skip(2))
                    .map(|(d1, d2, d3)| d1 + d2 + d3)
                    .collect::<Vec<u32>>();

    let increased = depths
                        .iter()
                        .zip(depths.iter().skip(1))
                        .filter(|(d1, d2)| d2.gt(d1))
                        .count();

    println!("Part 2: Depth sum measurements increased {} times", increased);

    Ok(())
}
