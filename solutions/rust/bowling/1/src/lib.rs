#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    current_frame: u8,
    is_complete: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: Vec::with_capacity(21),
            current_frame: 1,
            is_complete: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        let len = self.rolls.len();
        // Dynamically find the start of the 10th frame
        let mut frame = 0;
        let mut idx = 0;
        while frame < 9 && idx < self.rolls.len() {
            if self.rolls[idx] == 10 {
                idx += 1;
            } else {
                idx += 2;
            }
            frame += 1;
        }
        let tenth_start = idx;
        // 10th frame logic
        if self.current_frame == 10 {
            let rolls_in_10th = len.saturating_sub(tenth_start);
            match rolls_in_10th {
                0 => { /* first roll in 10th, always allowed if <= 10 */ }
                1 => {
                    let first = self.rolls[tenth_start];
                    if first < 10 && first + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }
                2 => {
                    let first = self.rolls[tenth_start];
                    let second = self.rolls[tenth_start + 1];
                    if first == 10 {
                        // Strike in first roll, second roll can be anything, but if second is not strike, sum with third must be <= 10
                        if second < 10 && second + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                    } else if first + second == 10 {
                        // Spare in first two rolls, third roll can be anything <= 10
                        // already checked pins <= 10 above
                    } else {
                        // No strike or spare, should not allow a third roll
                        return Err(Error::GameComplete);
                    }
                }
                _ => {
                    // Already had 3 rolls in 10th frame
                    return Err(Error::GameComplete);
                }
            }
            self.rolls.push(pins);
            // Mark complete if 10th frame is done
            let rolls_in_10th = self.rolls.len().saturating_sub(tenth_start);
            if rolls_in_10th == 2 {
                let first = self.rolls[tenth_start];
                let second = self.rolls[tenth_start + 1];
                if first < 10 && first + second < 10 {
                    self.is_complete = true;
                }
            } else if rolls_in_10th == 3 {
                self.is_complete = true;
            }
            return Ok(());
        }
        // Frames 1-9 logic
        if self.current_frame < 10 {
            if len > 0 && (len % 2 == 1) && self.rolls[len - 1] != 10 {
                // Second roll of frame, check sum
                if self.rolls[len - 1] + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
            self.rolls.push(pins);
            if pins == 10 && (len % 2 == 0) {
                // Strike, advance frame (do not pad with 0)
                self.current_frame += 1;
            } else if len % 2 == 1 {
                // Second roll of frame
                self.current_frame += 1;
            }
            return Ok(());
        }
        Ok(())
    }
    pub fn score(&self) -> Option<u16> {
        if !self.is_complete {
            return None;
        }
        let mut score = 0;
        let mut roll_idx = 0;
        let rolls = &self.rolls;
        for frame in 0..10 {
            if roll_idx >= rolls.len() {
                return None;
            }
            if rolls[roll_idx] == 10 {
                // Strike
                score += 10 + rolls.get(roll_idx + 1).unwrap_or(&0) + rolls.get(roll_idx + 2).unwrap_or(&0);
                roll_idx += 1;
            } else if rolls.get(roll_idx + 1).unwrap_or(&0) + rolls[roll_idx] == 10 {
                // Spare
                score += 10 + rolls.get(roll_idx + 2).unwrap_or(&0);
                roll_idx += 2;
            } else {
                // Open frame
                score += rolls[roll_idx] + rolls.get(roll_idx + 1).unwrap_or(&0);
                roll_idx += 2;
            }
        }
        Some(score)
    }
}