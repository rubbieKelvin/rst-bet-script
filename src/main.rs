use std::{
    io::{stdin, stdout, Write},
    num::ParseIntError,
};

use rand::{thread_rng, Rng};

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

fn play(wallet: i32) -> Result<GameTermination, ParseIntError> {
    let stake_prompt = format!("How much do you want to stake? (max: ${}) ", wallet);
    let stake = input(&stake_prompt).unwrap().trim().parse::<i32>()?;

    if stake > wallet {
        println!("You dont have up to {}", stake);
        return Ok(GameTermination::PlayAgain(0));
    } else if stake == 0 {
        println!("Stake should be greater than 0");
        return Ok(GameTermination::PlayAgain(0));
    }

    println!("Generating new random number (0 - 5)...");

    let gain: i32;

    let number = thread_rng().gen_range(0..5);
    let user_number = input("Guess the random number: ")
        .unwrap()
        .trim()
        .parse::<i32>()?;
    let won = user_number == number;

    if won {
        gain = stake;
        println!(
            "You won ${}!\nYour new wallet balance: ${}.00",
            stake,
            wallet + gain
        );
    } else {
        gain = -stake;

        println!(
            "You lost ${}\nThe random number was: {}\nYour new wallet balance: ${}.00",
            stake,
            number,
            wallet + gain
        )
    }

    if (wallet + gain) <= 0 {
        println!("You exausted your stash");
        return Ok(GameTermination::Quit);
    }

    let play_again = input("Do you want to play again? ")
        .unwrap()
        .trim()
        .to_owned();

    if play_again.len() > 0 && !(play_again == "n" || play_again == "no") {
        return Ok(GameTermination::PlayAgain({
            if won {
                stake
            } else {
                -stake
            }
        }));
    } else {
        return Ok(GameTermination::Quit);
    }
}

fn main() -> Result<(), String> {
    let mut wallet: i32 = 1000;

    while wallet > 0 {
        match play(wallet) {
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
