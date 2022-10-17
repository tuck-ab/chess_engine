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
    let rook_test_code = "r..............................................................R";
    let mut game = Game::from_string(rook_test_code).unwrap();

    let moves = game.get_all_moves(White);

    assert_eq!(moves.len(), 14);
}