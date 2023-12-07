use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rayon::prelude::*;

const RADIX: u32 = 10;

// Function that processes each line
fn process_line(line: String) -> u32 {
    // Start processing lines
    let mut processor = line.chars();
    let mut reds = 0;
    let mut greens = 0;
    let mut blues = 0;

    // Skip the game_number
    let _ = processor.by_ref()
        .skip_while(|c| !c.is_digit(RADIX))
        .skip_while(|c| c.is_digit(RADIX));

    // While we have numbers in the line
    while let Some(next_number) = processor.by_ref()
        .skip_while(|c| !c.is_digit(RADIX))
        .take_while(|c| c.is_digit(RADIX))
        .fold(None, |acc, c| 
            acc.map_or_else(
                || Some(c.to_digit(RADIX).unwrap_or(0)),
                |num| Some(num * RADIX + c.to_digit(RADIX).unwrap_or(0))
            )
        )
    {
        // Match the first letter of the word after the next number and return 0 if the number is too high for that color
        match processor.next() {
            Some('r') => {reds = max(reds, next_number)},
            Some('g') => {greens = max(greens, next_number)},
            Some('b') => {blues = max(blues, next_number)},
            _ => (),
        }
    }

    // If nothing was wrong we return the number of reds, greens and blues multiplied together
    reds * greens * blues
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");
 
    // Process the lines and sum the power of sets
    let total_result: u32 = BufReader::new(file)
        .lines()
        .par_bridge()
        .map(|line| line.ok())
        .while_some()
        .map(process_line)
        .sum();

    // Print the final result
    println!("Summed power of sets: {}", total_result);
}