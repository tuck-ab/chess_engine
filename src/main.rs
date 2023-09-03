use chess::new_game;

fn main() {
    let mut game = new_game();

    // Because we get the moves from the game, it is trusted and so can be
    // done unchecked
    game.apply_unchecked_move(game.get_valid_moves()[0]);

    println!("{:?}", game);

    for (i, move_) in game.get_valid_moves().iter().enumerate() {
        println!("{}: {:?}", i+1, move_);
    }
}
