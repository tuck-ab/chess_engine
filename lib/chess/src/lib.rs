#[cfg(test)]
mod tests;

pub mod coord;
mod pieces;
mod game;
mod moves;

pub use game::Game;
pub use pieces::Side;
