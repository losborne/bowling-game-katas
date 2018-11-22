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
        let mut frame_index = 0;
        for frame in 0..10 {
            score += self.rolls[frame_index] + self.rolls[frame_index+1];
            if self.is_spare(frame_index) {
                score += self.rolls[frame_index+2]
            }
            frame_index += 2;
        }
        score
    }
    
    fn is_spare(&mut self, frame_index : usize) -> bool {
        self.rolls[frame_index] + self.rolls[frame_index+1] == 10 
    }
}
