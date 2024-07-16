extern crate prettytable;
use prettytable::{color, Attr, Cell, Row, Table};

macro_rules! clear_screen {
    () => {
        print!("\x1B[2J\x1B[1;1H");
    };
}

enum AppState {
    MainMenu,
    Playing,
    GameOver,
}

pub struct Game {
    state: AppState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: AppState::MainMenu,
        }
    }

    pub fn start(&mut self) {
        self.main_menu();
    }

    fn main_menu(&mut self) {
        // update the state
        self.state = AppState::MainMenu;

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
            "1" => println!("Play"),
            "2" => std::process::exit(0),
            _ => self.main_menu(),
        }
    }
}
