use std::fs::File;
use std::io::{BufRead, BufReader};

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

    // Read times and distances, by splitting on whitespace and skipping the descriptors
    // Then zip them together
    // Then find their respective range of integer solutions
    // Then multiply those together
    let result: u64 = parser.by_ref()
        .next()
        .unwrap()
        .expect("Could not get times line")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .zip(
            parser.by_ref()
            .next()
            .unwrap()
            .expect("Could not get distances line")
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
        )
        .map(|(time, distance)| find_integer_solution_range(time, distance))
        .product();

    // Print the final result
    println!("Multiplied ways to beat record: {}", result);
}