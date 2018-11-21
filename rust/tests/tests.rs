extern crate bowling_game_rust;
use bowling_game_rust::Game;

#[test]
fn test_gutter_game() {
    let mut g = Game::new();
    let mut n = 20;
    let mut pins = 0;
    for i in 0..n {
        g.roll(pins);
    }
    assert_eq!(0, g.score());
}

#[test]
fn test_all_ones() {
    let mut g = Game::new();
    for i in 0..20 {
        g.roll(1);
    }
    assert_eq!(20, g.score());
}
