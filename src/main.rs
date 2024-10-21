mod game;
mod ioutil;

fn main() {
    println!("Welcome to the Guess the number game!");
    println!("Please, enter the maximum number: ");
    let max_number = ioutil::get_user_number();

    println!("Please, enter the maximum number of attempts: ");
    let max_attempts = ioutil::get_user_number();

    let mut game = game::new_game();
    game.start(max_number, max_attempts);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");
        let input = ioutil::get_user_number();

        let guess = game.guess(input);

        match guess {
            1 => println!("Too big!"),
            -1 => println!("Too small!"),
            0 => {
                println!("You guessed it!");
                game.stop();
                break;
            },
            2 => {
                println!("You have run out of guesses!");
                game.stop();
                break;
            },
            _ => {
                println!("I have expectated 1, -1, 0 or 2, but I got: {}", guess);
                game.stop();
                break;
            }
        }
    }

    let (number, attempts) = game.stat();

    println!("===========");
    println!("The secret number was: {}", number);
    println!("You guessed {} times", attempts);
}
