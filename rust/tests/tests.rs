extern crate bowling_game_rust;
use bowling_game_rust::Game;


#[test]
fn test_gutter_game() {
    let mut game = Game::new();
    roll_many(20, 0, &mut game);
    assert_eq!(0, game.score());
}

#[test]
fn test_all_ones() {
    let mut game = Game::new();
    roll_many(20, 1, &mut game);
    assert_eq!(20, game.score());
}

fn roll_many(n : u16, pins : u16, game : &mut Game) {
    for i in 0..n {
        game.roll(pins);
    }
}
