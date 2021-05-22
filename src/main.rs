extern crate termion;

use log::debug;
use rand::Rng;
use termion::color;

const BOARD_SIZE: usize = 51;
const ALIVE_CELL: char = 'X';
const DEAD_CELL: char = 'O';

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

fn play_game(rounds: usize) {
    let initial_board = generate_random_board();
    let mut current_board = initial_board;
    println!("########### Welcome to the game of life ##########");
    for round in 0..rounds {
        println!();
        println!("{}########## Round {}: ##########", color::Fg(color::Blue), round);
        print_board(current_board);
        current_board = bread_new_board(current_board);
    }
    println!();
    println!("{}########## Round {}: ##########", color::Fg(color::Blue), rounds);
    print_board(current_board);
}

fn print_board(board: [[char; BOARD_SIZE]; BOARD_SIZE]) {
    for row in board.iter() {
        for cell in row {
            if cell == &ALIVE_CELL {
                print!("{}{}", color::Fg(color::Green), cell);
            } else {
                print!("{}{}", color::Fg(color::White), cell);
            };
        }
        println!();
    }
}

fn generate_dead_board() -> [[char; BOARD_SIZE]; BOARD_SIZE] {
    return [[DEAD_CELL; BOARD_SIZE]; BOARD_SIZE];
}

fn generate_random_board() -> [[char; BOARD_SIZE]; BOARD_SIZE] {
    let mut board = generate_dead_board();
    let mut rng = rand::thread_rng();

    for row_index in 0..BOARD_SIZE {
        for cell_index in 0..BOARD_SIZE {
            if rng.gen_range(0..2) == 1 {
                board[row_index][cell_index] = ALIVE_CELL
            }
        }
    }
    return board;
}

fn bread_new_board(
    current_board: [[char; BOARD_SIZE]; BOARD_SIZE],
) -> [[char; BOARD_SIZE]; BOARD_SIZE] {
    let mut new_board = generate_dead_board();
    for row_index in 0..BOARD_SIZE {
        for cell_index in 0..BOARD_SIZE {
            new_board[row_index][cell_index] =
                calculate_cells_next_state(current_board, (cell_index, row_index));
        }
    }
    return new_board;
}

fn calculate_cells_next_state(
    board: [[char; BOARD_SIZE]; BOARD_SIZE],
    cell_location: (usize, usize),
) -> char {
    let number_of_neighbors = get_number_of_neighbors(board, cell_location);
    let (cell_index, row_index) = cell_location;
    if board[row_index][cell_index] == ALIVE_CELL {
        if number_of_neighbors == 2 || number_of_neighbors == 3 {
            return ALIVE_CELL;
        }
    } else if number_of_neighbors == 3 {
        return ALIVE_CELL;
    }

    return DEAD_CELL;
}

fn get_number_of_neighbors(
    board: [[char; BOARD_SIZE]; BOARD_SIZE],
    cell_location: (usize, usize),
) -> usize {
    let mut number_of_neighbors = 0;
    let (x, y) = cell_location;
    for (x_offset, y_offset) in NEIGHBOR_LOCATIONS.iter() {
        let new_location = (x as i32 + x_offset, y as i32 + y_offset);
        debug!("Check location {}, {}", new_location.0, new_location.1);
        if is_location_valid(new_location) {
            let valid_location: (usize, usize) = (new_location.0 as usize, new_location.1 as usize);
            debug!(
                "Location {}, {} is valid!",
                valid_location.0, valid_location.1
            );
            if is_cell_alive(board, valid_location) {
                number_of_neighbors = number_of_neighbors + 1;
            }
        }
    }

    return number_of_neighbors;
}

fn is_location_valid(location: (i32, i32)) -> bool {
    return location.0 >= 0
        && location.1 >= 0
        && location.0 < BOARD_SIZE as i32
        && location.1 < BOARD_SIZE as i32;
}

fn is_cell_alive(board: [[char; BOARD_SIZE]; BOARD_SIZE], cell_location: (usize, usize)) -> bool {
    let (x, y) = cell_location;
    return board[y][x] == ALIVE_CELL;
}

fn main() {
    env_logger::init();
    play_game(300);
}
