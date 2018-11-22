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
        let mut i = 0;
        for frame in 0..10 {
            score += self.rolls[i] + self.rolls[i+1];
            if self.rolls[i] + self.rolls[i+1] == 10 {
                // spare
                score += self.rolls[i+2]
            }
            i += 2;
        }
        score
    }
}
