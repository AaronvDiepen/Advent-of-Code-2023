use std::fs::{metadata, File};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;

const RADIX: u32 = 10;

/// Function that processes each line
fn process_line(line: &str, line_previous: &str, line_next: &str) -> u32 {
    let mut result = 0;
    let mut number = 0;
    let mut start = 0;
    let mut preceded_by_symbol = false;

    // Iterate over subsequent digits to find the end position
    for (i, next_char) in line.char_indices() {
        if next_char.is_digit(RADIX) {
            if number == 0 {
                // If we don't have a number this is our new number
                number = next_char.to_digit(RADIX).unwrap();
                // Mark where we found it
                start = i;
            } else {
                // If we have a number add this digit behind it
                number = number * RADIX + next_char.to_digit(RADIX).unwrap();
            }
        } else {
            // If this is not a digit and we have a number
            if number > 0 {
                // Check the adjacent characters for symbols.
                if preceded_by_symbol {
                    // Add the number if there is a symbol in front of the number
                    result += number;
                } else if next_char != '.' && next_char != '\n' {
                    // Add the number if there is a symbol behind the number
                    result += number;
                } else {
                    // Check the characters on the previous and next line from the number for symbols.
                    let check_length = i - start + ((next_char != '\n') as usize) + ((start != 0) as usize);
                    if (!line_previous.is_empty() && line_previous.chars().skip(start - ((start != 0) as usize)).take(check_length).find(|&c| c != '.').is_some())
                        || (!line_next.is_empty() && line_next.chars().skip(start - ((start != 0) as usize)).take(check_length).find(|&c| c != '.').is_some()) {
                            result += number;
                    }
                }
            }
            
            // Mark whether the previous char was a symbol (can't be a digit due to earlier if statement)
            preceded_by_symbol = next_char != '.';

            // Reset the number
            number = 0;
        }
    }

    result
}

fn main() {
    // Extract the file length from the metadata
    let file_length: u64 = metadata("input").expect("Could not get input file metadata").len().try_into().unwrap();

    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");
    let reader = BufReader::new(file);

    // Get the first line from the input
    if let Some(Ok(first_line)) = reader.lines().next() {
        // Get the line length of the first line including newline
        let line_length: u64 = first_line.len() as u64 + 1;

        // Create a vector to store the handles to the spawned threads
        let mut handles = vec![];

        // Get the number of cpus
        let num_cpus: u64 = num_cpus::get().try_into().unwrap();

        // Calculate number of lines and split them acros the number of cpus
        let num_lines = file_length / line_length;
        let num_lines_per_thread = num_lines / num_cpus;
        let remainder = num_lines % num_cpus;

        // Spawn threads
        for i in 0..num_cpus {
            // Open the file again for each thread
            let thread_file = File::open("input").expect("Could not open file \"input\" relative to program");
            let mut thread_reader = BufReader::new(thread_file);

            // Spawn a new thread
            let handle = thread::spawn(move || {
                // How many lines this thread should process
                let num_lines_to_read = num_lines_per_thread + ((i + 1 == num_cpus) as u64) * remainder;
                if num_lines_to_read == 0 {
                    return 0
                }

                // Inside the thread, each thread processes a portion of the input
                let mut result = 0;
                
                // Create a reader at the line before the first line we should handle or the first line if we are processing the first line
                if let Err(err) = thread_reader.seek(
                    SeekFrom::Start(
                        (i * num_lines_per_thread - ((i != 0 && num_lines_per_thread > 0) as u64))
                        * line_length)) {
                    eprintln!("Error seeking position: {}", err);
                }

                // Create storage for next, current and previous line.
                let mut line_prev;
                let mut line_curr = String::new();
                let mut line_next = String::new();

                // Only populate previous line if we aren't handling the first line.
                if i != 0 && num_lines_per_thread > 0 {
                    thread_reader.read_line(&mut line_curr).expect("Could not read line");
                }

                // Populate the next line
                thread_reader.read_line(&mut line_next).expect("Could not read line");

                // Process all lines
                for j in 0..num_lines_to_read {
                    // Move the lines to their new place
                    line_prev = line_curr;
                    line_curr = line_next;

                    // Read the next line if there is one.
                    if j + 1 != num_lines_to_read || i + 1 != num_cpus {
                        line_next = String::new();
                        thread_reader.read_line(&mut line_next).expect("Could not read line");
                    } else {
                        line_next = String::new();
                    }

                    result += process_line(&line_curr, &line_prev, &line_next);
                }

                result
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
        println!("Summed gear ratios: {}", total_result);
    } else {
        println!("The input file is empty");
    }
}
