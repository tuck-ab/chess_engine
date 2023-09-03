#[derive(Copy, Clone, Hash)]
/// Coordinate system for the board
pub struct Coord {
    index: usize
}

impl Eq for Coord {}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [x, y] = self.get_x_and_y();
        write!(f, "Coord: ({}, {})", x, y)?;
        Ok(())
    }
}

impl Coord {
    pub fn from_index(index: usize) -> Self {
        Self {index}
    }

    pub fn from_x_and_y(x: i8, y: i8) -> Self {
        Self {index: x as usize + (8 * (7-(y as usize)))}
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_x_and_y(&self) -> [i8; 2] {
        [self.index as i8 % 8, 7 - (self.index as i8 / 8)]
    }
}
