use std::fs::File;
use std::io::{BufRead, BufReader};

use rayon::prelude::*;

const RADIX: u32 = 10;
const REDS: u32 = 12;
const GREENS: u32 = 13;
const BLUES: u32 = 14;

// Function that processes each line
fn process_line(line: String) -> u32 {
    // Start processing lines
    let mut processor = line.chars();

    // Get the game number
    let game_number: u32 = processor.by_ref()
        .skip_while(|c| !c.is_digit(RADIX))
        .take_while(|c| c.is_digit(RADIX))
        .fold(0, |acc, c| acc * RADIX + c.to_digit(RADIX).unwrap_or(0));

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
            Some('r') => {if next_number > REDS {return 0}},
            Some('g') => {if next_number > GREENS {return 0}},
            Some('b') => {if next_number > BLUES {return 0}},
            _ => (),
        }
    }

    // If nothing was wrong we return the game number to add to the total
    game_number
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");

    // Process the lines and sum the possible games
    let total_result: u32 = BufReader::new(file)
        .lines()
        .par_bridge()
        .map(|line| line.ok())
        .while_some()
        .map(process_line)
        .sum();
    
    // Print the final result
    println!("Summed possible games: {}", total_result);
}