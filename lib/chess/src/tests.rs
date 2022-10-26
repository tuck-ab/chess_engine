use crate::*;

// Coords test
#[test]
fn coord_test() {
    assert_eq!(Coord::Index(0), Coord::XandY(0, 7));
    assert_eq!(Coord::Index(63), Coord::XandY(7, 0));
    assert_eq!(Coord::Index(12), Coord::XandY(4, 6));

    assert_eq!(Coord::XandY(0, 7), Coord::Index(0));
    assert_eq!(Coord::XandY(7, 0), Coord::Index(63));
    assert_eq!(Coord::XandY(0, 0), Coord::Index(56));
}

#[test]
fn piece_tests() {
    let test_piece_white = Piece {
        piece_type: Queen,
        side: White,
        loc: Coord::XandY(4, 4)
    };

    let test_piece_black = Piece {
        piece_type: Queen,
        side: Black,
        loc: Coord::XandY(5, 5)
    };

    assert!(test_piece_black.is_black());
    assert!(test_piece_white.is_white());
    assert!(!test_piece_black.is_white());
    assert!(!test_piece_white.is_black());

    assert!(!test_piece_black.is_same_side(&test_piece_white))
}

#[test]
fn game_test() {
    let game = Game::new();

    assert_eq!(game.pieces.len(), 32);
    assert_eq!(game.board[Coord::XandY(0,0).get_index()], Some(Piece {
        piece_type: Rook { has_moved: false },
        side: White,
        loc: Coord::XandY(0,0)
    }))
}

#[test]
fn rook_test() {
    let rook_test_code = "r.......p......p...............................................R";
    let game = Game::from_string(rook_test_code).unwrap();

    let moves = game.get_all_moves(White);

    assert_eq!(moves.len(), 13);

    let possible_move = Move {
        piece: Rook { has_moved: true },
        from: Coord::XandY(7, 0),
        to: Coord::XandY(7, 5)
    };

    let not_possible_move = Move {
        piece: Rook {has_moved: true},
        from: Coord::XandY(7, 0),
        to: Coord::XandY(7, 7)
    };

    let takes_possible_move = Move {
        piece: Rook { has_moved: true },
        from: Coord::XandY(7, 0),
        to: Coord::XandY(7, 6)
    };

    assert!(moves.contains(&possible_move));
    assert!(!moves.contains(&not_possible_move));
    assert!(moves.contains(&takes_possible_move));
}

#[test]
fn bishop_test() {
    let bishop_test_code = ".........B............................................p.........";
    let game = Game::from_string(bishop_test_code).unwrap();

    let moves = game.get_all_moves(White);

    assert_eq!(moves.len(), 8);

    let possible_move = Move {
        piece: Bishop,
        from: Coord::XandY(1, 6),
        to: Coord::XandY(4, 3)
    };

    let not_possible_move = Move {
        piece: Bishop,
        from: Coord::XandY(1, 6),
        to: Coord::XandY(7, 0)
    };

    let takes_possible_move = Move {
        piece: Bishop,
        from: Coord::XandY(1, 6),
        to: Coord::XandY(6, 1)
    };

    assert!(moves.contains(&possible_move));
    assert!(!moves.contains(&not_possible_move));
    assert!(moves.contains(&takes_possible_move));
}

#[test]
fn queen_test() {
    let queen_test_code = ".........Q.......................................p....p.........";
    let game = Game::from_string(queen_test_code).unwrap();

    let moves = game.get_all_moves(White);

    // 8 Bishop like moves, 13 Rook like moves
    assert_eq!(moves.len(), 8 + 13);

    let possible_move_diag = Move {
        piece: Queen,
        from: Coord::XandY(1, 6),
        to: Coord::XandY(4, 3)
    };

    let possible_move_vert = Move {
        piece: Queen,
        from: Coord::XandY(1, 6),
        to: Coord::XandY(7, 6)
    };

    let impossible_move_diag = Move {
        piece: Queen,
        from: Coord::XandY(1, 6),
        to: Coord::XandY(7, 0)
    };

    let impossible_move_vert = Move {
        piece: Queen,
        from: Coord::XandY(1, 6),
        to: Coord::XandY(1, 0)
    };

    let takes_move_diag = Move {
        piece: Queen,
        from: Coord::XandY(1, 6),
        to: Coord::XandY(6, 1)
    };

    let takes_move_vert = Move {
        piece: Queen,
        from: Coord::XandY(1, 6),
        to: Coord::XandY(1, 1)
    };

    assert!(moves.contains(&possible_move_diag));
    assert!(moves.contains(&possible_move_vert));
    assert!(!moves.contains(&impossible_move_diag));
    assert!(!moves.contains(&impossible_move_vert));
    assert!(moves.contains(&takes_move_diag));
    assert!(moves.contains(&takes_move_vert));
}

#[test]
fn knight_test() {
    let knight_test_code = "...................N............................................";
    let game = Game::from_string(knight_test_code).unwrap();

    let moves = game.get_all_moves(White);

    assert_eq!(moves.len(), 8);

    let possible_move = Move {
        piece: Knight,
        from: Coord::XandY(3, 5),
        to: Coord::XandY(1, 6)
    };

    assert!(moves.contains(&possible_move))
}

#[test]
fn pawn_test() {
    let basic_pawn_test_code = "............................................q.q......P.P........";
    let game = Game::from_string(basic_pawn_test_code).unwrap();

    let moves = game.get_all_moves(White);

    assert_eq!(moves.len(), 7);

    let possible_takes_move = Move {
        piece: Pawn {has_moved: true},
        from: Coord::XandY(7, 1),
        to: Coord::XandY(6, 2)
    };



    assert!(moves.contains(&possible_takes_move));
}