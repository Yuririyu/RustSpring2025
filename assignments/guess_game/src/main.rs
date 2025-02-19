fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_number = 3; // Hard-coded secret number
    let mut guess = 2; // Simulated guess+
    let mut attempts = 0;

    loop {
        attempts += 1;

        match check_guess(guess, secret_number) {
            0 => {
                println!("Correct guess!");
                break;
            }
            1 => {
                println!("Too high!");
                guess -= 1;
            }
            -1 =>{
                println!("Too low!");
                guess += 1;
            }_ => {}
        }
    }

    println!("It took {} guesses.", attempts);
}