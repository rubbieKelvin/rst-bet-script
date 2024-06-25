use std::{
    io::{stdin, stdout, Write},
    num::ParseIntError,
    ops::Div,
};

enum GameTermination {
    Quit,
    PlayAgain(i32),
}

fn input(propmt: &str) -> Result<String, String> {
    print!("{}", propmt);
    stdout().flush().map_err(|e| e.to_string())?;

    let mut res = String::new();
    stdin().read_line(&mut res).map_err(|e| e.to_string())?;
    return Ok(res);
}

fn play(_username: &String, wallet: i32) -> Result<GameTermination, ParseIntError> {
    let stake_prompt = format!("How much do you want to stake? (max: ${}) ", wallet);
    let stake = input(&stake_prompt).unwrap().trim().parse::<i32>()?;

    let leverage_prompt: String = format!("How much leverage? (1-{})x ", wallet.div(stake));
    let leverage = input(&leverage_prompt).unwrap().trim().parse::<i32>()?;

    println!("Generating new random number...");

    let gains: i32 = stake * leverage;
    let number = 0;
    let user_number = input("Guess the random number: ")
        .unwrap()
        .trim()
        .parse::<i32>()?;
    let won = user_number == number;

    if won {
        println!(
            "You won ${}!\nYour new wallet balance: ${}.00",
            gains,
            wallet + gains
        );
    } else {
        println!(
            "You lost ${}\nYour new wallet balance: ${}.00",
            gains,
            wallet - gains
        )
    }

    let play_again = input("Do you want to play again? ")
        .unwrap()
        .trim()
        .to_owned();

    if play_again.len() > 0 && !(play_again == "n" || play_again == "no") {
        return Ok(GameTermination::PlayAgain({
            if won {
                gains
            } else {
                -gains
            }
        }));
    } else {
        return Ok(GameTermination::Quit);
    }
}

fn main() -> Result<(), String> {
    let mut wallet: i32 = 1000;
    let username = input("Hi, what's your name? ")?;

    while wallet != 0 {
        match play(&username, wallet) {
            Ok(state) => match state {
                GameTermination::PlayAgain(gains) => wallet += gains,
                GameTermination::Quit => {
                    break;
                }
            },
            Err(..) => {
                println!("You entered an incorrect number");
            }
        }
    }
    return Ok(());
}
