use std::fs::File;
use std::io::{BufRead, BufReader};

const RADIX: u32 = 10;

// Find the range of solutions for n to the problem d + 1 < (n * t - n)
fn find_integer_solution_range(t: i64, d: i64) -> i64 {
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
    let discriminant_sqrt = discriminant.sqrt() as i64;
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

    // Read time and distance by getting all digit characters on their lines, then folding them together into numbers and finding their range of integer solutions
    let result: i64 = find_integer_solution_range(
        parser.by_ref()
            .next()
            .unwrap()
            .expect("Could not get time line")
            .chars()
            .filter(|&c| c.is_digit(RADIX))
            .fold(0, |acc, digit| acc * RADIX as i64 + digit.to_digit(RADIX).unwrap() as i64),
        parser.by_ref()
            .next()
            .unwrap()
            .expect("Could not get distance line")
            .chars()
            .filter(|&c| c.is_digit(RADIX))
            .fold(0, |acc, digit| acc * RADIX as i64 + digit.to_digit(RADIX).unwrap() as i64)
        );

    // Print the final result
    println!("Ways to beat record: {}", result);
}