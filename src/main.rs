use game::Game;
mod ball;
mod block;
mod game;
mod player;

#[macroquad::main("breakout")]
async fn main() {
    let mut game = Game::default();
    game.start().await;
}
