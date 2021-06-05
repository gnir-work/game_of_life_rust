#[derive(Clone)]
#[derive(PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Alive => "X".fmt(f),
            Cell::Dead => "O".fmt(f),
        }
    }
}
