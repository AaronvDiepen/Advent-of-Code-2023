use std::fs::{metadata, File};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;

const RADIX: u32 = 10;

/// Function that processes a line to number of gears and their ratio
fn get_ratio_line(i: usize, line: &str, check_middle: bool) -> (usize, u32) {
    // Find the adjacent numbers to i, and multiply them times the gear ratio
    // Try creating a number at the gear if possible
    if check_middle && line.chars().skip(i).next().expect("Could not get a character").is_digit(RADIX) {
        // Grow number to the left
        let mut gear_ratio = line.chars()
            .rev()
            .skip(line.len() - i)
            .take_while(|c| c.is_digit(RADIX))
            .map(|c| c.to_digit(RADIX).unwrap())
            .enumerate()
            .fold(0, |number, (j, digit)| number + digit * RADIX.pow(j as u32));

        // Grow number to the right
        gear_ratio = line.chars()
            .skip(i)
            .take_while(|c| c.is_digit(RADIX))
            .map(|c| c.to_digit(RADIX).unwrap())
            .fold(gear_ratio, |number, digit| number * RADIX + digit);

        (1, gear_ratio)
    } else {
        let mut gear_ratio = 1;
        let mut gear_count = 0;
        
        // Try creating a number in front of the gear
        if i > 0 && line.chars().skip(i-1).next().expect("Could not get a character").is_digit(RADIX) {
            gear_count += 1;
            // Grow number to the left
            gear_ratio *= line.chars()
                .rev()
                .skip(line.len() - i)
                .take_while(|c| c.is_digit(RADIX))
                .map(|c| c.to_digit(RADIX).unwrap())
                .enumerate()
                .fold(0, |number, (j, digit)| number + digit * RADIX.pow(j as u32));
        }

        // Try creating a number in behind the gear
        if i < line.len() && line.chars().skip(i+1).next().expect("Could not get a character").is_digit(RADIX) {
            gear_count += 1;
            // Grow number to the right
            gear_ratio *= line.chars()
                .skip(i + 1)
                .take_while(|c| c.is_digit(RADIX))
                .map(|c| c.to_digit(RADIX).unwrap())
                .fold(0, |number, digit| number * RADIX + digit);
        }

        (gear_count, gear_ratio)
    }
}

/// Function that processes each line
fn process_line(line: &str, line_previous: &str, line_next: &str) -> u32 {
    let mut result = 0;

    // Iterate over subsequent digits to find the end position
    for (i, next_char) in line.char_indices() {
        if next_char == '*' {
            // Check the current line for adjacent numbers
            let (mut gear_count, mut gear_ratio) = get_ratio_line(i, line, false);

            // Check previous line for adjacent numbers
            if !line_previous.is_empty() {
                let (new_gear_count, new_gear_ratio) = get_ratio_line(i, line_previous, true);
                gear_count += new_gear_count;
                gear_ratio *= new_gear_ratio;
            }

            // Check next line for adjacent numbers if we have not yet exceeded the gear count
            if !line_next.is_empty() && gear_count < 3 {
                let (new_gear_count, new_gear_ratio) = get_ratio_line(i, line_next, true);
                gear_count += new_gear_count;
                gear_ratio *= new_gear_ratio;
            }
            
            result += gear_ratio * ((gear_count == 2) as u32);
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
        // Get the line length of the first line
        let line_length: u64 = first_line.len().try_into().unwrap();

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
                        * (line_length + 1))) {
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
