use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

use num::integer::lcm;
use rayon::prelude::*;

const A_ASCII: usize = 'A' as usize;
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

    // Parse the lines to hands
    let mut parser = BufReader::new(file).lines();

    // Get the stepping function and parse it to 0 if L and 1 if R
    let binding = parser.by_ref()
        .next()
        .unwrap()
        .expect("Could not get first line");
    let steps = binding.chars()
        .map(direction_to_binary);

    // Get the network nodes and parse them to an array with left and right children
    // Simultaniously get our start nodes
    let network_temp = Mutex::new([[0; 2]; END_NODE+1]);
    let start_nodes_temp = Mutex::new(Vec::new());
    parser.skip(1)
        .par_bridge()
        .map(|line| line.ok())
        .while_some()
        .map(process_line)
        .for_each(|(name, directions)| {
            // Store the directions in the network
            network_temp.lock().unwrap()[name] = directions;
            // If this node ends in A add it as a start node
            if name % 26 == 0 {
                start_nodes_temp.lock().unwrap().push(name);
            }
    });
    let network = network_temp.lock().unwrap();

    // Traverse the network according to steps until we reach the END nodes
    let start_nodes = start_nodes_temp.lock().unwrap().clone();
    let mut step_counts = vec!(0; start_nodes.len());
    for (i, node) in start_nodes.iter().enumerate() {
        let mut step_count = 0;
        let mut current_node = *node;
        for direction in steps.clone().cycle() {
            // Take a step
            step_count += 1;
            current_node = network[current_node][direction];
            if current_node % 26 == 25 {
                step_counts[i] = step_count as u64;
                break;
            }
        }
    }

    // Use Lowest Common Multiple to find step count that reaches all exit nodes
    let step_count = step_counts.iter()
        .fold(1, |current, step_count| lcm(current, *step_count));
    
    // Print the final result
    println!("Number of steps required: {}", step_count);
}