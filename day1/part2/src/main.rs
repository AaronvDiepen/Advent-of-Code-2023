use std::fs::File;
use std::io::{BufRead, BufReader};

use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use rayon::prelude::*;

/// Function that processes each line
fn process_line(line: String, fwac: AhoCorasick, bwac: AhoCorasick) -> u32 {
    let mut result = 0;

    // Get the first match of the Aho-Corasick on the string and add the corresponding value to the result
    if let Some(mat) = fwac.find(&line) {
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

    // Process the lines and sum the calibration values
    let total_result: u32 = BufReader::new(file)
        .lines()
        .par_bridge()
        .map(|line| line.ok())
        .while_some()
        .map(|line| process_line(line, fwac.clone(), bwac.clone()))
        .sum();

    // Print the final result
    println!("Summed calibration values: {}", total_result);
}