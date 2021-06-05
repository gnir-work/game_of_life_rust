//! This module export all of the basic functionality that is needed to interact with the board.
//!
//! The board is built from several rows of [Cell] enums were each Cell can be either alive or dead.
//! The board exports all of the needed functionality in order to implement game of life on top of
//! it.
//!
//! For a more detailed explanation of the functionality please look at [Board]
//!
//! # Usage
//! ```
//! use board::{Board, Cell};
//!
//! let initial_board = Board::create_new_board(10);
//! if initial_board.is_cell_alive((5, 5)) {
//!     initial_board.update_cell((5, 5), Cell:Dead);
//! }
//! ```

extern crate termion;

mod cell;

use rand::Rng;
use termion::color;
use log::debug;
use std::vec::Vec;
pub use self::cell::Cell;

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


#[derive(Clone)]
struct Row {
    cells: Vec<Cell>,
}

/// The board struct which represent the whole board.
/// All of the basic manupilation on the board can be found here, including creating a new board.
#[derive(Clone)]
pub struct Board {
    /// The size of the board (both x and y).
    pub size: usize,

    /// The rows that build up the board.
    ///
    /// For more information look at [Row].
    rows: Vec<Row>,
}

impl Board {
    /// Builder function for the board class.
    ///
    /// This function will return a random board, probably containing both [Alive](Cell::Alive) and [Dead](Cell::Dead) cells.
    pub fn create_new_board(size: usize) -> Board {
        let mut rng = rand::thread_rng();
        let mut rows: Vec<Row> = Vec::with_capacity(size);
        for _ in 0..size {
            let mut cells = Vec::with_capacity(size);
            for _ in 0..size {
                if rng.gen_range(0..2) == 1 {
                    cells.push(Cell::Alive);
                } else {
                    cells.push(Cell::Dead);
                }
            }
            rows.push(Row { cells });
        }
        return Board {
            rows,
            size,
        };
    }

    /// Print the board to the stdout.
    pub fn print(&self) {
        for row in self.rows.iter() {
            for cell in row.cells.iter() {
                match cell {
                    Cell::Alive => {
                        print!("{}{}", color::Fg(color::Green), cell);
                    }
                    Cell::Dead => {
                        print!("{}{}", color::Fg(color::White), cell);
                    }
                }
            }
            println!();
        }
    }

    /// Return the number of alive cells for a given location.
    ///
    /// Please note: For each cell there is a totoal of 8 neighbors - The 4 nighbors at an angle are
    /// also accounted for.
    pub fn get_number_of_alive_cells(&self, cell_location: (usize, usize)) -> usize {
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

    /// Checks if the current location is inside of the board.
    pub fn is_valid_location(&self, location: (i32, i32)) -> bool {
        return location.0 >= 0
            && location.1 >= 0
            && location.0 < self.size as i32
            && location.1 < self.size as i32;
    }

    pub fn is_cell_alive(&self, location: (usize, usize)) -> bool {
        let (x, y) = location;
        return Cell::Alive == self.rows[y].cells[x];
    }

    pub fn update_cell(&mut self, location: (usize, usize), new_state: Cell) {
        let (x, y) = location;
        self.rows[y].cells[x] = new_state;
    }
}