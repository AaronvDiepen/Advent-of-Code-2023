use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");
    let reader = BufReader::new(file);

    // Create a single parser for all the lines
    let mut parser = reader.lines();

    // Read the seed ranges as (start, end)
    let mut seeds: Vec<(i64, i64)> = parser.by_ref()
        .next()
        .unwrap()
        .expect("Could not get line with seeds")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect::<Vec<(i64, i64)>>();

    seeds.sort_by_key(|&(start, _)| start);

    // Initialize the vector to store grouped transfer functions
    // groups of transfer functions containing (dest_start, dest_end, offset)
    let mut transfer_functions_grouped: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    // While there are lines to be parsed
    while parser.next().is_some() {
        // Get the transfer functions of a block by parsing all the sequential lines with at least one number
        // and add it to the list of transfer functions groups
        transfer_functions_grouped.push(parser.by_ref()
            .skip_while(|line| line.as_ref().map_or(true, |s| !s.chars().any(char::is_numeric)))
            .take_while(|line| line.as_ref().map_or(false, |s| s.chars().any(char::is_numeric)))
            .map(|s| {
                // Map a single line to a tuple (dest_start, dest_end, offset)
                let transfer_function = s.expect("Could not get lines")
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect::<Vec<i64>>();
                    (transfer_function[0], transfer_function[0] + transfer_function[2] - 1, transfer_function[1] as i64 - transfer_function[0] as i64)
            })
            .collect());
    }

    // Sort transfer functions based on start value of destination range, last in increasing order others in decreasing order
    transfer_functions_grouped.iter_mut().rev().next().expect("Could not get last transfer function").sort_by_key(|(dest_start, _, _)| *dest_start);
    transfer_functions_grouped.iter_mut().rev().skip(1).for_each(|transfer_functions| {
        transfer_functions.sort_by_key(|(dest_start, _, _)| -dest_start);
    });

    // We traverse backward from the lowest outputs first to find the best input value that is a seed.

    // Create a new empty queue storing value ranges, output_value and what type of value i.e. seed, soil, etc.
    // ((start_range, end_range), lowest_output, type)
    let mut possible_output_queue: VecDeque<((i64, i64), i64, usize)> = VecDeque::new();

    // Convert possible output values to ranges
    let mut output_last = 0;
    let output_group = transfer_functions_grouped.len() - 1;

    // Initialize the queue with the first transfer function ranges that map to output
    for (dest_start, dest_end, offset) in transfer_functions_grouped[output_group].iter() {
        // If the transfer function starts after the last output add the range between it
        if output_last < *dest_start{
            possible_output_queue.push_back(((output_last, dest_start - 1), output_last, output_group));
        }
        // Add the range spanned by the transfer function
        possible_output_queue.push_back(((dest_start + offset, dest_end + offset), *dest_start, output_group));
        // Note the last number we transfered from
        output_last = dest_end + 1;
    }

    // While we have entries in the queue and have not found a valid input seed explore the lowest possible solution
    while let Some(((output_start, output_end), lowest_output, output_group)) = possible_output_queue.pop_front() {
        // If a possible output is a potential seed
        if output_group == 0 {
            // Check if the potential seed exists
            for &(seed_start, seed_end) in &seeds {
                // If the seed range overlaps the output range then we have found a valid seed
                if output_start < seed_end && output_start > seed_start {
                    // Print the final result
                    println!("Lowest location number: {:?}", lowest_output);
                    exit(0);
                }

                // If the seed range starts after the potential seed range we either have a valid seed or can assume it is not valid
                if output_start < seed_start {
                    // If the seed range starts before the potential seed range ends we have a valid seed somewhere in the range
                    if output_end > seed_start {
                        println!("Lowest location number: {:?}", lowest_output + seed_start - output_start);
                        exit(0);
                    }
                    // No valid seed exists
                    break;
                }
            }
            // Keep solving entries in the queue to find the next lowest potential seed
            continue;
        }

        // Store the end_point of the next range that we want to convert and add to the queue
        output_last = output_end;

        // Traverse the relevant tranfser functions backwards and add the highest ranges first so the lowest are at the front of the queue
        for (dest_start, dest_end, offset) in transfer_functions_grouped[output_group - 1].iter() {
            // If there are no valid transfers left stop trying to use transfer functions to convert the range
            if output_start > *dest_end {
                break;
            }
            // If we have a valid transfer range split use it to transfer the current range to the lower type
            if *dest_start < output_last {                
                // If the range we want to transfer cannot be fully converted using the transfer function add a direct mapping 
                if output_last > *dest_end {
                    possible_output_queue.push_front(((dest_end + 1, output_last), lowest_output + dest_end + 1 - output_start, output_group - 1));
                    output_last = *dest_end;
                }
                // If the range we want to transfer is shorter than the range of the transfer function only transfer that part, we are done
                if output_start > *dest_start {
                    possible_output_queue.push_front(((output_start + offset, output_last + offset), lowest_output, output_group - 1));
                    output_last = output_start - 1;
                    break;
                }
                
                // Transfer the part of the range we want to transfer using this transfer function, for the remained use the other transfer functions
                possible_output_queue.push_front(((dest_start + offset, output_last + offset), lowest_output + dest_start - output_start, output_group - 1));
                output_last = dest_start - 1;
            }
        }

        // If we could not transfer a part of the range we want to transfer using the transfer functions then add a direct mapping for it
        if output_last > output_start {
            possible_output_queue.push_front(((output_start, output_last), lowest_output, output_group - 1));
        }
    }
}