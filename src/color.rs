use crossterm::style::Color;
#[derive(Copy, Clone)]
pub enum BoardColor {
    Red,
    Green,
    Blue,
    Yellow,
    LightBlue,
    Purple,
    Black,
}

impl BoardColor{

    pub fn convert_to_crossterm_color(p0: &BoardColor) -> Color {
        match p0 {
            BoardColor::Red => Color::Red,
            BoardColor::Green => Color::Green,
            BoardColor::Blue => Color::Blue,
            BoardColor::Yellow => Color::Yellow,
            BoardColor::LightBlue => Color::Blue,
            BoardColor::Purple => Color::DarkMagenta,
            BoardColor::Black => Color::Black,
        }
    }
}