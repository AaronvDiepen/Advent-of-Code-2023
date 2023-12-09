use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

use rayon::prelude::*;

const A_ASCII: usize = 'A' as usize;
const START_NODE: usize = 0; //sequence_to_number("AAA");
const END_NODE: usize = 17575; //sequence_to_number("ZZZ");

// Parse a sequence like AAB to 1 and ABA to 26
fn sequence_to_number(sequence: &str) -> usize {
    sequence.chars()
        .map(|c| c as usize - A_ASCII)
        .fold(0, |output, c| output * 26 + c)
}

// Function that processes each line
fn process_line(line: String) -> (usize, [usize; 2]) {
    let name = sequence_to_number(line.get(0..3).unwrap());
    let left = sequence_to_number(line.get(7..10).unwrap());
    let right = sequence_to_number(line.get(12..15).unwrap());
    
    (name, [left, right])
}

// Maps L to 0 and R to 1
fn direction_to_binary(direction: char) -> usize {
    if direction == 'L' {
        0
    } else {
        1
    }
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");

    // Create a parser to parse the file
    let mut parser = BufReader::new(file).lines();

    // Get the stepping function and parse it to 0 if L and 1 if R
    let binding = parser.by_ref()
        .next()
        .unwrap()
        .expect("Could not get first line");
    let steps = binding.chars()
        .map(direction_to_binary);

    // Get the network nodes and parse them to an array with left and right children
    let network_temp = Mutex::new([[0; 2]; END_NODE+1]);
    parser.skip(1)
        .par_bridge()
        .map(|line| line.ok())
        .while_some()
        .map(process_line)
        .for_each(|(name, directions)| {
            // Store the directions in the network
            network_temp.lock().unwrap()[name] = directions;
        });
    let network = network_temp.lock().unwrap();

    // Traverse the network according to steps until we reach the END node
    let mut current_node = START_NODE;
    let mut step_count = 0;
    for direction in steps.cycle() {
        current_node = network[current_node][direction];
        step_count += 1;
        // If we reach the exit print the number of steps and stop
        if current_node == END_NODE {
            println!("Number of steps required: {}", step_count);
            break;
        }
    }
}