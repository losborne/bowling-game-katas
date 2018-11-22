pub struct Game {
    rolls : [u16; 21],
    current_roll : usize
}

impl Game {
    pub fn new() -> Game {
        Game { rolls: [0; 21], current_roll: 0 }
    }

    pub fn roll(&mut self, pins : u16) {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
    }

    pub fn score(&mut self) -> u16 {
        let mut score = 0;
        for i in 0..self.rolls.len() {
            score += self.rolls[i]
        }
        score
    }
}
