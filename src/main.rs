use foo::guessing_game::Game;

fn main() {
    println!("Guess the number! (between 1 and 100)");

    let game = Game::new(Game::random());
    game.run();
}
