extern crate prettytable;
use prettytable::{color, Attr, Cell, Row, Table};

use rand::Rng;

use fancy_print::{Animation, FancyPrinter};
use std::time::Duration;

macro_rules! clear_screen {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    };
}

pub struct Game {
    difficulty: Difficulty,

    secret_number: u32,
    attempts: u32,
    attempt: u32,

    printer: FancyPrinter,
}

enum Difficulty {
    None,
    Easy,
    Medium,
    Hard,
}

impl Game {
    pub fn new() -> Self {
        let printer = FancyPrinter::builder()
            .animation(Animation::Typing)
            .time_delay(Duration::from_millis(25))
            .build();

        Self {
            difficulty: Difficulty::None,

            secret_number: 0,
            attempts: 0,
            attempt: 0,

            printer,
        }
    }

    pub fn run(&mut self) {
        self.reset();
        self.menu_screen();
    }

    fn exit(&self) {
        std::process::exit(0);
    }

    fn reset(&mut self) {
        self.difficulty = Difficulty::None;

        self.secret_number = 0;
        self.attempts = 0;
        self.attempt = 0;
    }

    fn generate_secret_number(&mut self) {
        let mut rng = rand::thread_rng();

        match self.difficulty {
            Difficulty::Easy => self.secret_number = rng.gen_range(1..=1000),
            Difficulty::Medium => self.secret_number = rng.gen_range(1..=10000),
            Difficulty::Hard => self.secret_number = rng.gen_range(1..=1000000),
            _ => self.difficulty_screen(),
        }
    }

    fn play(&mut self) {
        // clear the screen
        clear_screen!();

        // show the difficulty menu
        self.difficulty_screen();

        // generate a secret number based on the difficulty
        self.generate_secret_number();

        // game loop
        self.game_loop();
    }

    fn game_loop(&mut self) {
        loop {
            // show the game screen
            self.game_screen();

            // get user attempt
            let mut attempt = String::new();
            std::io::stdin().read_line(&mut attempt).unwrap();

            // match the input
            match attempt.trim().parse::<u32>() {
                Ok(number) => {
                    self.attempt = number;
                    self.attempts += 1;

                    if number == self.secret_number {
                        println!(
                            "Congratulations! You guessed the number in {} attempts.",
                            self.attempts
                        );
                        break;
                    }
                }
                Err(_) => continue,
            }
        }
    }

    fn menu_screen(&mut self) {
        // clear the screen
        clear_screen!();

        // create a table
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("1. Play").with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("2. Exit").with_style(Attr::ForegroundColor(color::RED)),
        ]));

        // print the table
        table.printstd();

        // get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // match the input
        match input.trim() {
            "1" => self.play(),
            "2" => self.exit(),
            _ => self.menu_screen(),
        }
    }

    fn difficulty_screen(&mut self) {
        // clear the screen
        clear_screen!();

        // create a table
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("1. Easy").with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("2. Medium").with_style(Attr::ForegroundColor(color::YELLOW)),
            Cell::new("3. Hard").with_style(Attr::ForegroundColor(color::RED)),
        ]));

        // print the table
        table.printstd();

        // get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // match the input
        match input.trim() {
            "1" => self.difficulty = Difficulty::Easy,
            "2" => self.difficulty = Difficulty::Medium,
            "3" => self.difficulty = Difficulty::Hard,
            _ => self.difficulty_screen(),
        }
    }

    fn game_screen(&self) {
        // clear the screen
        clear_screen!();

        // create a table
        let mut table = Table::new();
        table.set_titles(Row::new(vec![Cell::new("Guess the number!")
            .with_style(Attr::Bold)
            .with_style(Attr::Blink)
            .with_style(match self.difficulty {
                Difficulty::Easy => Attr::ForegroundColor(color::GREEN),
                Difficulty::Medium => Attr::ForegroundColor(color::YELLOW),
                Difficulty::Hard => Attr::ForegroundColor(color::RED),
                _ => Attr::ForegroundColor(color::WHITE),
            })]));

        // display the number of attempts
        table.add_row(Row::new(vec![
            Cell::new("Attempts:").with_style(Attr::Bold),
            Cell::new(&self.attempts.to_string()).with_style(Attr::Bold),
        ]));

        // display the previous attempt
        table.add_row(Row::new(vec![
            Cell::new("Previous attempt:").with_style(Attr::Bold),
            Cell::new(&self.attempt.to_string()).with_style(Attr::Bold),
        ]));

        // print the table
        table.printstd();

        // show hint
        if self.attempts > 0 {
            if self.attempt < self.secret_number {
                self.printer.print("\nHigher!\n")
            } else {
                self.printer.print("\nLower!\n")
            }
        }
    }

    fn win_screen(&mut self)
}
