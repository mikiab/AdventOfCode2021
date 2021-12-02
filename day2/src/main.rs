use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>  {
    println!("Day 2: Dive!");

    let input = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: {} <filename>", env::args().next().unwrap());
        std::process::exit(0);
    });

    let input = File::open(input)?;

    let commands = io::BufReader::new(input)
                        .lines()
                        .map(|line| {
                            let mut line = line.as_ref().unwrap().split_ascii_whitespace();
                            (
                                line.next().unwrap().chars().next().unwrap(), 
                                line.next().unwrap().parse::<i32>().unwrap()
                            )
                        })
                        .collect::<Vec<_>>();

    let (depth, h_pos) = commands
                            .iter()
                            .fold((0, 0), |(mut depth, mut h_pos), (cmd, value)| {
                                match cmd {
                                    'd' => depth += value,
                                    'u' => depth -= value,
                                    'f' => h_pos += value,
                                     _  => panic!("Unexpected value.")
                                }
                                (depth, h_pos)
                            });


    println!("Part 1: horizontal position x depth = {}", h_pos * depth);

    let (depth, h_pos, _) = commands
                                .iter()
                                .fold((0, 0, 0), |(mut depth, mut h_pos, mut aim), (cmd, value)| {
                                    match cmd {
                                        'd' => aim += value,
                                        'u' => aim -= value,
                                        'f' => {
                                            h_pos += value;
                                            depth += value * aim; 
                                        }
                                        _ => panic!("Unexpected value.")
                                    }
                                    (depth, h_pos, aim)
                                });


    println!("Part 2: horizontal position x depth = {}", h_pos * depth);

    Ok(())
}
