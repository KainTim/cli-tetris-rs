use std::ops::Deref;
use crate::board::Board;
use crate::color::BoardColor;
use crate::color::BoardColor::{Green, Red};
use crate::piece::Piece;

pub struct GameLogic{
    board: Board,
    x_coordinate:i32,
    y_coordinate:i32,
    active_piece:Option<Piece>
}

impl GameLogic {
}

impl GameLogic{
    pub fn new(board: Board, piece: Piece ) -> GameLogic{
        let mut logic = GameLogic{
            board,
            x_coordinate: 0,
            y_coordinate: 0,
            active_piece:None
        };
        logic.spawn_new_piece(piece.clone());
        logic.active_piece=Option::from(piece);
        logic.board.set_block(0,0,Red);
        logic
    }
    pub fn print(&self){
        self.board.print_board();
    }
    pub fn move_horizontal(&mut self, amount:i32){
        if self.active_piece.is_none(){
            return;
        }
        let mut piece = self.active_piece.take().unwrap();
        let board_color = self.board.background_color;
        Self::print_piece(&mut self.board, &piece, board_color);
        self.x_coordinate+=amount;
        self.clamp_coordinates(&piece);
        
        piece.move_to(self.x_coordinate,self.y_coordinate, self.board.columns,self.board.rows);

        Self::print_piece(&mut self.board, &piece, piece.color);
        self.active_piece = Option::from(piece);
    }
    
    fn print_piece(board: &mut Board, piece: &Piece, color:BoardColor){
        for position in piece.get_block_positions() {
            board.set_block(position.0,position.1,color);
        }
    }

    pub fn move_down(&mut self) {
        if self.active_piece.is_none(){
            return;
        }
        if self.y_coordinate>=self.board.rows-1 { 
            self.spawn_new_piece(Piece::new([(0,0),(0,1),(1,0),(2,0)],"L-Piece",Red));
            return;
        }
        
        let mut piece = self.active_piece.take().unwrap();
        let board_color = self.board.background_color;
        
        Self::print_piece(&mut self.board, &piece, board_color);
        
        if self.y_coordinate==self.board.rows-1 {
            self.y_coordinate=0;
            self.x_coordinate=0;
            return;
        }
        
        self.y_coordinate+=1;
        self.clamp_coordinates(&piece);
        
        piece.move_to(self.x_coordinate,self.y_coordinate, self.board.columns,self.board.rows);

        Self::print_piece(&mut self.board, &piece, piece.color);

        self.active_piece = Option::from(piece);
    }

    fn clamp_coordinates(&mut self, piece: &Piece){
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

    pub(crate) fn spawn_new_piece(&mut self, mut piece: Piece) {
        self.y_coordinate = 0;
        self.x_coordinate = 0;
        piece.move_to(self.x_coordinate,self.y_coordinate, self.board.columns,self.board.rows);
        for position in piece.get_block_positions() {
            self.board.set_block(position.0,position.1,Red);
        }
        self.active_piece= Option::from(piece);
    }
}