use clap::{Parser, ValueEnum};
use chess;

#[derive(Parser)]
struct Cli {
    /// Who will play as White
    #[arg(short, long, value_enum)]
    white: PlayerMode,
    /// Who will play as Black
    #[arg(short, long, value_enum)]
    black: PlayerMode,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum PlayerMode {
    Human
}

trait Player {
    fn make_move(&self, game: &mut chess::Game);
}

mod humanplayer;
use humanplayer::HumanPlayer;

fn main() {
    let cli = Cli::parse();

    let mut game = chess::Game::new();

    println!("{:?}", game);

    for (i, move_) in game.get_valid_moves().iter().enumerate() {
        println!("{}: {:?}", i+1, move_);
    }
}
