use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

const RADIX: u32 = 10;

// Function that processes each line
fn process_line(line: &str) -> u32 {
    let mut result = 0;

    // Get the first digit and multiply it by the RADIX and add it to result
    line.chars()
        .find(|c| c.is_digit(RADIX))
        .and_then(|c| c.to_digit(RADIX))
        .map(|a| result += a * RADIX);
    
    // Get the last digit and add it to result
    line.chars().rev().
        find(|c| c.is_digit(RADIX))
        .and_then(|c| c.to_digit(RADIX))
        .map(|b| result += b);

    result
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
    println!("Summed calibration values: {}", total_result);
}