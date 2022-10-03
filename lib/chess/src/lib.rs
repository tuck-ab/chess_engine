// mod pieces;

// use pieces::Piece;
// use pieces::PieceType::*;
// use pieces::Side::*;

#[derive(Copy, Clone)]
pub enum Side {
    Black,
    White
}

#[derive(Copy, Clone)]
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

#[derive(Copy,Clone)]
pub struct Piece {
    piece_type: PieceType,
    side: Side,
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
                side: White
            }),
            'k' => Some(Piece {
                piece_type: King{has_moved: false},
                side: Black
            }),
            'Q' => Some(Piece {
                piece_type: Queen,
                side: White
            }),
            'q' => Some(Piece {
                piece_type: Queen,
                side: Black
            }),
            'B' => Some(Piece {
                piece_type: Bishop,
                side: White
            }),
            'b' => Some(Piece {
                piece_type: Bishop,
                side: Black
            }),
            'N' => Some(Piece {
                piece_type: Knight,
                side: White
            }),
            'n' => Some(Piece {
                piece_type: Knight,
                side: Black
            }),
            'R' => Some(Piece {
                piece_type: Rook{has_moved:false},
                side: White
            }),
            'r' => Some(Piece {
                piece_type: Rook{has_moved:false},
                side: Black
            }),
            'P' => Some(Piece {
                piece_type: Pawn{has_moved:false},
                side: White
            }),
            'p' => Some(Piece {
                piece_type: Pawn{has_moved: false},
                side: Black
            }),
            '.' => None,
            _ => {return Err(ParseError::UnexpectedCharacter)}
        };
    }

    Ok(board)
}