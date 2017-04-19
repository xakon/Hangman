extern crate rand;
extern crate ansi_term;

use std::process::Command;

use std::io;
use std::io::{BufReader, BufRead};
use std::fs::File;
use rand::{thread_rng, sample};

use ansi_term::Colour::{Red, Green, Yellow};

struct GameData {
    secret_line         : String,
    discovered_letters  : String,
    lives               : i32,
    status              : String
}

enum UserInputStatus {
    AlreadyDiscovered,
    LetterGuessed,
    LetterMissed,
}

impl GameData {
    fn new(secret: &str, lives: i32) -> GameData {
        GameData {
            lives: lives,
            secret_line: secret.to_string(),
            discovered_letters: String::new(),
            status: String::new(),
        }
    }

    fn format_masked_string(&self) -> String
    {
        let (input, mask) = (&self.secret_line, &self.discovered_letters);
        let mut result : String = String::new();

        for c in input.chars()
        {
            result.push(if c == ' ' {c}
                else if mask.contains(c) {c}
                else {'_'});
            result.push(' ');
        }

        result
    }

    fn check_user_guess(&self, user_guess: char) -> UserInputStatus
    {
        if self.discovered_letters.contains(user_guess)
        {
            return UserInputStatus::AlreadyDiscovered;
        }

        if !self.secret_line.contains(user_guess)
        {
            return UserInputStatus::LetterMissed;
        }

        UserInputStatus::LetterGuessed
    }

}

fn main()
{
    let random_line = get_random_line("input.txt").expect("Failed to read input data!");

    let mut gd : GameData = GameData::new(&random_line, 5);

    let mut secret_line_masked = gd.format_masked_string();

    loop
    {
        update_screen(&gd, &secret_line_masked);

        println!("Type your guess:");
        let user_guess = read_guess();

        if !validate_user_guess(user_guess)
        {
            let status = format!("It is not a letter!");
            gd.status = Yellow.paint(status).to_string();
            continue;
        }

        if !secret_line_masked.contains('_')
        {
            gd.status = Green.bold().paint("You won!").to_string();
            update_screen(&gd, &secret_line_masked);
            break;
        }

        if gd.lives == 0
        {
            gd.status = Red.bold().paint("You lost!").to_string();
            secret_line_masked = gd.format_masked_string();
            update_screen(&gd, &secret_line_masked);
            break;
        }

        let guess_lower = user_guess.unwrap().to_lowercase().next().unwrap();

        match gd.check_user_guess(guess_lower)
        {
            UserInputStatus::LetterGuessed =>
            {
                gd.discovered_letters.push(guess_lower);
                let status = format!("You discovered {}", guess_lower);
                gd.status = Green.paint(status).to_string();
                secret_line_masked = gd.format_masked_string();
            }

            UserInputStatus::LetterMissed =>
            {
                gd.discovered_letters.push(guess_lower);
                gd.lives = gd.lives - 1;

                if gd.lives > 0
                {
                    let status = format!("Unfortunately, no {}", guess_lower);
                    gd.status = Red.paint(status).to_string();
                }
            }

            UserInputStatus::AlreadyDiscovered =>
            {
                let status = format!("{} is already discovered!", guess_lower);
                gd.status = Yellow.paint(status).to_string();
            }
        }
    }
}

fn read_guess() -> Option<char>
{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess.trim().chars().nth(0)
}

fn get_random_line(filename: &str) -> io::Result<String>
{
    let f = try!(File::open(filename));
    let file = BufReader::new(&f);
    let mut rng = thread_rng();
    let sample = sample(&mut rng, file.lines(), 1).pop().unwrap();
    let secret_line = sample.unwrap().to_lowercase();
    Ok(secret_line)
}

fn validate_user_guess(user_guess: Option<char>) -> bool
{
    match user_guess
    {
        Some(guess)	=> guess.is_alphabetic(),
        None		=> false,
    }
}

fn update_screen(gd: &GameData, secret_line: &str)
{
    clear();
    println!("HANGMAN: CAN YOU GUESS THE SENTENCE?");
    println!("Lives: {}. Discovered letters: {}", gd.lives, gd.discovered_letters);
    print_hangman(gd.lives);
    println!("{}", secret_line);
    println!("{}", gd.status);
}

fn print_hangman(lives: i32)
{
    match lives
    {
        0 =>
        {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         XO ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|            ");
            println!("|            ");
        }

        1 =>
        {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        2 =>
        {
            println!(" _________   ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        3 =>
        {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");

        }

        4 =>
        {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
        }

        _ =>
        {
            println!("             ");
            println!("             ");
            println!("             ");
            println!("             ");
            println!("          O  ");
            println!("         /|\\ ");
            println!("         / \\ ");
        }
    }
}

fn clear()
{
  let output = Command::new("clear").output().unwrap_or_else(|e|{
    panic!("failed to execute process: {}", e)
  });
  println!("{}", String::from_utf8_lossy(&output.stdout));
}

