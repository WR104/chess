use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

pub const WHITE: Color = Color::White;
pub const BLACK: Color = Color::Black;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Position {
    row: i32,
    col: i32,
}

impl Position {
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    #[inline]
    pub fn get_row(&self) -> i32 {
        self.row
    }

    #[inline]
    pub fn get_col(&self) -> i32 {
        self.col
    }


    /// Get the position directly left of this position.
    /// 
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_left(&self) -> Self {
        Self::new(self.row, self.col - 1)
    }

        /// Get the position directly right of this position.
    /// 
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_right(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Piece {
    King(Color, Position),
    Queen(Color, Position),
    Rook(Color, Position),
    Bishop(Color, Position),
    Knight(Color, Position),
    Pawn(Color, Position),
}

impl Piece {
    #[inline]
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::King(_, _) => "king",
            Self::Queen(_, _) => "queen",
            Self::Rook(_, _) => "rook",
            Self::Bishop(_, _) => "bishop",
            Self::Knight(_, _) => "knight",
            Self::Pawn(_, _) => "pawn",
        }
    }

    #[inline]
    pub fn get_pos(&self) -> Position {
        match self {
            Self::King(_, p)
            | Self::Queen(_, p)
            | Self::Rook(_, p)
            | Self::Bishop(_, p)
            | Self::Knight(_, p)
            | Self::Pawn(_, p)
            => *p,
        }
    }

    /// Change the position of this piece to a new position.
    ///
    /// For example, `Pawn(Color::White, E4).move_to(E5)` will result in
    /// `Pawn(Color::White, E5)`. This does not check for move legality,
    /// it merely creates a new piece with the same color and type, but
    /// with a new position.
    #[inline]
    pub fn move_to(&self, new_pos: Position) -> Self {
        match *self {
            Self::King(c, _) => Self::King(c, new_pos),
            Self::Queen(c, _) => Self::Queen(c, new_pos),
            Self::Rook(c, _) => Self::Rook(c, new_pos),
            Self::Bishop(c, _) => Self::Bishop(c, new_pos),
            Self::Knight(c, _) => Self::Knight(c, new_pos),
            Self::Pawn(c, _) => Self::Pawn(c, new_pos),
        }
    }
}