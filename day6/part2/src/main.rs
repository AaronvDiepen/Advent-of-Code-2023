use std::fs::File;
use std::io::{BufRead, BufReader};

const RADIX: u32 = 10;

// Find the range of solutions for n to the problem d + 1 < (n * t - n)
fn find_integer_solution_range(t: u64, d: u64) -> u64 {
    // Calculate the determinant
    let discriminant = (t.pow(2) - 4 * (d + 1)) as f64;

    // If there are no valid solutions to the problem we return 0
    if discriminant < 0.0 {
        return 0
    }

    // If there is exactly one valid solution to the problem we return 1
    if discriminant == 0.0 {
        return 1
    }

    // Calculate the lowest and highest valid integer solutions
    let discriminant_sqrt = discriminant.sqrt() as u64;

    let n1 = t - (t + discriminant_sqrt) / 2;
    let n2 = t - (t - discriminant_sqrt + 1) / 2;

    // Return the range between the highest and lowest integer solutions
    n2 - n1 + 1
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");
    let reader = BufReader::new(file);

    // Create a single parser for all the lines
    let mut parser = reader.lines();

    // Read time and distance, by filtering all digit characters from the input then folding them together into numbers
    // Then find the range of integer solutions
    let result = find_integer_solution_range(
        parser.by_ref()
            .next()
            .unwrap()
            .expect("Could not get time line")
            .chars()
            .filter_map(|c| c.to_digit(RADIX))
            .fold(0, |time, digit| time * RADIX as u64 + digit as u64),
        parser.by_ref()
            .next()
            .unwrap()
            .expect("Could not get distance line")
            .chars()
            .filter_map(|c| c.to_digit(RADIX))
            .fold(0, |distance, digit| distance * RADIX as u64 + digit as u64)
        );

    // Print the final result
    println!("Ways to beat record: {}", result);
}