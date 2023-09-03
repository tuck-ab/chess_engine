use crate::coord::*;
use crate::game::*;
use crate::moves::*;
use crate::pieces::*;

#[test]
fn coord_test() {
    assert_eq!(Coord::from_index(0), Coord::from_x_and_y(0, 7));
    assert_eq!(Coord::from_index(63), Coord::from_x_and_y(7, 0));
    assert_eq!(Coord::from_index(12), Coord::from_x_and_y(4, 6));

    assert_eq!(Coord::from_x_and_y(0, 7), Coord::from_index(0));
    assert_eq!(Coord::from_x_and_y(7, 0), Coord::from_index(63));
    assert_eq!(Coord::from_x_and_y(0, 0), Coord::from_index(56));
}

#[test]
fn king_into_check_test() {
    let start_code = "..K.k...\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let white_king = game.get_white_king();
    let moves = get_piece_moves(&game, white_king);

    assert_eq!(moves.len(), 3)
}

#[test]
fn pin_test() {
    let start_code = "....k...\
                      ........\
                      ........\
                      ........\
                      ........\
                      ....q...\
                      ....Q...\
                      ....K...";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let white_king = game.get_white_king();
    let moves = get_piece_moves(&game, white_king);

    assert_eq!(moves.len(), 2)
}

#[test]
fn king_moves() {
    let start_code = "....k...\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ....K...";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let white_king = game.get_white_king();
    let moves = get_piece_moves(&game, white_king);

    assert_eq!(moves.len(), 5);
    assert!(moves.contains(&Move::Standard(StandardMove::new(
        Piece::new(Side::White, false, PieceType::King, Coord::from_x_and_y(4, 0)),
        Coord::from_x_and_y(4, 0),
        Coord::from_x_and_y(3, 0)
    ))));
    assert!(!moves.contains(&Move::Standard(StandardMove::new(
        Piece::new(Side::White, false, PieceType::King, Coord::from_x_and_y(4, 0)),
        Coord::from_x_and_y(4, 0),
        Coord::from_x_and_y(5, 5)
    ))));
}

#[test]
fn queen_moves() {
    let start_code = "....k..K\
                      Q.......\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let piece = Piece::new(
        Side::White,
        false,
        PieceType::Queen,
        Coord::from_x_and_y(0, 6)
    );

    let moves = get_piece_moves(&game, piece);

    assert_eq!(moves.len(), 7+7+1+6);

    assert!(moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 6),
        Coord::from_x_and_y(0, 0)
    ))));

    assert!(!moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 6),
        Coord::from_x_and_y(1, 1)
    ))));
}

#[test]
fn bishop_moves() {
    let start_code = "....k..K\
                      B.......\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let piece = Piece::new(
        Side::White,
        false,
        PieceType::Bishop,
        Coord::from_x_and_y(0, 6)
    );

    let moves = get_piece_moves(&game, piece);

    assert_eq!(moves.len(), 1+6);

    assert!(moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 6),
        Coord::from_x_and_y(1, 7)
    ))));

    assert!(!moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 6),
        Coord::from_x_and_y(1, 1)
    ))));
}

#[test]
fn rook_moves() {
    let start_code = "....k..K\
                      R.......\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let piece = Piece::new(
        Side::White,
        false,
        PieceType::Rook,
        Coord::from_x_and_y(0, 6)
    );

    let moves = get_piece_moves(&game, piece);

    assert_eq!(moves.len(), 7+7);

    assert!(moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 6),
        Coord::from_x_and_y(0, 0)
    ))));

    assert!(!moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 6),
        Coord::from_x_and_y(1, 1)
    ))));
}

#[test]
fn knight_moves() {
    let start_code = "....k..K\
                      N.......\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let piece = Piece::new(
        Side::White,
        false,
        PieceType::Knight,
        Coord::from_x_and_y(0, 6)
    );

    let moves = get_piece_moves(&game, piece);

    assert_eq!(moves.len(), 3);

    assert!(moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 6),
        Coord::from_x_and_y(2, 7)
    ))));

    assert!(!moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 6),
        Coord::from_x_and_y(1, 1)
    ))));
}

#[test]
fn castle_test() {
    let start_code = "r...k..r\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ..R.K...";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let piece = Piece::new(
        Side::Black,
        false,
        PieceType::King,
        Coord::from_x_and_y(4, 7)
    );

    let moves = get_piece_moves(&game, piece);

    assert_eq!(moves.len(), 6);

    assert!(moves.contains(&Move::Castle(Castle::new(
        piece,
        Piece::new(Side::Black, false, PieceType::Rook, Coord::from_x_and_y(7, 7)),
        Coord::from_x_and_y(4, 7),
        Coord::from_x_and_y(7, 7),
        Coord::from_x_and_y(6, 7),
        Coord::from_x_and_y(5, 7)
    ))));

    assert!(!moves.contains(&Move::Castle(Castle::new(
        piece,
        Piece::new(Side::Black, false, PieceType::Rook, Coord::from_x_and_y(0, 7)),
        Coord::from_x_and_y(4, 7),
        Coord::from_x_and_y(0, 7),
        Coord::from_x_and_y(2, 7),
        Coord::from_x_and_y(3, 7)
    ))));

    let start_code = "r...k..r\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      ....K...";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let piece = Piece::new(
        Side::Black,
        false,
        PieceType::King,
        Coord::from_x_and_y(4, 7)
    );

    let moves = get_piece_moves(&game, piece);

    assert!(moves.contains(&Move::Castle(Castle::new(
        piece,
        Piece::new(Side::Black, false, PieceType::Rook, Coord::from_x_and_y(0, 7)),
        Coord::from_x_and_y(4, 7),
        Coord::from_x_and_y(0, 7),
        Coord::from_x_and_y(2, 7),
        Coord::from_x_and_y(3, 7)
    ))));
}

#[test]
fn pawn_forward_move() {
    let start_code = "....k...\
                      ........\
                      ........\
                      ........\
                      ........\
                      ........\
                      P.......\
                      ....K...";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let piece = Piece::new(
        Side::White,
        false,
        PieceType::Pawn,
        Coord::from_x_and_y(0, 1)
    );

    let moves = get_piece_moves(&game, piece);

    assert_eq!(moves.len(), 2);

    assert!(moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 1),
        Coord::from_x_and_y(0, 3)
    ))));

    assert!(moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 1),
        Coord::from_x_and_y(0, 2)
    ))));

    assert!(!moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 1),
        Coord::from_x_and_y(1, 2)
    ))));

    let start_code = "....k...\
                      ........\
                      ........\
                      ........\
                      .b......\
                      ........\
                      ...P....\
                      ....K...";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let piece = Piece::new(
        Side::White,
        false,
        PieceType::Pawn,
        Coord::from_x_and_y(3, 1)
    );

    let moves = get_piece_moves(&game, piece);

    assert_eq!(moves.len(), 0);
}

#[test]
fn pawn_capturing() {
    let start_code = "....k...\
                      ........\
                      ........\
                      ........\
                      ........\
                      ..b.....\
                      .P......\
                      ....K...";
    let game = Game::from_string(start_code, Side::White).unwrap();
    let piece = Piece::new(
        Side::White,
        false,
        PieceType::Pawn,
        Coord::from_x_and_y(1, 1)
    );

    let moves = get_piece_moves(&game, piece);

    // Taking the bishop is the only legal move
    assert_eq!(moves.len(), 1);

    assert!(moves.contains(&Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(1, 1),
        Coord::from_x_and_y(2, 2)
    ))));
}

#[test]
fn pawn_promotion() {
    let start_code = "....k...\
                      ........\
                      P.......\
                      ........\
                      ........\
                      ........\
                      ........\
                      ....K...";
    let mut game = Game::from_string(start_code, Side::White).unwrap();
    let piece = game.get_piece_at(Coord::from_x_and_y(0, 5)).unwrap();

    game.apply_unchecked_move(Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(0, 5),
        Coord::from_x_and_y(0, 6)
    )), false);

    let moves = get_piece_moves(&game, game.get_piece_at(Coord::from_x_and_y(0, 6)).unwrap());
    assert_eq!(moves.len(), 4);

    let promoting_move = Move::Promotion(Promotion::new(
        game.get_piece_at(Coord::from_x_and_y(0, 6)).unwrap(),
        Piece::new(
            Side::White,
            true,
            PieceType::Queen,
            Coord::from_x_and_y(0, 7)
        ),
        Coord::from_x_and_y(0, 6),
        Coord::from_x_and_y(0, 7)
    ));

    assert!(moves.contains(&promoting_move));

    game.apply_unchecked_move(promoting_move, false);

    assert!(game.get_piece_at(Coord::from_x_and_y(0, 7)).is_some());
    assert!(game.get_piece_at(Coord::from_x_and_y(0, 7)).unwrap().is_type(PieceType::Queen))
}

#[test]
fn en_passant() {
    let start_code = "....k...\
                      .p......\
                      ........\
                      ..P.....\
                      ........\
                      ........\
                      ........\
                      ....K...";
    let mut game = Game::from_string(start_code, Side::Black).unwrap();
    let black_piece = game.get_piece_at(Coord::from_x_and_y(1, 6)).unwrap();

    game.apply_unchecked_move(Move::Standard(StandardMove::new(
        black_piece, 
        Coord::from_x_and_y(1, 6),
        Coord::from_x_and_y(1, 4)
    )), false);

    let moves = game.get_valid_moves();
    let test_move = Move::EnPassant(EnPassant { 
        piece: game.get_piece_at(Coord::from_x_and_y(2, 4)).unwrap(), 
        from: Coord::from_x_and_y(2, 4), 
        to: Coord::from_x_and_y(1, 5), 
        piece_taken: game.get_piece_at(Coord::from_x_and_y(1, 4)).unwrap(), 
        coord_taken: Coord::from_x_and_y(1, 4) 
    });

    assert!(moves.contains(&test_move));

    game.apply_move(Move::Standard(StandardMove::new(
        game.get_piece_at(Coord::from_x_and_y(4, 0)).unwrap(), 
        Coord::from_x_and_y(4, 0),
        Coord::from_x_and_y(5, 0)
    ))).unwrap();
    game.apply_move(Move::Standard(StandardMove::new(
        game.get_piece_at(Coord::from_x_and_y(4, 7)).unwrap(), 
        Coord::from_x_and_y(4, 7),
        Coord::from_x_and_y(5, 7)
    ))).unwrap();

    let moves = game.get_valid_moves();
    assert!(!moves.contains(&test_move));
}

#[test]
fn checkmate() {
    let start_code = ".....K.k\
                            ........\
                            ........\
                            ........\
                            ........\
                            ........\
                            ........\
                            ......Q.";
    let mut game = Game::from_string(start_code, Side::White).unwrap();
    let piece = game.get_piece_at(Coord::from_x_and_y(6, 0)).unwrap();

    game.apply_unchecked_move(Move::Standard(StandardMove::new(
        piece, 
        Coord::from_x_and_y(6, 0),
        Coord::from_x_and_y(6, 7)
    )), true);

    assert_eq!(game.get_valid_moves().len(), 0);
    assert_eq!(game.get_winner(), Some(Side::White));
}
