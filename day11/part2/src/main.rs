use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

// Function that processes each line
fn process_line(line: String) -> Vec<bool> {
    // Create vectors indicating positions of galaxies
    line.chars().map(|c| match c {
        '#' => true,
        _ => false,
    }).collect()
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");

    // Process the lines to vectors contianing possible connections and optional starting positions in each line
    let universe: Vec<Vec<bool>> = BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .map(process_line)
        .collect();

    // Find which columns are empty
    let mut empty_columns = vec![true; universe.len()];
    let mut empty_rows = vec![true; universe[0].len()];
    universe.iter().enumerate().for_each(|(row, universe_row)| {
        universe_row.iter().enumerate().for_each(|(column, &square)| {
            if square {
                empty_columns[column] = false;
                empty_rows[row] = false;
            }
        })
    });

    // Convert grid with galaxies to array with galaxy position
    // While accounting for the extra offset of empty rows and colums
    // 1000000 - 1 = 999999
    let mut galaxies = vec![];
    let mut extra_height = 0;
    universe.iter().enumerate().for_each(|(row, universe_row)| {
        if empty_rows[row] {
            extra_height += 999999;
        }
        let mut extra_width = 0;
        universe_row.iter().enumerate().for_each(|(column, &square)| {
            if empty_columns[column] {
                extra_width += 999999;
            }
            if square {
                galaxies.push((row + extra_height, column + extra_width));
            }
        })
    });

    // Sum distances between all pairs of galaxies
    let total_output: usize = galaxies.iter().combinations(2).map(|galaxies| {
        let mut distance = 0;
        if galaxies[0].0 > galaxies[1].0 {
            distance += galaxies[0].0 - galaxies[1].0;
        } else {
            distance += galaxies[1].0 - galaxies[0].0;
        }
        if galaxies[0].1 > galaxies[1].1 {
            distance += galaxies[0].1 - galaxies[1].1;
        } else {
            distance += galaxies[1].1 - galaxies[0].1;
        }
        distance
    }).sum();

    // Print the final result
    println!("Summed distances between galaxy pairs: {}", total_output);
}
