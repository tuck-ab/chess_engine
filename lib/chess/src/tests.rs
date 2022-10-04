use crate::*;

// Coords test
#[test]
fn coord_test() {
    assert_eq!(Coord::Index(0).as_x_and_y(), Coord::XandY(0, 7));
    assert_eq!(Coord::Index(63).as_x_and_y(), Coord::XandY(7, 0));
    assert_eq!(Coord::Index(12).as_x_and_y(), Coord::XandY(4, 6));

    assert_eq!(Coord::XandY(0, 7).as_index(), Coord::Index(0));
    assert_eq!(Coord::XandY(7, 0).as_index(), Coord::Index(63));
    assert_eq!(Coord::XandY(0, 0).as_index(), Coord::Index(56));
}