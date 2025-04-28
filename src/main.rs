mod block;
mod board;
mod color;
mod game_logic;
mod input;
mod piece;
mod rotation;

use crate::game_logic::GameLogic;
use crate::input::handle_input;
use block::Block;
use board::Board;
use crossterm::{execute, QueueableCommand};
use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::event::{poll, DisableMouseCapture};
use crossterm::terminal::ClearType::All;
use crossterm::terminal::{
    Clear, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use signal_hook::consts::{SIGINT, SIGKILL};
use signal_hook::iterator::Signals;
use std::io::{Write, stdout};
use std::process::exit;
use std::thread;
use std::time::{Duration, Instant};
use crate::color::BoardColor::Red;
use crate::piece::Piece;

fn main() {
    init();
    let mut logic: GameLogic = GameLogic::new(Board::new(), Piece::new([(0,0),(1,0),(2,0),(3,0)],"I-Piece",Red));
    let mut last_moved_down_time: Instant = Instant::now();
    loop {
        if poll(Duration::from_millis(20)).unwrap() {
            handle_input(&mut logic);
            if last_moved_down_time.elapsed().as_millis() > 1000 {
                logic.move_down();
                last_moved_down_time = Instant::now();
            }
            logic.print();
            continue;
        }
        if last_moved_down_time.elapsed().as_millis() > 800 {
            logic.move_down();
            last_moved_down_time = Instant::now();
        }
        logic.print();
        stdout().flush().unwrap()
    }
}

fn clear_screen() {
    stdout().queue(Clear(All)).unwrap();
    stdout().queue(MoveTo(0, 0)).unwrap();
}
fn print(string: String) {
    write!(stdout(), "{}", string).expect("Error while writing to stdout");
}
fn println(string: String) {
    write!(stdout(), "{}", string).expect("Error while writing to stdout");
    stdout().queue(MoveToNextLine(1)).unwrap();
}
fn print_str(string: &str) {
    write!(stdout(), "{}", string).expect("Error while writing to stdout");
}
fn println_str(string: &str) {
    write!(stdout(), "{}", string).expect("Error while writing to stdout");
    stdout().queue(MoveToNextLine(1)).unwrap();
}
fn shutdown() {
    disable_raw_mode().unwrap();
    stdout().queue(LeaveAlternateScreen).unwrap();
    println_str("\nExiting...");
    exit(0)
}
fn init() {
    stdout().queue(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
}
