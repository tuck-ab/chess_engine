#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    Black,
    White
}

impl Side{
    pub fn other(&self) -> Self {
        match *self {
            White => Black,
            Black => White
        }
    }

    pub fn get_promotion_y(&self) -> i8 {
        match *self {
            White => 7,
            Black => 0
        }
    }

    pub fn get_dir(&self) -> i8 {
        match *self {
            White => 1,
            Black => -1
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Rook,
    Knight,
    Pawn
}

pub const PROMOTABLE_PIECES: [PieceType; 4] = [Queen, Bishop, Rook, Knight];

use Side::*;
use PieceType::*;
use crate::coord::*;


#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    side: Side,
    has_moved: bool,
    piece_type: PieceType,
    loc: Coord
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{:?} {:?} on {:?} [{} Moved ]", 
            self.side, self.piece_type, self.loc, 
            if self.has_moved {""} else {" Not"}
        )?;
        Ok(())
    }
}

impl Piece {
    pub fn new(side: Side, 
               has_moved: bool, 
               piece_type: PieceType, 
               loc: Coord)
               -> Self {
        Self {side, has_moved, piece_type, loc}
    }

    pub fn is_type(&self, piece_type: PieceType) -> bool {
        self.piece_type == piece_type
    }

    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn is_side(&self, side: Side) -> bool {
        self.side == side
    }

    pub fn get_side(&self) -> Side {
        self.side
    }

    pub fn get_loc(&self) -> Coord {
        self.loc
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub fn move_to(&mut self, loc: Coord) -> Self {
        self.loc = loc;
        self.has_moved = true;
        *self
    }
}

pub fn get_piece_char(piece: &Piece) -> char {
    let out_char = match piece.piece_type {
        King => 'K',
        Queen => 'Q',
        Bishop => 'B',
        Knight => 'N',
        Rook => 'R',
        Pawn => 'P'
    };

    if piece.side == Black {
        out_char.to_ascii_lowercase()
    } else {
        out_char
    }
}