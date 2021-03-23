use chess::{Board, Game};

fn main() {
    let game = Game::new();
    assert_eq!(game.current_position(), Board::default());
    println!("{:?}", game.current_position().get_hash());
}
