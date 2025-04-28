use crate::board::Board;
use crate::color::BoardColor;
use crate::color::BoardColor::{Green, Red};

pub struct GameLogic{
    board: Board,
    x_coordinate:i32,
    y_coordinate:i32,
}
impl GameLogic{
    pub fn new(board: Board) -> GameLogic{
        let mut logic = GameLogic{
            board,
            x_coordinate: 0,
            y_coordinate: 0,
        };
        logic.board.set_block(0,0,Red);
        logic
    }
    pub fn print(&self){
        self.board.print_board();
    }
    pub fn move_horizontal(&mut self, amount:i32){
        self.board.set_block(self.y_coordinate,self.x_coordinate,self.board.background_color.clone());
        self.x_coordinate+=amount;
        self.clamp_coordinates();
        self.board.set_block(self.y_coordinate,self.x_coordinate,Red);
    }
    fn clamp_coordinates(&mut self){
        if self.x_coordinate<0 {
            self.x_coordinate = 0
        }
        if self.x_coordinate>=self.board.columns {
            self.x_coordinate = self.board.columns-1;
        }
        if self.y_coordinate<0 {
            self.y_coordinate = 0
        }
        if self.y_coordinate>=self.board.rows {
            self.y_coordinate =self.board.rows-1;
        }
    }

    pub fn move_down(&mut self) {

        if self.y_coordinate==self.board.rows-1 {
            self.y_coordinate=0;
            self.x_coordinate=0;
            return;
        }

        self.board.set_block(self.y_coordinate,self.x_coordinate,self.board.background_color.clone());
        self.y_coordinate+=1;
        self.clamp_coordinates();
        self.board.set_block(self.y_coordinate,self.x_coordinate,Red);

    }
}