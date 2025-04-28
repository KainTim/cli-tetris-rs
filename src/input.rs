use crate::game_logic::GameLogic;
use crate::shutdown;
use crossterm::event::KeyCode::Char;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

pub fn handle_input(logic: &mut GameLogic) {
    match read().unwrap() {
        Event::Key(event) => {
            match event {
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: Char('q'),
                    ..
                } => shutdown(),
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Left,
                    ..
                } => handle_left_input(logic),
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Right,
                    ..
                } => handle_right_input(logic),
                KeyEvent {
                    modifiers: KeyModifiers::NONE,
                    code: KeyCode::Down,
                    ..
                } => handle_down_input(logic),
                _=> (),
            }
        }
        _ => (),
    };
}

fn handle_right_input(logic: &mut GameLogic) {
    logic.move_horizontal(1);
}

fn handle_left_input(logic: &mut GameLogic) {
    logic.move_horizontal(-1);
}
fn handle_down_input(logic: &mut GameLogic) {
    logic.move_down();
}