#[cfg(test)]
mod tests;

pub mod coord;
mod pieces;
mod game;
mod moves;

use game::Game;

pub fn new_game() -> Game {
    Game::new()
}

