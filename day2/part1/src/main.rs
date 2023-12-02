use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

const RADIX: u32 = 10;
const REDS: u32 = 12;
const GREENS: u32 = 13;
const BLUES: u32 = 14;

// Function that processes each line
fn process_line(line: &str) -> u32 {
    // Start processing lines
    let mut processor = line.chars();

    // Get the game number
    let game_number: u32 = processor.by_ref()
        .skip_while(|c| !c.is_digit(RADIX))
        .take_while(|c| c.is_digit(RADIX))
        .fold(0, |acc, c| acc * RADIX + c.to_digit(RADIX).unwrap_or(0));

    // While we have numbers in the line
    while let Some(next_number) = processor.by_ref()
        .skip_while(|c| !c.is_digit(RADIX))
        .take_while(|c| c.is_digit(RADIX))
        .fold(None, |acc, c| 
            acc.map_or_else(
                || Some(c.to_digit(RADIX).unwrap_or(0)),
                |num| Some(num * RADIX + c.to_digit(RADIX).unwrap_or(0))
            )
        )
    {
        // Match the first letter of the word after the next number and return 0 if the number is too high for that color
        match processor.next() {
            Some('r') => {if next_number > REDS {return 0}},
            Some('g') => {if next_number > GREENS {return 0}},
            Some('b') => {if next_number > BLUES {return 0}},
            _ => (),
        }
    }

    // If nothing was wrong we return the game number to add to the total
    game_number
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
    println!("Summed possible games: {}", total_result);
}