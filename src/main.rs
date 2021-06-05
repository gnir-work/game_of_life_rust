mod board;

use board::{Board, Cell};
use termion::color;
use std::io;

fn play_game(board_size: usize, rounds: usize) {
    let initial_board = Board::create_new_board(board_size);
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

fn bread_new_board(current_board: &Board) -> Board {
    let mut new_board: Board = (*current_board).clone();
    for row_index in 0..current_board.size {
        for cell_index in 0..current_board.size {
            let location = (cell_index, row_index);
            new_board.update_cell(location,
                                  calculate_cell_next_state(current_board, location));
        }
    }
    return new_board;
}

fn calculate_cell_next_state(board: &Board, cell_location: (usize, usize)) -> Cell {
    let number_of_neighbors = board.get_number_of_alive_cells(cell_location);
    if board.is_cell_alive(cell_location) {
        if number_of_neighbors == 2 || number_of_neighbors == 3 {
            return Cell::Alive;
        }
    } else if number_of_neighbors == 3 {
        return Cell::Alive;
    }
    return Cell::Dead;
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
