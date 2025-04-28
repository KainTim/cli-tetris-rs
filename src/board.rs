use crate::color::BoardColor;
use crate::color::BoardColor::*;
use crate::{clear_screen, print, print_str, println_str, Block};
use crossterm::style::Color::White;
use crossterm::style::{Color, SetForegroundColor};
use crossterm::QueueableCommand;
use std::io::{stdout, Write};

pub struct Board{
    pub(crate) rows:i32,
    pub(crate) columns:i32,
    board: [[Block;10]; 20],
    wall_color: Color,
    pub(crate) background_color: BoardColor,
}
impl Board {
    pub fn new() -> Board {
        Board {
            rows: 20,
            columns: 10,
            board: core::array::from_fn(|_|
                core::array::from_fn(|_|
                    Block::new('#', Black))),
            wall_color: White,
            background_color: Black,
        }
    }
    pub fn print_board(&self) {
        clear_screen();
        stdout().queue(SetForegroundColor(self.wall_color)).unwrap();
        self.board.iter().enumerate().for_each(|(i,_)|
            print(format!("{}", match i {
                0=> "╔",
                _=>"═"
            }
        )));
        println_str("═╗");
        stdout().flush().unwrap();
        for row in self.board.iter() {
            print_str("║");
            for cell in row.iter() {
                stdout().queue(SetForegroundColor(BoardColor::convert_to_crossterm_color(&cell.color))).unwrap();
                print(format!("{cell} "));
            }
            stdout().queue(SetForegroundColor(self.wall_color)).unwrap();
            println_str("║");
        }
        stdout().queue(SetForegroundColor(self.wall_color)).unwrap();
        self.board.iter().enumerate().for_each(|(i,_)|
            print(format!("{}", match i {
                0=> "╚",
                _=>"═"
            }
            )));
        print_str("═╝");
        stdout().flush().unwrap();
    }
    pub fn set_block(&mut self, row:i32, column:i32, color:BoardColor){
        if !(row>=0 && row <self.rows) { return; }
        if !(column>=0 && column <self.columns) { return; }
        assert!(row>=0 && row <self.rows, "Row isn't in bounds");
        assert!(column>=0 && column <self.columns, "Column isn't in bounds");
        let column:usize = column as usize;
        let row:usize = row as usize;
        self.board[row][column] = Block::new('#',color)
    }

}