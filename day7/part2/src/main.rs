use std::fs::File;
use std::io::{BufRead, BufReader};

use rayon::prelude::*;

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
fn process_line(line: String) -> (usize, [usize; 5], u64) {
    let mut parser = line.chars();
    
    // Parse the first characters before a space to a hand
    let parsed_hand: [usize; 5] = parser.by_ref()
        .take_while(|c| *c != ' ')
        .map(parse_card)
        .collect::<Vec<usize>>()
        .try_into()
        .expect("Failed to get 5 cards in a hand");

    // Count the number of matching cards of each kind
    let mut card_counts = vec![0; 13];
    parsed_hand.iter().for_each(|card| card_counts[*card] += 1);

    // Score the hand based on the card counts
    let score = score_hand(card_counts);

    // Get the bid amount
    let bid = parser.by_ref()
        .filter_map(|c| c.to_digit(RADIX))
        .fold(0, |bid, digit| bid * RADIX as u64 + digit as u64);

    (score, parsed_hand, bid)
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");

    // Parse the lines to hands
    let mut parsed_lines = BufReader::new(file)
        .lines()
        .par_bridge()
        .map(|line| line.ok())
        .while_some()
        .map(process_line)
        .collect::<Vec<(usize, [usize; 5], u64)>>();

    // Sort the hands
    // primary key:     type of hand
    // secondary key:   cards in hand
    // tertiary key:    bid
    parsed_lines.sort();

    // Sum the rank times bid for each hand
    let total_result: u64 = parsed_lines.par_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) as u64 * bid)
        .sum();
        
    // Print the final result
    println!("Total winnings: {}", total_result);
}