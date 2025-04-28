use std::fmt::Formatter;
use crate::color::BoardColor;

pub struct Block{
    character:char,
    pub(crate) color: BoardColor
}
impl Block{
    pub fn new(character:char, color: BoardColor) ->Block{
        Block{
            character,
            color
        }
    }
}
impl std::fmt::Display for Block{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",self.character)
    }
}