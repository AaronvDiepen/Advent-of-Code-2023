use std::fs::File;
use std::io::{BufRead, BufReader};

use rayon::prelude::*;
use num_integer::IterBinomial;

// Function that processes each line
fn process_line(line: String) -> i64 {
    let history: Vec<i64> = line.split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect();

    // Use the row of pascals triangle that is equal to the length of our input to calculate the new entry
    history.iter()
        // Multiply the entry with the values in the corresponding row of pascals triangle
        .zip(IterBinomial::new(first_row.len()))
        .map(|(entry, binomial)| {
            binomial as i64 * entry
        })
        // Combine the values into a new entry
        // We need to allow wrapping since partial sums can get very large
        .enumerate()
        .fold(0, |new_entry, (index, value)| {
            if index & 1 == 0 {
                // For even indexed values we need to add the value to the total
                new_entry.wrapping_add(value)
            } else {
                // For odd indexed values we need to subtract the value from the total
                new_entry.wrapping_sub(value)
            }
        })
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