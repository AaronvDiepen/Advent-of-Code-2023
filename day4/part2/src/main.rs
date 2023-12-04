use std::collections::HashSet;
use std::fs::{File, metadata};
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

const RADIX: u32 = 10;

// Function that processes each line
fn process_line(line: &str, shared_output: &Arc<Mutex<Vec<usize>>>) {
    // Extract the numbers on the left and right of the pipe
    let numbers: Vec<_> = line.split('|').collect();

    // Extract individual numbers from the left part into an hashset
    let mut left_words = numbers[0]
        .split_whitespace();

    let card_number = left_words.by_ref()
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .take_while(|c| c.is_digit(RADIX))
        .fold(0, |acc, c| acc * RADIX as usize + c.to_digit(RADIX).unwrap() as usize);

    let left_numbers: HashSet<_> = left_words.by_ref()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    // Count the number of matches on the right by filtering out number that are not on the left and counting the sum.
    let count = numbers[1]
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .filter(|num| left_numbers.contains(num))
        .count();

    let mut output = shared_output.lock().unwrap();
    output[card_number-1] = count;
}

fn main() {
    // Extract the file length from the metadata
    let file_length: usize = metadata("input").expect("Could not get input file metadata").len().try_into().unwrap();

    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");
    let reader = BufReader::new(file);

    // Get the first line from the input
    if let Some(Ok(first_line)) = reader.lines().next() {
        // Get the line length of the first line including newline
        let line_length: usize = first_line.len() + 1;

        // Create a thread-safe reader
        let file = File::open("input").expect("Could not open file \"input\" relative to program");
        let reader = BufReader::new(file);
        let thread_safe_reader = Arc::new(Mutex::new(reader));

        // Create a vector to store the handles to the spawned threads
        let mut handles = vec![];

        // Get the number of lines in the file assuming equal length and create an array to store partial output
        let num_lines = file_length / line_length;

        let card_wins: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![0; num_lines]));

        // Spawn threads
        for _ in 0..num_cpus::get() {
            let thread_safe_reader_clone = Arc::clone(&thread_safe_reader);
            let card_wins_clone = Arc::clone(&card_wins);

            // Spawn a new thread
            let handle = thread::spawn(move || {
                // Inside the thread, each thread processes a portion of the input

                // Lock the reader and read lines
                while let Ok(mut locked_reader) = thread_safe_reader_clone.lock() {
                    let mut line = String::new();

                    // Read a line from the reader
                    match locked_reader.read_line(&mut line) {
                        Ok(0) => break, // End of file
                        Ok(_) => process_line(&line, &card_wins_clone),
                        Err(e) => eprintln!("Error reading line: {}", e),
                    }
                }
            });

            // Save the handle to the vector
            handles.push(handle);
        }

        // Wait for all threads to finish and collect results
        for handle in handles {
            handle.join().unwrap();
        }

        // Count the number of cards we end up with
        let mut final_card_wins = card_wins.lock().unwrap().clone();
        
        // Start from the back and replace number of winning numbers with the number of cards that we win from having that card
        for i in (0..final_card_wins.len()).rev() {
            let mut won_cards = 1;
            if final_card_wins[i] > 0 {
                for j in 1..=final_card_wins[i] {
                    let card_won = i + j;
                    if card_won < final_card_wins.len() {
                        won_cards += final_card_wins[card_won];
                    }
                }
            }
            final_card_wins[i] = won_cards;
        }

        // Summing the number of cards we win from winning a card for all cards gets us our number of cards.
        let total_result: usize = final_card_wins.iter().sum();

        // Print the final result
        println!("Number of cards scratched: {}", total_result);
    }
}