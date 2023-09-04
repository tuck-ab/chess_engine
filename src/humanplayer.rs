use std::io::{self, Write};
use crate::{Player, chess};

pub struct HumanPlayer {}

impl Player for HumanPlayer {
    fn make_move(&self, game: &mut chess::Game) {
        let mut piece_selection: Option<usize> = None;
        let mut move_selection: Option<usize> = None;

        let pieces = game.get_pieces();
        let mut moves= Vec::new();

        while piece_selection.is_none() || move_selection.is_none() {
            println!("{:?}\n", game);
            if piece_selection.is_none() {
                println!("Pieces:");

                for (i, p) in pieces.iter().enumerate() {
                    println!("{}: {:?}", i+1, p);
                }

                let mut input_buffer: String = String::new();
                print!("\nEnter Piece Selection: ");

                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut input_buffer).expect("Error reading in piece");
                let _ = input_buffer.trim().parse::<usize>().map(
                    |i| if i <= pieces.len() && i > 0 {piece_selection = Some(i-1)}
                );
            } else {
                moves = game.get_moves_for_unchecked_piece(pieces[piece_selection.unwrap()]);

                println!("Piece: {:?}\n", pieces[piece_selection.unwrap()]);
                println!("Moves:");

                for (i, m) in moves.iter().enumerate() {
                    println!("{}: {:?}", i+1, m)
                }

                println!("\n0: BACK");

                let mut input_buffer: String = String::new();
                print!("\nEnter Move Selection: ");

                let _ = io::stdout().flush();
                let _ = io::stdin().read_line(&mut input_buffer).expect("Error reading in piece");
                let _ = input_buffer.trim().parse::<usize>().map(
                    |i| if i <= moves.len() && i > 0 {move_selection = Some(i-1)}
                    else if i == 0 {piece_selection = None}
                );
            }
        }

        game.apply_unchecked_move(moves[move_selection.unwrap()], true);
    }
}