extern crate termion;

use log::debug;
use rand::Rng;
use std::io;
use std::vec::Vec;
use termion::color;

const ALIVE_CELL: char = 'X';
const DEAD_CELL: char = 'O';

#[derive(Clone)]
struct Row {
    cells: Vec<char>,
}

#[derive(Clone)]
struct Board {
    size: usize,
    rows: Vec<Row>,
}

impl Board {
    fn print(&self) {
        for row in self.rows.iter() {
            for cell in row.cells.iter() {
                if cell == &ALIVE_CELL {
                    print!("{}{}", color::Fg(color::Green), cell);
                } else {
                    print!("{}{}", color::Fg(color::White), cell);
                };
            }
            println!();
        }
    }

    fn get_number_of_neighbors(self: &Board, cell_location: (usize, usize)) -> usize {
        let mut number_of_neighbors = 0;
        let (x, y) = cell_location;
        for (x_offset, y_offset) in NEIGHBOR_LOCATIONS.iter() {
            let new_location = (x as i32 + x_offset, y as i32 + y_offset);
            debug!("Check location {}, {}", new_location.0, new_location.1);
            if self.is_valid_location(new_location) {
                let valid_location: (usize, usize) = (new_location.0 as usize, new_location.1 as usize);
                debug!(
                    "Location {}, {} is valid!",
                    valid_location.0, valid_location.1
                );
                if self.is_cell_alive(valid_location) {
                    number_of_neighbors = number_of_neighbors + 1;
                }
            }
        }
        return number_of_neighbors;
    }

    fn is_valid_location(self: &Board, location: (i32, i32)) -> bool {
        return location.0 >= 0
            && location.1 >= 0
            && location.0 < self.size as i32
            && location.1 < self.size as i32;
    }

    fn is_cell_alive(self: &Board, location: (usize, usize)) -> bool {
        let (x, y) = location;
        return self.rows[y].cells[x] == ALIVE_CELL;
    }
}

const NEIGHBOR_LOCATIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn play_game(board_size: usize, rounds: usize) {
    let initial_board = generate_random_board(board_size);
    let mut current_board = initial_board;
    println!("########### Welcome to the game of life ##########");
    for round in 0..rounds {
        println!();
        println!(
            "{}########## Round {}: ##########",
            color::Fg(color::Blue),
            round
        );
        current_board.print();
        current_board = bread_new_board(&current_board);
    }
    println!();
    println!(
        "{}########## Round {}: ##########",
        color::Fg(color::Blue),
        rounds
    );
    current_board.print();
}


fn generate_random_board(size: usize) -> Board {
    let mut board = generate_dead_board(size);
    let mut rng = rand::thread_rng();

    for row_index in 0..size {
        for cell_index in 0..size {
            if rng.gen_range(0..2) == 1 {
                board.rows[row_index].cells[cell_index] = ALIVE_CELL
            }
        }
    }
    return board;
}

fn bread_new_board(current_board: &Board) -> Board {
    let mut new_board: Board = (*current_board).clone();
    for row_index in 0..current_board.size {
        for cell_index in 0..current_board.size {
            new_board.rows[row_index].cells[cell_index] =
                calculate_cells_next_state(current_board, (cell_index, row_index));
        }
    }
    return new_board;
}

fn calculate_cells_next_state(board: &Board, cell_location: (usize, usize)) -> char {
    let number_of_neighbors = board.get_number_of_neighbors(cell_location);
    let (cell_index, row_index) = cell_location;
    if board.rows[row_index].cells[cell_index] == ALIVE_CELL {
        if number_of_neighbors == 2 || number_of_neighbors == 3 {
            return ALIVE_CELL;
        }
    } else if number_of_neighbors == 3 {
        return ALIVE_CELL;
    }

    return DEAD_CELL;
}


fn generate_dead_row(size: usize) -> Row {
    return Row {
        cells: vec![DEAD_CELL; size],
    };
}

fn generate_dead_board(size: usize) -> Board {
    let mut rows: Vec<Row> = Vec::with_capacity(size);
    for _ in 0..size {
        rows.push(generate_dead_row(size));
    }
    return Board { rows, size };
}

fn get_positive_number_from_user(prompt: &str) -> usize {
    println!("{}", prompt);
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: usize = number.trim().parse().expect("Please enter a number!");
    return number;
}

fn get_board_size_from_user() -> usize {
    return get_positive_number_from_user("Please enter the board size: ");
}

fn get_number_of_rounds_from_user() -> usize {
    return get_positive_number_from_user("Please enter the number of rounds: ");
}

fn main() {
    env_logger::init();
    let board_size = get_board_size_from_user();
    let rounds = get_number_of_rounds_from_user();

    play_game(board_size, rounds);
}
