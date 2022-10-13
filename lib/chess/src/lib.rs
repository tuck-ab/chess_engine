// mod pieces;

// use pieces::Piece;
// use pieces::PieceType::*;
// use pieces::Side::*;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug)]
pub enum Side {
    Black,
    White
}

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    King{has_moved: bool},
    Queen,
    Bishop,
    Knight,
    Rook{has_moved: bool},
    Pawn{has_moved: bool}
}

use crate::PieceType::*;
use crate::Side::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Coord {
    Index (usize),
    XandY (i8, i8)
}

impl Coord {
    pub fn as_index(&self) -> Self {
        match *self {
            Coord::Index(_) => *self,
            Coord::XandY(x, y) => Coord::Index((x + (8 * (7-y))).into())
        }
    }

    pub fn get_index(&self) -> usize {
        match *self {
            Coord::Index(i) => i,
            Coord::XandY(x, y) => (x + (8 * (7-y))).into()
        }
    }

    pub fn as_x_and_y(&self) -> Self {
        match *self {
            Coord::Index(i) => Coord::XandY(i as i8 % 8, 7 - (i as i8 / 8)),
            Coord::XandY(_, _) => *self
        }
    }

    pub fn get_x_and_y(&self) -> [i8; 2] {
        match *self {
            Coord::Index(i) => [i as i8 % 8, 7 - (i as i8 / 8)],
            Coord::XandY(x, y) => [x, y]
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    piece_type: PieceType,
    side: Side,
    loc: Coord
}

pub struct Game {
    board: [Option<Piece>; 64]
}

impl Game {
    pub fn new() -> Self {
        let start_code = "rnbqkbnrpppppppp................................PPPPPPPPRNBQKBNR";
        Game { 
            board: board_from_string(start_code).unwrap()
        }
    }
}

#[derive(Debug)]
enum ParseError {
    UnexpectedCharacter
}

fn board_from_string(code: &str) -> Result<[Option<Piece>; 64], ParseError> {
    let mut board:[Option<Piece>; 64] = [None; 64];
    for (i, c) in code.chars().enumerate() {
        board[i] = match c {
            'K' => Some(Piece {
                piece_type: King{has_moved:false},
                side: White,
                loc: Coord::Index(i)
            }),
            'k' => Some(Piece {
                piece_type: King{has_moved: false},
                side: Black,
                loc: Coord::Index(i)
            }),
            'Q' => Some(Piece {
                piece_type: Queen,
                side: White,
                loc: Coord::Index(i)
            }),
            'q' => Some(Piece {
                piece_type: Queen,
                side: Black,
                loc: Coord::Index(i)
            }),
            'B' => Some(Piece {
                piece_type: Bishop,
                side: White,
                loc: Coord::Index(i)
            }),
            'b' => Some(Piece {
                piece_type: Bishop,
                side: Black,
                loc: Coord::Index(i)
            }),
            'N' => Some(Piece {
                piece_type: Knight,
                side: White,
                loc: Coord::Index(i)
            }),
            'n' => Some(Piece {
                piece_type: Knight,
                side: Black,
                loc: Coord::Index(i)
            }),
            'R' => Some(Piece {
                piece_type: Rook{has_moved:false},
                side: White,
                loc: Coord::Index(i)
            }),
            'r' => Some(Piece {
                piece_type: Rook{has_moved:false},
                side: Black,
                loc: Coord::Index(i)
            }),
            'P' => Some(Piece {
                piece_type: Pawn{has_moved:false},
                side: White,
                loc: Coord::Index(i)
            }),
            'p' => Some(Piece {
                piece_type: Pawn{has_moved: false},
                side: Black,
                loc: Coord::Index(i)
            }),
            '.' => None,
            _ => {return Err(ParseError::UnexpectedCharacter)}
        };
    }

    Ok(board)
}