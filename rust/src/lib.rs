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
            if self.is_strike(frame_index) {
                score += 10 + self.strike_bonus(frame_index);
                frame_index += 1;
            } else if self.is_spare(frame_index) {
                score += 10 + self.spare_bonus(frame_index);
                frame_index += 2;
            } else {
                score += self.sum_of_balls_in_frame(frame_index);
                frame_index += 2;
            }
        }
        score
    }

    fn sum_of_balls_in_frame(&mut self, frame_index : usize) -> u16 { 
        self.rolls[frame_index] + self.rolls[frame_index+1]
    }

    
    fn is_spare(&mut self, frame_index : usize) -> bool {
        self.rolls[frame_index] + self.rolls[frame_index+1] == 10 
    }

    fn is_strike(&mut self, frame_index : usize) -> bool {
        self.rolls[frame_index] == 10 
    }

    fn spare_bonus(&mut self, frame_index : usize) -> u16 {
        self.rolls[frame_index+2]
    }

    fn strike_bonus(&mut self, frame_index : usize) -> u16 {
        self.rolls[frame_index+1] + self.rolls[frame_index+2]
    }
}
