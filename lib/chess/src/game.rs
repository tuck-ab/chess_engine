use std::collections::HashSet;

use crate::moves::{Move, get_side_targets, get_piece_moves, MoveError};
use crate::pieces::*;
use crate::coord::*;

#[derive(Clone, Copy)]
pub struct Game {
    board: [Option<Piece>; 64],
    white_king_loc: Coord,
    black_king_loc: Coord,
    current_turn: Side,
    previous_move: Option<Move>
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
        write!(f, "{:?} to move", self.current_turn)?;

        Ok(())
    }
}

impl Game {
    pub fn new() -> Self {
        let start_code = "rnbqkbnr\
                                pppppppp\
                                ........\
                                ........\
                                ........\
                                ........\
                                PPPPPPPP\
                                RNBQKBNR";

        Game::from_string(start_code, Side::White).unwrap()
    }


    /// Returns the square on the board given by a Coord enum.
    pub fn get_piece_at(&self, coord: Coord) -> Option<Piece> {
        self.board[coord.get_index()]
    }

    /// Gets a Piece struct for the White King on the board
    /// 
    /// # Panics
    /// 
    /// Panics if the square the board thinks the king is on is empty
    pub fn get_white_king(&self) -> Piece {
        self.board[self.white_king_loc.get_index()].unwrap()
    }


    /// Gets a Piece struct for the Black King on the board
    /// 
    /// # Panics
    /// 
    /// Panics if the square the board thinks the king is on is empty
    pub fn get_black_king(&self) -> Piece {
        self.board[self.black_king_loc.get_index()].unwrap()
    }


    pub fn apply_move(&mut self, move_: Move) -> Result<(), MoveError> {
        if self.get_valid_moves().contains(&move_) {
            // Move has been checked so can use the unsafe function
            self.apply_unchecked_move(move_);
        } else {
            return Err(MoveError::InvalidMove)
        }

        Ok(())
    }


    /// Force the move to be made on the board. There is no check to
    /// ensure the move is valid. Ensure move is valid before calling this
    /// function otherwise unexpected behaviour may be experienced. If unsure
    /// whether move is valid use the safer `apply_move` function
    pub fn apply_unchecked_move(&mut self, move_: Move) -> Self {
        match move_ {
            Move::Standard(m) => {
                // Make square "to" have piece
                self.board[m.to.get_index()] = Some(m.piece.clone().move_to(m.to));

                // Remove the piece from the old position
                self.board[m.from.get_index()] = None;

                // If the piece is a king then the parameters in self need updating
                if m.piece.is_type(PieceType::King) {
                    match m.piece.get_side() {
                        Side::White => self.white_king_loc = m.to,
                        Side::Black => self.black_king_loc = m.to
                    }
                };
            },
            Move::Castle(m) => {
                // Move king and rook
                self.board[m.king_to.get_index()] = Some(m.king_piece.clone().move_to(m.king_to));
                self.board[m.rook_to.get_index()] = Some(m.rook_piece.clone().move_to(m.rook_to));

                // Remove king and rook from old squares
                self.board[m.king_from.get_index()] = None;
                self.board[m.rook_from.get_index()] = None;

                // Update the king
                match m.king_piece.get_side() {
                    Side::White => self.white_king_loc = m.king_to,
                    Side::Black => self.black_king_loc = m.king_to
                };
            },
            Move::Promotion(_m) => {
                todo!("Implimentation for promotion")
            }
        }

        // Swap the turn player
        self.current_turn = self.current_turn.other();

        // Update the previous move
        self.previous_move = Some(move_);

        *self
    }


    pub fn is_side_in_check(&self, side: Side) -> bool {
        let other_targets: HashSet<Coord> = get_side_targets(self, side.other());

        let result = other_targets.contains(match side {
            Side::White => &(self.white_king_loc),
            Side::Black => &(self.black_king_loc)
        });

        result
    }


    pub fn get_valid_moves(&self) -> Vec<Move> {
        self.iter()
            .filter(|x| x.is_side(self.current_turn))
            .map(|x| get_piece_moves(self, x))
            .fold(Vec::<Move>::new(), |mut acc, mut x| {acc.append(&mut x); acc})
    }
}

pub struct GameIter<'a> {
    game: &'a Game,
    i: usize
}

impl Game {
    pub fn iter(&self) -> GameIter<'_> {
        GameIter {
            game: self,
            i: 0
        }
    }
}

impl<'a> Iterator for GameIter<'a> {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;

        if self.i <= self.game.board.len() {
            match self.game.board[self.i-1] {
                Some(item) => Some(item),
                None => self.next()
            }
        } else {
           None
        }
    }
}

/// Implimentation for I/O of games

/// Error types from parsing strings representing the board
#[derive(Debug)]
pub enum BoardStringParseError {
    UnexpectedCharacter,
    NoWhiteKing,
    NoBlackKing
}

impl Game {
    pub fn from_string(code: &str, start_side: Side) -> Result<Self, BoardStringParseError>{
        let board = board_from_string(code)?;

        let mut white_king_loc: Option<Coord> = None;
        let mut black_king_loc: Option<Coord> = None;

        for space in board {
            // If its an empty space then continue
            if space.is_none() {continue}

            let piece = space.unwrap();

            // Check if the piece is a king to get their locations
            if piece.is_type(PieceType::King) {
                match piece.get_side() {
                    Side::White => white_king_loc = Some(piece.get_loc()),
                    Side::Black => black_king_loc = Some(piece.get_loc())
                }
            }
        }

        // Ensure there are kings
        if white_king_loc.is_none() 
            {return Err(BoardStringParseError::NoWhiteKing)}
        if black_king_loc.is_none()
            {return Err(BoardStringParseError::NoBlackKing)}

        Ok(Self{
            board, 
            white_king_loc: white_king_loc.unwrap(), 
            black_king_loc: black_king_loc.unwrap(),
            current_turn: start_side,
            previous_move: None
        })
    }
}

fn board_from_string(code: &str) -> Result<[Option<Piece>; 64], BoardStringParseError> {
    let mut board:[Option<Piece>; 64] = [None; 64];

    // Loop through all the chars in the input string with an index
    for (i, c) in code.chars().enumerate() {
        // Work out the piece type
        let piece_type = match c.to_ascii_uppercase() {
            'K' => PieceType::King,
            'Q' => PieceType::Queen,
            'B' => PieceType::Bishop,
            'N' => PieceType::Knight,
            'R' => PieceType::Rook,
            'P' => PieceType::Pawn,
            // If its an empty square then set it in the board and continue
            '.' => {board[i] = None; continue},
            // Error handling
            _ => {return Err(BoardStringParseError::UnexpectedCharacter)}
        };
    
        // Work out the side of the piece
        let side = if c.is_ascii_uppercase() { Side::White } 
                         else { Side::Black };

        // Add the piece to the board
        board[i] = Some(
            Piece::new(side, false, piece_type, Coord::from_index(i))
        );
    }

    Ok(board)
}