use rand::prelude::*;

pub trait GuessTheNumberGame {
    fn start(&mut self, max_number:i32, max_attempts: i32);
    fn guess(&mut self, number: i32) -> i8;
    fn stop(&mut self);
    fn stat(&self) -> (i32, i32);
}

struct GuessTheNumberGameImpl {
    secret_number: i32,
    attempts: i32,
    max_attempts: i32,
    is_game_over: bool,
}

pub fn new_game() -> Box<dyn GuessTheNumberGame> {
    Box::new(GuessTheNumberGameImpl {
        secret_number: 0,
        attempts: 0,
        max_attempts: 0,
        is_game_over: false,
    })
}

impl GuessTheNumberGame for GuessTheNumberGameImpl {
    fn start(&mut self, max_number:i32, max_attempts:i32) {
        let max_number = if max_number < 1 {
            10
        } else {
            max_number
        };

        let max_attempts = if max_attempts < 1 {
            5
        } else {
            max_attempts
        };

        self.secret_number = rand::thread_rng().gen_range(1..=max_number);
        self.attempts = 0;
        self.max_attempts = max_attempts;
        self.is_game_over = false;
    }

    fn guess(&mut self, number: i32) -> i8  {
        self.attempts += 1;

        if self.attempts > self.max_attempts {
            return 2;
        }
        if number == self.secret_number {
            return 0;
        }
        if number > self.secret_number {
            return 1;
        }
        return -1;
    }

    fn stop(&mut self) {
        self.is_game_over = true;
    }
    
    fn stat(&self) -> (i32, i32) {
        (self.secret_number, self.attempts)
    }
}