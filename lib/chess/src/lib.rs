#[cfg(test)]
mod tests;

pub mod coord;
pub mod pieces;
pub mod game;
pub mod moves;

use game::Game;

pub fn new_game() -> Game {
    Game::new()
}

