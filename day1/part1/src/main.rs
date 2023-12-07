use std::fs::File;
use std::io::{BufRead, BufReader};

use rayon::prelude::*;

const RADIX: u32 = 10;

// Function that processes each line
fn process_line(line: String) -> u32 {
    let mut result = 0;

    // Get the first digit and multiply it by the RADIX and add it to result
    result += line.chars()
        .find_map(|c| c.to_digit(RADIX))
        .unwrap() * RADIX;
    
    // Get the last digit and add it to result
    result += line.chars().rev()
        .find_map(|c| c.to_digit(RADIX))
        .unwrap();

    result
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");

    // Process the lines and sum the calibration values
    let total_result: u32 = BufReader::new(file)
        .lines()
        .par_bridge()
        .map(|line| line.ok())
        .while_some()
        .map(process_line)
        .sum();

    // Print the final result
    println!("Summed calibration values: {}", total_result);
}