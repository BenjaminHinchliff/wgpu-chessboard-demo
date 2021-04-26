use crate::piece::{Color, Piece, Type};

pub type Board = [[Option<Piece>; 8]; 8];

pub const fn default_board() -> Board {
    [
        [
            Some(Piece::new(Color::Black, Type::Rook)),
            Some(Piece::new(Color::Black, Type::Knight)),
            Some(Piece::new(Color::Black, Type::Bishop)),
            Some(Piece::new(Color::Black, Type::Queen)),
            Some(Piece::new(Color::Black, Type::King)),
            Some(Piece::new(Color::Black, Type::Bishop)),
            Some(Piece::new(Color::Black, Type::Knight)),
            Some(Piece::new(Color::Black, Type::Rook)),
        ],
        [Some(Piece::new(Color::Black, Type::Pawn)); 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [Some(Piece::new(Color::White, Type::Pawn)); 8],
        [
            Some(Piece::new(Color::White, Type::Rook)),
            Some(Piece::new(Color::White, Type::Knight)),
            Some(Piece::new(Color::White, Type::Bishop)),
            Some(Piece::new(Color::White, Type::Queen)),
            Some(Piece::new(Color::White, Type::King)),
            Some(Piece::new(Color::White, Type::Bishop)),
            Some(Piece::new(Color::White, Type::Knight)),
            Some(Piece::new(Color::White, Type::Rook)),
        ],
    ]
}
