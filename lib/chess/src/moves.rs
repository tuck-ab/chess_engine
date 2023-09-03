use std::collections::HashSet;

use crate::pieces::{Piece, Side, PieceType, PROMOTABLE_PIECES};
use crate::coord::Coord;
use crate::game::Game;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Move {
    Standard(StandardMove),
    Castle(Castle),
    Promotion(Promotion),
    EnPassant(EnPassant)
}

impl Move {
    pub fn get_from(&self) -> Coord {
        match *self {
            Self::Standard(move_) => move_.from,
            Self::Castle(move_) => move_.king_from,
            Self::Promotion(move_) => move_.from,
            Self::EnPassant(move_) => move_.from
        }
    }

    pub fn get_to(&self) -> Coord {
        match *self {
            Self::Standard(move_) => move_.to,
            Self::Castle(move_) => move_.king_to,
            Self::Promotion(move_) => move_.to,
            Self::EnPassant(move_) => move_.to
        }
    }

    pub fn get_piece(&self) -> Piece {
        match *self {
            Self::Standard(move_) => move_.piece,
            Self::Castle(move_) => move_.king_piece,
            Self::Promotion(move_) => move_.old_piece,
            Self::EnPassant(move_) => move_.piece
        }
    }

    pub fn get_side(&self) -> Side {
        self.get_piece().get_side()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StandardMove {
    pub piece: Piece,
    pub from: Coord,
    pub to: Coord,
}

impl StandardMove {
    pub fn new(piece: Piece, from: Coord, to: Coord) -> Self {
        StandardMove{piece, from, to}
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Castle {
    pub king_piece: Piece,
    pub rook_piece: Piece,
    pub king_from: Coord,
    pub rook_from: Coord,
    pub king_to: Coord,
    pub rook_to: Coord
}

impl Castle {
    pub fn new(king_piece: Piece,
               rook_piece: Piece,
               king_from: Coord,
               rook_from: Coord,
               king_to: Coord,
               rook_to: Coord)
               -> Self {
        Castle{rook_piece, king_piece, king_from, rook_from, king_to, rook_to}
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Promotion {
    /// The Pawn being promoted
    pub old_piece: Piece, 
    /// The New promoted piece
    pub new_piece: Piece,
    /// Location of the Pawn
    pub from: Coord,
    /// Location of the new Piece
    pub to: Coord
}

impl Promotion {
    pub fn new(old_piece: Piece, new_piece: Piece, from: Coord, to: Coord) -> Self {
        Self { old_piece, new_piece, from, to }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EnPassant {
    pub piece: Piece,
    pub from: Coord,
    pub to: Coord,
    pub piece_taken: Piece,
    pub coord_taken: Coord
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveError{
    InvalidMove
}

/// Returns target squares for moves in a given direction from a source square.
/// The searching checks for other pieces on the board to prevent a target
/// being behind another piece.
/// 
/// The `king_limit` is to limit the search to one square as the king usually
/// can only move one square.
fn get_target_locs_in_dir(game: &Game,
                          source: Coord, 
                          dir: [i8; 2], 
                          king_limit: bool,
                          cur_side: Side) 
                          -> Vec<Coord> {
    let [mut x, mut y] = source.get_x_and_y();

    let mut targets: Vec<Coord> = Vec::new();

    #[allow(while_true)]
    'searcher: while true {
        x += dir[0];
        y += dir[1];

        // If the new pos is off the board then break out
        if x < 0 || x > 7 || y < 0 || y > 7 {
            break 'searcher
        }

        match game.get_piece_at(Coord::from_x_and_y(x, y)) {
            Some(piece) => {
                if piece.get_side() != cur_side {
                    targets.push(Coord::from_x_and_y(x, y))
                };

                break 'searcher;
            },
            None => targets.push(Coord::from_x_and_y(x, y))
        }

        // If its a king then they only need to search through once
        if king_limit { break 'searcher }
    }

    targets
}

fn get_king_targets(game: &Game, piece: Piece) -> Vec<Coord> {
    let mut targets: Vec<Coord> = Vec::new();
    let dirs = [
        [1,1], [-1,1], [1,-1], [-1,-1], 
        [1,0], [0,1], [-1,0], [0,-1]
    ];

    for dir in dirs {
        targets.append(&mut get_target_locs_in_dir(
            game, 
            piece.get_loc(), 
            dir, 
            true, 
            piece.get_side()
        ));
    }

    targets
}

fn get_queen_targets(game: &Game, piece: Piece) -> Vec<Coord> {
    let mut targets: Vec<Coord> = Vec::new();
    let dirs = [
        [1,1], [-1,1], [1,-1], [-1,-1], 
        [1,0], [0,1], [-1,0], [0,-1]
    ];

    for dir in dirs {
        targets.append(&mut get_target_locs_in_dir(
            game, 
            piece.get_loc(), 
            dir, 
            false, 
            piece.get_side()
        ));
    }

    targets
}


pub fn get_bishop_targets(game: &Game, piece: Piece) -> Vec<Coord> {
    let mut targets: Vec<Coord> = Vec::new();
    let dirs = [[1,1], [-1,1], [1,-1], [-1,-1]];

    for dir in dirs {
        targets.append(&mut get_target_locs_in_dir(
            game, 
            piece.get_loc(), 
            dir, 
            false, 
            piece.get_side()
        ));
    }

    targets
}


pub fn get_rook_targets(game: &Game, piece: Piece) -> Vec<Coord> {
    let mut targets: Vec<Coord> = Vec::new();
    let dirs = [[1,0], [0,1], [-1,0], [0,-1]];

    for dir in dirs {
        targets.append(&mut get_target_locs_in_dir(
            game, 
            piece.get_loc(), 
            dir, 
            false, 
            piece.get_side()
        ));
    }

    targets
}


pub fn get_knight_targets(game: &Game, piece: Piece) -> Vec<Coord> {
    let mut targets: Vec<Coord> = Vec::new();
    let dirs = [
        [2,1], [2,-1], [1,2], [1,-2], 
        [-2, 1], [-2,-1], [-1,2], [-1,-2]
    ];

    for dir in dirs {
        let [mut x, mut y] = piece.get_loc().get_x_and_y();

        x += dir[0];
        y += dir[1];

        // If the new pos is off the board then continue to next square
        if x < 0 || x > 7 || y < 0 || y > 7 {continue}

        match game.get_piece_at(Coord::from_x_and_y(x, y)) {
            Some(other_piece) => {
                if other_piece.get_side() != piece.get_side() {
                    targets.push(Coord::from_x_and_y(x, y))
                }
            },
            None => targets.push(Coord::from_x_and_y(x, y))
        }
    }

    targets
}

fn get_pawn_standard_targets(game: &Game, piece: Piece) -> Vec<Coord> {
    let mut targets: Vec<Coord> = Vec::new();

    let dir = match piece.get_side() {
        Side::White => 1,
        Side::Black => -1
    };

    for dx in [-1, 1] {
        let x = piece.get_loc().get_x_and_y()[0] + dx;
        let y = piece.get_loc().get_x_and_y()[1] + dir;

        // If the new pos is off the board then continue
        if x < 0 || x > 7 || y < 0 || y > 7 {continue}

        match game.get_piece_at(Coord::from_x_and_y(x, y)) {
            Some(other_piece) => {
                if other_piece.get_side() != piece.get_side() {
                    targets.push(Coord::from_x_and_y(x, y))
                }
            },
            None => {}
        }
    }

    targets
}


pub fn get_piece_standard_targets(game: &Game, piece: Piece) -> HashSet<Coord> {
    match piece.get_type() {
        PieceType::King => HashSet::from_iter(get_king_targets(game, piece)),
        PieceType::Queen => HashSet::from_iter(get_queen_targets(game, piece)),
        PieceType::Bishop => HashSet::from_iter(get_bishop_targets(game, piece)),
        PieceType::Rook => HashSet::from_iter(get_rook_targets(game, piece)),
        PieceType::Knight => HashSet::from_iter(get_knight_targets(game, piece)),
        PieceType::Pawn => HashSet::from_iter(get_pawn_standard_targets(game, piece))
    }
}

pub fn get_side_targets(game: &Game, side: Side) -> HashSet<Coord> {
    let mut targets: HashSet<Coord> = HashSet::new();

    for piece in game.iter() {
        if piece.is_side(side) {
            targets.extend(get_piece_standard_targets(game, piece))
        }
    }

    targets
}

fn get_pawn_moves(game: &Game, piece: Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let dir = match piece.get_side() {
        Side::White => 1,
        Side::Black => -1
    };

    let [x, y] = piece.get_loc().get_x_and_y();
    if game.get_piece_at(Coord::from_x_and_y(x, y + dir)).is_none() {
        moves.push(Move::Standard(StandardMove { 
            piece, 
            from: piece.get_loc(), 
            to: Coord::from_x_and_y(x, y + dir) 
        }));

        // If the square is empty and the pawn hasn't move then it can double move
        if !piece.has_moved() && 
           game.get_piece_at(Coord::from_x_and_y(x, y + 2*dir)).is_none() {
            moves.push(Move::Standard(StandardMove { 
                piece, 
                from: piece.get_loc(), 
                to: Coord::from_x_and_y(x, y + 2*dir) 
            }));
        }

        // If the square is at the end of the board then it can promote
    }

    moves
}

fn get_castle_moves(game: &Game, piece: Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let y = match piece.get_side() {
        Side::White => 0,
        Side::Black => 7
    };

    let targets = get_side_targets(game, piece.get_side().other());

    // Short castle
    // Check Rook hasn't moved
    if game.get_piece_at(Coord::from_x_and_y(7, y))
           .map(|p| p.is_type(PieceType::Rook) && 
                           !p.has_moved() && 
                           // Strictly speaking this last check is not 
                           // neccessary as a piece that hasn't moved in the 
                           // correct square should be of the same type
                           p.is_side(piece.get_side())) 
           .unwrap_or(false) &&
    // Check the spaces are empty
    [5, 6].map(|x| game.get_piece_at(Coord::from_x_and_y(x, y)).is_none())
          .into_iter()
          .fold(true, |acc, x| acc && x) &&
    // Check the spaces the king move aren't in check            
    [5, 6].map(|x| !targets.contains(&Coord::from_x_and_y(x, y)))
          .into_iter()
          .fold(true, |acc, x| acc && x) {
        moves.push(Move::Castle(Castle::new(
            piece, 
            game.get_piece_at(Coord::from_x_and_y(7, y)).unwrap(),
            piece.get_loc(),
            Coord::from_x_and_y(7, y),
            Coord::from_x_and_y(6, y),
            Coord::from_x_and_y(5, y)
        )))
    }


    // Check long castle
    if game.get_piece_at(Coord::from_x_and_y(0, y))
           .map(|p| p.is_type(PieceType::Rook) && 
                           !p.has_moved() && 
                           // Strictly speaking this last check is not 
                           // neccessary as a piece that hasn't moved in the 
                           // correct square should be of the same type
                           p.is_side(piece.get_side())) 
           .unwrap_or(false) &&
    // Check the spaces are empty
    [1, 2, 3].map(|x| game.get_piece_at(Coord::from_x_and_y(x, y)).is_none())
          .into_iter()
          .fold(true, |acc, x| acc && x) &&
    // Check the spaces the king move aren't in check            
    [2, 3].map(|x| !targets.contains(&Coord::from_x_and_y(x, y)))
          .into_iter()
          .fold(true, |acc, x| acc && x) {
        moves.push(Move::Castle(Castle::new(
            piece, 
            game.get_piece_at(Coord::from_x_and_y(0, y)).unwrap(),
            piece.get_loc(),
            Coord::from_x_and_y(0, y),
            Coord::from_x_and_y(2, y),
            Coord::from_x_and_y(3, y)
        )))
    }

    moves
}

pub fn get_piece_moves(game: &Game, piece: Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for target in get_piece_standard_targets(game, piece) {
        let move_: Move = Move::Standard(StandardMove::new(piece, piece.get_loc(), target));

        // Move is only valid if not in check after
        if !is_in_check_after_move(game, move_) {
            if piece.is_type(PieceType::Pawn) &&
               move_.get_to().get_x_and_y()[1] == piece.get_side().get_promotion_y() {
                add_promotions(move_, &mut moves)
            } else {
                moves.push(move_)
            }
        }
    }

    // Add non-targeting pawn moves and en passant
    if piece.is_type(PieceType::Pawn) {
        for move_ in get_pawn_moves(game, piece) {
            // Check if the move results in a pawn promotion
            if !is_in_check_after_move(game, move_) {
                if move_.get_to().get_x_and_y()[1] == piece.get_side().get_promotion_y() {
                    add_promotions(move_, &mut moves)
                } else {
                    moves.push(move_)
                }
            }
        }

        if game.get_previous_move()
        .map(
            |m| m.get_piece().is_type(PieceType::Pawn) && 
            ((piece.get_loc().get_x_and_y()[0] - m.get_to().get_x_and_y()[0]).abs() == 1) &&
            ((m.get_from().get_x_and_y()[1] - m.get_to().get_x_and_y()[1]).abs() == 2)
        ).unwrap_or(false) {
            moves.push(Move::EnPassant(EnPassant { 
                piece, 
                from: piece.get_loc(), 
                to: Coord::from_x_and_y(
                    game.get_previous_move().unwrap().get_piece().get_loc().get_x_and_y()[0], 
                    piece.get_loc().get_x_and_y()[1] + piece.get_side().get_dir()), 
                piece_taken: game.get_piece_at(game.get_previous_move().unwrap().get_to()).unwrap(), 
                coord_taken: game.get_previous_move().unwrap().get_to()
            }))
        }
    }

    // Check for castling
    // Making assumption that the unmoved Rooks are in the corner
    // This will need to be changed for chess 960
    if piece.is_type(PieceType::King) && !piece.has_moved() {
        moves.append(&mut get_castle_moves(game, piece))
    }

    // Check for enPassent
    
    moves
}

fn add_promotions(old_move: Move, moves: &mut Vec<Move>) {
    let old_piece = old_move.get_piece();
    let promotion_square = old_move.get_to();
    for promote_to in PROMOTABLE_PIECES {
        moves.push(Move::Promotion(Promotion { 
            old_piece, 
            new_piece: Piece::new(
                old_piece.get_side(), 
                true, 
                promote_to, 
                promotion_square), 
            from: old_move.get_from(), 
            to: promotion_square 
        }))
    }
}

fn is_in_check_after_move(game: &Game, move_: Move) -> bool {
    (*game).clone().apply_unchecked_move(move_).is_side_in_check(move_.get_side())
}
