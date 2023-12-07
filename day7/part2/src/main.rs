use std::fs::{File, metadata};
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

const RADIX: u32 = 10;

// Function that parses a card to its value
fn parse_card(c: char) -> usize {
    match c {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        'J' => 0,
        _ => panic!("Invalid card character"),
    }
}

// Function that finds the highest and second highest cards in a hand
fn find_highest_and_second_highest(card_counts: &[usize]) -> (usize, usize) {
    let mut highest = usize::MIN;
    let mut second_highest = usize::MIN;

    for count in card_counts {
        if *count > highest {
            second_highest = highest;
            highest = *count;
        } else if *count > second_highest {
            second_highest = *count;
        }
    }

    (highest, second_highest)
}

// Function that scores a hand of card based on type
fn score_hand(card_counts: Vec<usize>) -> usize {
    // Get the highest and second highest card counts excluding the jokers which are at position 0
    let (highest, second_highest) = find_highest_and_second_highest(&card_counts[1..]);

    // Based on the highest + jokers and second highest card counts assign a score
    match card_counts[0] + highest {
        // (0) High card        (highest count is 1)
        1 => 0,
        // (1) One pair         (highest count is 2 and second highest count is 1)
        // (2) Two pair         (highest count is 2 and second highest count is 2)
        2 => second_highest,
        // (3) Three of a kind  (highest count is 3 and second highest count is 1)
        // (4) Full house       (highest count is 3 and second highest count is 2)
        3 => second_highest + 2,
        // (5) Four of a kind   (highest count is 4)
        4 => 5,
        // (6) Five of a kind   (highest count is 5)
        5 => 6,
        _ => panic!("Invalid card counts"),
    }
}

// Function that processes each line
fn process_line(line: &str, shared_output: &Arc<Mutex<Vec<(usize, [usize; 5], u64)>>>) {
    let mut parser = line.chars();
    
    // Parse the first characters before a space to a hand
    let parsed_hand: Vec<usize> = parser.by_ref()
        .take_while(|c| *c != ' ')
        .map(parse_card)
        .collect();

    // Count the number of matching cards of each kind
    let mut card_counts = vec![0; 13];
    parsed_hand.iter().for_each(|card| card_counts[*card] += 1);

    // Score the hand based on the card counts
    let score = score_hand(card_counts);

    // Get the bid amount
    let bid = parser.by_ref()
        .filter_map(|c| c.to_digit(RADIX))
        .fold(0, |bid, digit| bid * RADIX as u64 + digit as u64);

    // Write to the shared output
    let mut output = shared_output.lock().unwrap();
    output.push((score, (parsed_hand[0], parsed_hand[1], parsed_hand[2], parsed_hand[3], parsed_hand[4]).into(), bid));
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

        let hands_parsed: Arc<Mutex<Vec<(usize, [usize; 5], u64)>>> = Arc::new(Mutex::new(Vec::with_capacity(num_lines)));

        // Spawn threads
        for _ in 0..num_cpus::get() {
            let thread_safe_reader_clone = Arc::clone(&thread_safe_reader);
            let hands_parsed_clone = Arc::clone(&hands_parsed);

            // Spawn a new thread
            let handle = thread::spawn(move || {
                // Inside the thread, each thread processes a portion of the input

                // Lock the reader and read lines
                while let Ok(mut locked_reader) = thread_safe_reader_clone.lock() {
                    let mut line = String::new();

                    // Read a line from the reader
                    match locked_reader.read_line(&mut line) {
                        Ok(0) => break, // End of file
                        Ok(_) => process_line(&line, &hands_parsed_clone),
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

        // Sort the hands
        // primary key:     type of hand
        // secondary key:   cards in hand
        // tertiary key:    bid
        let mut parsed_hands = hands_parsed.lock().unwrap();
        parsed_hands.sort();

        // Sum the rank times bid for each hand
        let total_result: u64 = parsed_hands.iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) as u64 * hand.2)
            .sum();
        
        // Print the final result
        println!("Total winnings: {}", total_result);
    }
}