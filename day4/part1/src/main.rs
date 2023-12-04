use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

// Function that processes each line
fn process_line(line: &str) -> u32 {
    // Extract the numbers on the left and right of the pipe
    let numbers: Vec<_> = line.split('|').collect();

    // Extract individual numbers from the left part into an hashset
    let left_numbers: HashSet<_> = numbers[0]
        .split_whitespace()
        .skip(2) // Skip Card number
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    // Count the number of matches on the right by filtering out number that are not on the left and counting the sum.
    let count = numbers[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|num| left_numbers.contains(num))
        .count();

    if count == 0 {
        0
    } else {
        1 << (count - 1)
    }
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");
    let reader = BufReader::new(file);

    // Create a thread-safe reader
    let thread_safe_reader = Arc::new(Mutex::new(reader));

    // Create a vector to store the handles to the spawned threads
    let mut handles = vec![];

    // Spawn threads
    for _ in 0..num_cpus::get() {
        let thread_safe_reader_clone = Arc::clone(&thread_safe_reader);

        // Spawn a new thread
        let handle = thread::spawn(move || {
            // Inside the thread, each thread processes a portion of the input
            let mut result = 0;

            // Lock the reader and read lines
            while let Ok(mut locked_reader) = thread_safe_reader_clone.lock() {
                let mut line = String::new();

                // Read a line from the reader
                match locked_reader.read_line(&mut line) {
                    Ok(0) => break, // End of file
                    Ok(_) => result += process_line(&line),
                    Err(e) => eprintln!("Error reading line: {}", e),
                }
            }
            return result
        });

        // Save the handle to the vector
        handles.push(handle);
    }

    // Wait for all threads to finish and collect results
    let mut total_result = 0;
    for handle in handles {
        total_result += handle.join().unwrap();
    }

    // Print the final result
    println!("Summed card scores: {}", total_result);
}