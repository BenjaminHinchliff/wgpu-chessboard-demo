#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    type_: Type,
    color: Color,
}
