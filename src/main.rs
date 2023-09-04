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

    let white: Box<dyn Player> = decode_player_arg(cli.white);
    let black: Box<dyn Player> = decode_player_arg(cli.black);

    while game.get_winner().is_none() {
        match game.get_side_to_play() {
            chess::Side::White => white.make_move(&mut game),
            chess::Side::Black => black.make_move(&mut game)
        }
    }

    println!("{:?}", game)

}

fn decode_player_arg(arg: PlayerMode) -> Box<dyn Player> {
    match arg {
        PlayerMode::Human => Box::new(HumanPlayer {})
    }
}