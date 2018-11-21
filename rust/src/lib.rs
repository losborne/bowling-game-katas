pub struct Game {
    score : u16
}

impl Game {
    pub fn new() -> Game {
        Game { score: 0 }
    }

    pub fn roll(&mut self, pins : u16) {
        self.score += pins;
    }

    pub fn score(&mut self) -> u16 {
        self.score
    }
}
