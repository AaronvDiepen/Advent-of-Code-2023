use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");
    let reader = BufReader::new(file);

    // Create a single parser for all the lines
    let mut parser = reader.lines();

    // Read seeds
    let mut seeds: Vec<u64> = parser.by_ref()
        .next()
        .unwrap()
        .expect("Could not get line with seeds")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // While there are lines to be parsed
    while parser.next().is_some() {
        // Get the transfer functions of a block by parsing all the sequential lines with at least one number
        let transfer_functions = parser.by_ref()
            .skip_while(|line| line.as_ref().map_or(true, |s| !s.chars().any(char::is_numeric)))
            .take_while(|line| line.as_ref().map_or(false, |s| s.chars().any(char::is_numeric)))
            .map(|s| s.expect("Could not get lines").split_whitespace().filter_map(|s| s.parse().ok()).collect::<Vec<u64>>());

        // Use an array to denote whether we already used a transfer function on a value
        let mut transfered = vec![0; seeds.len()];

        // Use the transfer functions on the values
        for transfer_function in transfer_functions  {
            for i in 0..seeds.len() {
                // If we haven't transfered this value yet and it is within range of the transfer function
                if transfered[i] == 0 && seeds[i] >= transfer_function[1] && seeds[i] < transfer_function[1] + transfer_function[2] {
                    // Transfer the value using the transfer function
                    seeds[i] = seeds[i] + transfer_function[0] - transfer_function[1];

                    // Mark it as transfered so we don't transfer a value twice
                    transfered[i] = 1;
                }
            }
        }
    }

    // Get the lowest location number
    let result = seeds.iter().min().expect("Couldn't find minimum");

    // Print the final result
    println!("Lowest location number: {}", result);
}