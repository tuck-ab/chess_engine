use crate::{Player, chess};

pub struct HumanPlayer {}

impl Player for HumanPlayer {
    fn make_move(&self, game: &mut chess::Game) {
        println!("{:?}", game);
    }
}