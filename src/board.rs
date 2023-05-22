extern crate js_sys;
use crate::{piece::{Color, Position, Piece, BLACK, WHITE}};

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square {
    piece: Option<Piece>,
}

pub const EMPTY_SQUARE: Square = Square { piece: None };

impl From<Piece> for Square {
    fn from(piece: Piece) -> Self {
        Self { piece: Some(piece) }
    }
}

#[wasm_bindgen]
impl Square {
    pub fn get_piece_js(&self) -> JsValue {
        match &self.piece {
            Some(piece) => to_value(piece).unwrap(),
            None => JsValue::NULL,
        }
    }

}

impl Square {
    pub fn get_piece(&self) -> Option<Piece> {
        self.piece
    }
}

impl Default for Square {
    fn default() -> Self {
        Self {
            piece: None,
        }
    }
}

pub struct BoardBuilder {
    board: Board,
}

impl From<Board> for BoardBuilder {
    fn from(board: Board) -> Self {
        Self { board }
    }
}

impl Default for BoardBuilder {
    fn default() -> Self {
        let mut board = Board::empty();
        Self { board }
    }
}

impl BoardBuilder {
    pub fn row(mut self, piece: Piece) -> Self {
        let mut pos = piece.get_pos();
        while pos.get_col() > 0 {
            pos = pos.next_left()
        }

        for _ in 0..8 {
            *self.board.get_square(pos) = Square::from(piece.move_to(pos));
            pos = pos.next_right();
        }

        self
    }
    pub fn piece(mut self, piece: Piece) -> Self {
        let pos = piece.get_pos();
        *self.board.get_square(pos) = Square::from(piece);
        self
    }

    pub fn build(self) -> Board {
        self.board
    }
}

#[wasm_bindgen]
pub struct Board {
    squares: [Square; 64],
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        BoardBuilder::default()
        .piece(Piece::Rook(BLACK, A8))
        .piece(Piece::Knight(BLACK, B8))
        .piece(Piece::Bishop(BLACK, C8))
        .piece(Piece::Queen(BLACK, D8))
        .piece(Piece::King(BLACK, E8))
        .piece(Piece::Bishop(BLACK, F8))
        .piece(Piece::Knight(BLACK, G8))
        .piece(Piece::Rook(BLACK, H8))
        .row(Piece::Pawn(BLACK, A7))
        .row(Piece::Pawn(WHITE, A2))
        .piece(Piece::Rook(WHITE, A1))
        .piece(Piece::Knight(WHITE, B1))
        .piece(Piece::Bishop(WHITE, C1))
        .piece(Piece::Queen(WHITE, D1))
        .piece(Piece::King(WHITE, E1))
        .piece(Piece::Bishop(WHITE, F1))
        .piece(Piece::Knight(WHITE, G1))
        .piece(Piece::Rook(WHITE, H1))
        .build()
    }

    pub fn squares_num(&self) -> usize {
        self.squares.len()
    }

    pub fn squares_u8_value_js(&self) -> Box<[u8]> {
        Box::new(self.squares_u8_value())
    }

}
impl Board {
    pub fn empty() -> Self {
        Self {
            squares: [EMPTY_SQUARE; 64],
        }
    }
    #[inline]
    fn get_square(&mut self, pos: Position) -> &mut Square {
        &mut self.squares[((7 - pos.get_row()) * 8 + pos.get_col()) as usize]
    }

    pub fn squares_u8_value(&self) -> [u8; 64]  {
        let mut result = [0; 64];
        for (i, square) in self.squares.iter().enumerate() {
            result[i] = match square.piece {
                None => 0,
                Some(Piece::Pawn(Color::White, _)) => 1,
                Some(Piece::Knight(Color::White, _)) => 2,
                Some(Piece::Bishop(Color::White, _)) => 3,
                Some(Piece::Rook(Color::White, _)) => 4,
                Some(Piece::Queen(Color::White, _)) => 5,
                Some(Piece::King(Color::White, _)) => 6,
                Some(Piece::Pawn(Color::Black, _)) => 7,
                Some(Piece::Knight(Color::Black, _)) => 8,
                Some(Piece::Bishop(Color::Black, _)) => 9,
                Some(Piece::Rook(Color::Black, _)) => 10,
                Some(Piece::Queen(Color::Black, _)) => 11,
                Some(Piece::King(Color::Black, _)) => 12,
            };
        }

        result
    }
}

pub const A1: Position = Position::new(0, 0);
pub const A2: Position = Position::new(1, 0);
pub const A3: Position = Position::new(2, 0);
pub const A4: Position = Position::new(3, 0);
pub const A5: Position = Position::new(4, 0);
pub const A6: Position = Position::new(5, 0);
pub const A7: Position = Position::new(6, 0);
pub const A8: Position = Position::new(7, 0);

pub const B1: Position = Position::new(0, 1);
pub const B2: Position = Position::new(1, 1);
pub const B3: Position = Position::new(2, 1);
pub const B4: Position = Position::new(3, 1);
pub const B5: Position = Position::new(4, 1);
pub const B6: Position = Position::new(5, 1);
pub const B7: Position = Position::new(6, 1);
pub const B8: Position = Position::new(7, 1);

pub const C1: Position = Position::new(0, 2);
pub const C2: Position = Position::new(1, 2);
pub const C3: Position = Position::new(2, 2);
pub const C4: Position = Position::new(3, 2);
pub const C5: Position = Position::new(4, 2);
pub const C6: Position = Position::new(5, 2);
pub const C7: Position = Position::new(6, 2);
pub const C8: Position = Position::new(7, 2);

pub const D1: Position = Position::new(0, 3);
pub const D2: Position = Position::new(1, 3);
pub const D3: Position = Position::new(2, 3);
pub const D4: Position = Position::new(3, 3);
pub const D5: Position = Position::new(4, 3);
pub const D6: Position = Position::new(5, 3);
pub const D7: Position = Position::new(6, 3);
pub const D8: Position = Position::new(7, 3);

pub const E1: Position = Position::new(0, 4);
pub const E2: Position = Position::new(1, 4);
pub const E3: Position = Position::new(2, 4);
pub const E4: Position = Position::new(3, 4);
pub const E5: Position = Position::new(4, 4);
pub const E6: Position = Position::new(5, 4);
pub const E7: Position = Position::new(6, 4);
pub const E8: Position = Position::new(7, 4);

pub const F1: Position = Position::new(0, 5);
pub const F2: Position = Position::new(1, 5);
pub const F3: Position = Position::new(2, 5);
pub const F4: Position = Position::new(3, 5);
pub const F5: Position = Position::new(4, 5);
pub const F6: Position = Position::new(5, 5);
pub const F7: Position = Position::new(6, 5);
pub const F8: Position = Position::new(7, 5);

pub const G1: Position = Position::new(0, 6);
pub const G2: Position = Position::new(1, 6);
pub const G3: Position = Position::new(2, 6);
pub const G4: Position = Position::new(3, 6);
pub const G5: Position = Position::new(4, 6);
pub const G6: Position = Position::new(5, 6);
pub const G7: Position = Position::new(6, 6);
pub const G8: Position = Position::new(7, 6);

pub const H1: Position = Position::new(0, 7);
pub const H2: Position = Position::new(1, 7);
pub const H3: Position = Position::new(2, 7);
pub const H4: Position = Position::new(3, 7);
pub const H5: Position = Position::new(4, 7);
pub const H6: Position = Position::new(5, 7);
pub const H7: Position = Position::new(6, 7);
pub const H8: Position = Position::new(7, 7);