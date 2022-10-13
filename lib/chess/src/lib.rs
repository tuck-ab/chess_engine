// mod pieces;

// use pieces::Piece;
// use pieces::PieceType::*;
// use pieces::Side::*;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug)]
pub struct Move {
    piece: PieceType,
    from: Coord,
    to: Coord
}

pub struct Game {
    board: [Option<Piece>; 64],
    pieces: Vec<Piece>
}

impl Game {
    pub fn new() -> Self {
        let start_code = "rnbqkbnrpppppppp................................PPPPPPPPRNBQKBNR";
        let returned_board = board_from_string(start_code).unwrap();
        let mut pieces = Vec::<Piece>::new();

        returned_board.map(|x| {
            match x {
                Some(p) => pieces.push(p),
                _ => {}
            }
        });

        Game { 
            board: returned_board,
            pieces
        }
    }

    pub fn get_all_moves(&self, side: Side) {
        let mut moves: Vec<Move> = Vec::<Move>::new();

        for piece in &self.pieces {
            if piece.side == side {
                match piece.piece_type {
                    King { has_moved } => {

                    },
                    Queen => {

                    },
                    Bishop => {

                    },
                    Knight => {

                    },
                    Rook { has_moved } => {
                        for dir in [[1,0], [0,1], [-1,0], [0,-1]] {
                            let finding = true;
                            while finding {
                                let [x, y] = piece.loc.get_x_and_y();
                                x += dir[0];
                                y += dir[1];

                                let square = &self.board[Coord::XandY(x, y).get_index()];
                                match &self.board[Coord::XandY(x, y).get_index()] {
                                    Some(p) => {
                                        finding = false;
                                        if p.side != side {
                                            moves.push(Move { 
                                                piece: Rook { has_moved: true }, 
                                                from: piece.loc.clone(), 
                                                to: Coord::XandY(x, y) })
                                        }
                                    },
                                    None => {
                                        moves.push(Move { 
                                            piece: Rook { has_moved: true }, 
                                            from: piece.loc.clone(), 
                                            to: Coord::XandY(x, y) })
                                    }
                                }
                            }
                        }
                    },
                    Pawn { has_moved } => {

                    }
                }
            }
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