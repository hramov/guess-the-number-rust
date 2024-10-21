use rand::prelude::*;

trait GuessTheNumberGame {
    fn new() -> GuessTheNumberGameImpl;
    fn start(&mut self, max_number:i32, max_attempts: i32);
    fn guess(&mut self, number: i32) -> Direction;
    fn stop(&mut self);
}

struct GuessTheNumberGameImpl {
    secret_number: i32,
    attempts: i32,
    max_attempts: i32,
    is_game_over: bool,
}

enum Direction {
    Greater,
    Lower,
    Exact,
    Unknown,
}

impl GuessTheNumberGame for GuessTheNumberGameImpl {
    fn new() -> GuessTheNumberGameImpl {
        GuessTheNumberGameImpl {
            secret_number: 0,
            attempts: 0,
            max_attempts: 10,
            is_game_over: false,
        }
    }

    fn start(&mut self, max_number:i32, max_attempts:i32) { 
        self.secret_number = rand::thread_rng().gen_range(1..=max_number);
        self.attempts = 0;
        self.max_attempts = max_attempts;
        self.is_game_over = false;
    }

    fn guess(&mut self, number: i32) -> Direction  {
        self.attempts += 1;

        if self.attempts >= self.max_attempts {
            return Direction::Unknown;
        }
        if number == self.secret_number {
            return Direction::Exact;
        }
        if number > self.secret_number {
            return Direction::Greater;
        }
        return Direction::Lower;
    }

    fn stop(&mut self) {
        self.is_game_over = true;
    }
}

fn get_user_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut game = GuessTheNumberGameImpl::new();

    game.start(100, 10);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");
        let input = get_user_string()
            .parse::<i32>()
            .expect("Please type a number!");

        match game.guess(input) {
            Direction::Greater => println!("Too big!"),
            Direction::Lower => println!("Too small!"),
            Direction::Exact => {
                println!("You guessed it!");
                game.stop();
                break;
            },
            Direction::Unknown => {
                println!("You have run out of guesses!");
                game.stop();
                break;
            }
        }
    }

    println!("The secret number was: {}", game.secret_number);
    println!("You guessed {} times", game.attempts);
}
