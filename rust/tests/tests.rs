extern crate bowling_game_rust;
use bowling_game_rust::Game;

#[test]
fn test_gutter_game() {
    let mut g = Game::new();
    for i in 0..20 {
        g.roll(0);
    }
}
