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

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug)]
pub enum Coord {
    Index (usize),
    XandY (i8, i8)
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.get_index() == other.get_index()
    }
}

impl Coord {
    pub fn as_index(&self) -> Self {
        match *self {
            Coord::Index(_) => *self,
            Coord::XandY(x, y) => Coord::Index(x as usize + (8 * (7-(y as usize))))
        }
    }

    pub fn get_index(&self) -> usize {
        match *self {
            Coord::Index(i) => i,
            Coord::XandY(x, y) => (x as usize + (8 * (7-(y as usize))))
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    side: Side,
    loc: Coord
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    piece: PieceType,
    from: Coord,
    to: Coord
}

pub struct Game {
    board: [Option<Piece>; 64],
    pieces: Vec<Piece>
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::new();
        
        for (i, op) in self.board.iter().enumerate() {
            if i % 8 == 0 {
                writeln!(f, "{}", out_str)?;
                out_str = String::new();
            }

            match op {
                Some(p) => out_str.push(get_piece_char(p)),
                None => out_str.push('.')
            }
            out_str.push(' ');
        }     

        writeln!(f, "{}", out_str)?;

        Ok(())
    }
}

impl Game {
    pub fn new() -> Self {
        let start_code = "rnbqkbnrpppppppp................................PPPPPPPPRNBQKBNR";
        Game::from_string(start_code).unwrap()
    }

    pub fn from_string(code: &str) -> Result<Self, ParseError>{
        let board = board_from_string(code)?;

        let mut pieces = Vec::<Piece>::new();

        board.map(|x| x.map(|p| pieces.push(p)));

        Ok(Game{board, pieces})
    }

    pub fn get_all_moves(&self, side: Side) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::<Move>::new();

        for piece in &self.pieces {
            if piece.side == side {
                match piece.piece_type {
                    King { has_moved } => {

                    },
                    Queen => {
                        for dir in [[1,1], [-1,1], [1,-1], [-1,-1], [1,0], [0,1], [-1,0], [0,-1]] {
                            let mut finding = true;
                            let [mut x, mut y] = piece.loc.get_x_and_y();

                            while finding {
                                x += dir[0];
                                y += dir[1];

                                // If the new coord is still on the board
                                if !(x < 0 || x > 7 || y < 0 || y > 7) {
                                    // Check square on board for piece
                                    match &self.board[Coord::XandY(x, y).get_index()] {
                                        Some(p) => { // If there is a piece
                                            // End the search in direction as piece can't jump
                                            finding = false; 

                                            // If the peice is opposite colour then its a valid move
                                            if p.side != side { 
                                                moves.push(Move { 
                                                    piece: Queen, 
                                                    from: piece.loc.clone(), 
                                                    to: Coord::XandY(x, y) 
                                                })
                                            }
                                        },
                                        None => { // If there is no piece then its a valid move
                                            moves.push(Move { 
                                                piece: Queen, 
                                                from: piece.loc.clone(), 
                                                to: Coord::XandY(x, y) 
                                            })
                                        }
                                    }
                                } else {
                                    finding = false;
                                }
                            }
                        }
                    },
                    Bishop => {
                        for dir in [[1,1], [-1,1], [1,-1], [-1,-1]] {
                            let mut finding = true;
                            let [mut x, mut y] = piece.loc.get_x_and_y();

                            while finding {
                                x += dir[0];
                                y += dir[1];

                                // If the new coord is still on the board
                                if !(x < 0 || x > 7 || y < 0 || y > 7) {
                                    // Check square on board for piece
                                    match &self.board[Coord::XandY(x, y).get_index()] {
                                        Some(p) => { // If there is a piece
                                            // End the search in direction as piece can't jump
                                            finding = false; 

                                            // If the peice is opposite colour then its a valid move
                                            if p.side != side { 
                                                moves.push(Move { 
                                                    piece: Bishop, 
                                                    from: piece.loc.clone(), 
                                                    to: Coord::XandY(x, y) 
                                                })
                                            }
                                        },
                                        None => { // If there is no piece then its a valid move
                                            moves.push(Move { 
                                                piece: Bishop, 
                                                from: piece.loc.clone(), 
                                                to: Coord::XandY(x, y) 
                                            })
                                        }
                                    }
                                } else {
                                    finding = false;
                                }
                            }
                        }
                    },
                    Knight => {
                        for dir in [[2,1], [2,-1], [1,2], [1,-2], [-2, 1], [-2,-1], [-1,2], [-1,-2]] {
                            let [mut x, mut y] = piece.loc.get_x_and_y();

                            x += dir[0];
                            y += dir[1];

                            // If the new coord is still on the board
                            if !(x < 0 || x > 7 || y < 0 || y > 7) {
                                // Check square on board for piece
                                match &self.board[Coord::XandY(x, y).get_index()] {
                                    // If there is a piece there
                                    Some(p) => {
                                        // Add move if it can be taken
                                        if p.side != side {
                                            moves.push(Move {
                                                piece: Knight,
                                                from: piece.loc.clone(),
                                                to: Coord::XandY(x, y)
                                            })
                                        }
                                    },
                                    // If there is no piece there
                                    None => {
                                        moves.push(Move {
                                            piece: Knight,
                                            from: piece.loc.clone(),
                                            to: Coord::XandY(x, y)
                                        })
                                    }
                                }
                            }
                        }
                    },
                    Rook { has_moved: _ } => {
                        for dir in [[1,0], [0,1], [-1,0], [0,-1]] {
                            let mut finding = true;
                            let [mut x, mut y] = piece.loc.get_x_and_y();

                            while finding {
                                x += dir[0];
                                y += dir[1];

                                // If the new coord is still on the board
                                if !(x < 0 || x > 7 || y < 0 || y > 7) {
                                    // Check square on board for piece
                                    match &self.board[Coord::XandY(x, y).get_index()] {
                                        Some(p) => { // If there is a piece
                                            // End the search in direction as piece can't jump
                                            finding = false; 

                                            // If the peice is opposite colour then its a valid move
                                            if p.side != side { 
                                                moves.push(Move { 
                                                    piece: Rook { has_moved: true }, 
                                                    from: piece.loc.clone(), 
                                                    to: Coord::XandY(x, y) 
                                                })
                                            }
                                        },
                                        None => { // If there is no piece then its a valid move
                                            moves.push(Move { 
                                                piece: Rook { has_moved: true }, 
                                                from: piece.loc.clone(), 
                                                to: Coord::XandY(x, y) 
                                            })
                                        }
                                    }
                                } else {
                                    finding = false;
                                }
                            }
                        }
                    },
                    Pawn { has_moved } => {
                        // Find which way is forward based off side
                        let direction: i8 = match side {
                            White => 1,
                            Black => -1
                        };

                        let [x, y] = piece.loc.get_x_and_y();

                        // Forward moves
                        if self.board[Coord::XandY(x, y + direction).get_index()].is_none() {
                            moves.push(Move {
                                piece: Pawn {has_moved: true},
                                from: Coord::XandY(x, y),
                                to: Coord::XandY(x, y + direction)
                            });
                        
                            // Two forward if it hasn't moved
                            if !has_moved && self.board[Coord::XandY(x, y + (2*direction)).get_index()].is_none() {
                                moves.push(Move {
                                    piece: Pawn {has_moved: true},
                                    from: Coord::XandY(x, y),
                                    to: Coord::XandY(x, y + (2*direction))
                                });
                            }
                        }

                        // Diagonal taking
                        for dx in [1, -1] {
                            if !(x + dx < 0 || x + dx > 7) {
                                self.board[Coord::XandY(x + dx, y + direction).get_index()]
                                .map(|p| {
                                    if p.side != side {
                                        moves.push(Move {
                                            piece: Pawn {has_moved: true},
                                            from: Coord::XandY(x, y),
                                            to: Coord::XandY(x + dx, y + direction)
                                        });
                                    }
                                });
                            }
                        }

                        // En passent


                    }
                }
            }
        }
        
        moves
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedCharacter
}

fn get_piece_char(piece: &Piece) -> char {
    let out_char = match piece.piece_type {
        King {has_moved: _} => 'K',
        Queen => 'Q',
        Bishop => 'B',
        Knight => 'N',
        Rook {has_moved: _} => 'R',
        Pawn {has_moved: _} => 'P'
    };

    if piece.side == Black {
        out_char.to_ascii_lowercase()
    } else {
        out_char
    }
}

fn board_from_string(code: &str) -> Result<[Option<Piece>; 64], ParseError> {
    let mut board:[Option<Piece>; 64] = [None; 64];

    // Loop through all the chars in the input string with an index
    for (i, c) in code.chars().enumerate() {
        // Work out the piece type
        let piece_type = match c.to_ascii_uppercase() {
            'K' => King {has_moved: false},
            'Q' => Queen,
            'B' => Bishop,
            'N' => Knight,
            'R' => Rook {has_moved: false},
            'P' => Pawn {has_moved: false},
            // If its an empty square then set it in the board and continue
            '.' => {board[i] = None; continue},
            // Error handling
            _ => {return Err(ParseError::UnexpectedCharacter)}
        };
    
        // Work out the side of the piece
        let side = if c.is_ascii_uppercase() { White } else { Black };

        // Add the piece to the board
        board[i] = Some(Piece{
            piece_type,
            side,
            loc: Coord::Index(i)
        });
    }

    Ok(board)
}