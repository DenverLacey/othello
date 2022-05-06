mod board;
mod othello;

use othello::Othello;

fn main() -> std::io::Result<()> {
    let mut game = Othello::new();
    game.play()
}
