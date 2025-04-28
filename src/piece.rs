use crossterm::style::Color;
use crate::color::BoardColor;
use crate::rotation::Rotation;
use crate::rotation::Rotation::*;

#[derive(Clone)]
pub struct Piece{
    name:String,
    block_position_offsets:[(i32,i32);4],
    x_coordinate:i32,
    y_coordinate:i32,
    rotation: Rotation,
    pub(crate) color: BoardColor
}
impl Piece{
    pub fn new(block_position_offsets:[(i32,i32);4], name: &str, color: BoardColor)->Piece{
        Piece{
            name: name.to_string(),
            block_position_offsets,
            x_coordinate: 0,
            y_coordinate: 0,
            rotation:Up,
            color,
        }
    }
    pub fn move_to(&mut self, x_coordinate:i32, y_coordinate:i32, columns:i32, rows:i32){
        self.x_coordinate=x_coordinate;
        self.y_coordinate=y_coordinate;
    }
    pub fn get_block_positions(&self) -> [(i32,i32);4]{
        match self.rotation {
            Up => self.block_position_offsets.map(|v| (v.0+self.y_coordinate,v.1+self.x_coordinate)),
            _ => self.block_position_offsets
        }
    }
}