use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

/// Function that processes each line
fn process_line(line: &str, fwac: &AhoCorasick, bwac: &AhoCorasick) -> u32 {
    let mut result = 0;

    // Get the first match of the Aho-Corasick on the string and add the corresponding value to the result
    if let Some(mat) = fwac.find(line) {
        // Convert the matched pattern index to the relevant integer multiply by 10 and add it
        result += (mat.pattern().as_u32() % 9 + 1) * 10;
    }

    // Get the first match of the reversed Aho-Corasick on the reverse string and add the corresponding value to the result
    let reversed_line : String = line.chars().rev().collect();
    if let Some(mat) = bwac.find(&reversed_line) {
        // Convert the matched pattern index to the relevant integer and add it
        result += mat.pattern().as_u32() % 9 + 1;
    }

    result
}

fn main() {
    // Define the words to be detected both forward(fw) and backward(bw)
    let fwwords = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let bwwords : Vec<String> = fwwords.iter().map(|&word| word.chars().rev().collect()).collect();
    
    // Build two Aho-Corasick tries
    let fwac = AhoCorasickBuilder::new().build(fwwords).expect("Should be able to build aho-corasick trie for forward matching");
    let bwac = AhoCorasickBuilder::new().build(bwwords).expect("Should be able to build aho-corasick trie for backward matching");

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
        let fwac_clone = fwac.clone();
        let bwac_clone = bwac.clone();

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
                    Ok(_) => result += process_line(&line, &fwac_clone, &bwac_clone),
                    Err(e) => eprintln!("Error reading line: {}", e),
                }
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
    println!("Summed calibration values: {}", total_result);
}