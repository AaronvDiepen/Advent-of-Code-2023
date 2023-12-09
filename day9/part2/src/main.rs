use std::fs::File;
use std::io::{BufRead, BufReader};

use rayon::prelude::*;
use num_integer::binomial;

// Function that processes each line
fn process_line(line: String) -> i64 {
    let first_row: Vec<i64> = line.split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    // Use the row of pascals triangle that is equal to the length of our input to calculate the new entry
    first_row.iter().enumerate()
        .map(|(i, entry)| {
            if i % 2 == 0 {
                // For even indexed entries we need to add the entry times the next value from pascals triangle
                binomial(first_row.len(), i + 1) as i64 * entry
            } else {
                // For odd indexed entries we need to subtract the entry times the next value from pascals triangle
                -1 * binomial(first_row.len(), i + 1) as i64 * entry
            }
        })
        // We need to allow wrapping since partial sums can get very large
        .fold(0, |acc, num| acc.wrapping_add(num))
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");

    // Process the lines and sum the result
    let total_result: i64 = BufReader::new(file).lines()
        .par_bridge()
        .map(|line| line.ok())
        .while_some()
        .map(process_line)
        .sum();


    // Print the final result
    println!("Summed extrapolated values: {:?}", total_result);
}