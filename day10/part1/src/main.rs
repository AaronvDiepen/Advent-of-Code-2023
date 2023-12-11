use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::MultiUnzip;

// Function that tells if a character can have a left connection
fn map_char_to_left(c: char) -> bool {
    match c {
        '-' | 'J' | '7' | 'S' => true,
        _ => false,
    }
}

// Function that tells if a character can have a right connection
fn map_char_to_right(c: char) -> bool {
    match c {
        '-' | 'L' | 'F' | 'S' => true,
        _ => false,
    }
}

// Function that tells if a character can have a top connection
fn map_char_to_top(c: char) -> bool {
    match c {
        '|' | 'L' | 'J' | 'S' => true,
        _ => false,
    }
}

// Function that tells if a character can have a bottom connection
fn map_char_to_bottom(c: char) -> bool {
    match c {
        '|' | '7' | 'F' | 'S' => true,
        _ => false,
    }
}


// Function that processes each line
fn process_line(line: String) -> (Vec<bool>, Vec<bool>, Vec<bool>, Vec<bool>, Option<usize>) {
    // Create vectors indicating whether a connection can be made into a certain direction
    let left = line.chars().map(map_char_to_left).collect();
    let right = line.chars().map(map_char_to_right).collect();
    let top = line.chars().map(map_char_to_top).collect();
    let bottom = line.chars().map(map_char_to_bottom).collect();

    // Create an option containing the index of S if it is in the current line
    let s_index = line.chars().position(|c| c == 'S');

    (left, right, top, bottom, s_index)
}

fn main() {
    // Open the input file
    let file = File::open("input").expect("Could not open file \"input\" relative to program");

    // Process the lines to vectors contianing possible connections and optional starting positions in each line
    let (left_connections, right_connections, top_connections, bottom_connections, s_indices): (Vec<Vec<bool>>, Vec<Vec<bool>>, Vec<Vec<bool>>, Vec<Vec<bool>>, Vec<Option<usize>>) = BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .map(process_line)
        .multiunzip();

    // Get all valid horizontal connections in the grid
    let horizontal_connections: Vec<Vec<bool>> = left_connections.iter()
        .zip(right_connections.iter())
        .map(|(left_connections_row, right_connection_row)| left_connections_row.iter()
            .skip(1)
            .zip(right_connection_row.iter())
            .map(|(left_connection, right_connection)| *left_connection && *right_connection)
            .collect()
        ).collect();

    // Get all valid vertical connections in the grid
    let vertical_connections: Vec<Vec<bool>> = top_connections.iter()
        .skip(1)
        .zip(bottom_connections.iter())
        .map(|(top_connections_row, bottom_connection_row)| top_connections_row.iter()
            .zip(bottom_connection_row.iter())
            .map(|(top_connection, bottom_connection)| *top_connection && *bottom_connection)
            .collect()
        ).collect();

    // Get the start position
    let start_position: (usize, usize) = s_indices.iter()
        .enumerate()
        .find_map(|(line_index, s_index_option)| {
            if s_index_option.is_some() {
                Some((line_index, s_index_option.unwrap()))
            } else {
                None
            }
        })
        .unwrap();

    // Get the grid height and width
    let grid_height = left_connections.len();
    let grid_width = left_connections[0].len();

    // Travel allong the loop that the start node is connected to and count the number of pipes
    // Use a grid with booleans indicating whether we have visited a node before
    let mut visited = vec![vec![false; grid_width]; grid_height];
    let mut current_position = start_position;
    let mut number_of_pipes = 0;
    loop {
        let (current_y, current_x) = current_position;
        visited[current_y][current_x] = true;
        number_of_pipes += 1;
        // Path to the left if possible
        if current_x > 0
        && horizontal_connections[current_y][current_x - 1]
        && !visited[current_y][current_x - 1] {
            current_position = (current_y, current_x - 1);
            continue;
        }
        // Path to the right if possible
        if current_x < grid_width - 1
        && horizontal_connections[current_y][current_x]
        && !visited[current_y][current_x + 1] {
            current_position = (current_y, current_x + 1);
            continue;
        }
        // Path to upward if possible
        if current_y > 0
        && vertical_connections[current_y - 1][current_x]
        && !visited[current_y - 1][current_x] {
            current_position = (current_y - 1, current_x);
            continue;
        }
        // Path to downward if possible
        if current_y < grid_height - 1
        && vertical_connections[current_y][current_x]
        && !visited[current_y + 1][current_x] {
            current_position = (current_y + 1, current_x);
            continue;
        }
        // If we have no new direction to path we have found the cycle
        break;
    }

    // Print the final result
    println!("Number of steps to furthest pipe: {}", number_of_pipes/2);
}
