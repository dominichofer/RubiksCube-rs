#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Green,  // Left face
    Blue,   // Right face
    White,  // Up face
    Yellow, // Down face
    Red,    // Front face
    Orange, // Back face
}

pub fn as_face_char(color: Color) -> char {
    match color {
        Color::Green => 'L',
        Color::Blue => 'R',
        Color::White => 'U',
        Color::Yellow => 'D',
        Color::Red => 'F',
        Color::Orange => 'B',
    }
}