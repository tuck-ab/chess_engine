use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::{Player, chess};

pub struct RandomPlayer {}

impl Player for RandomPlayer {
    fn make_move(&self, game: &mut chess::Game) {
        game.apply_unchecked_move(*game.get_valid_moves().choose(&mut thread_rng()).expect("Random bot couldn't choose move as there is no valid move"), true);
    }
}