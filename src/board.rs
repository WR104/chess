extern crate js_sys;
use crate::{piece::{Color, Position, Piece, BLACK, WHITE}, 
            Evaluate,
            Move};

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square {
    piece: Option<Piece>,
}

pub const EMPTY_SQUARE: Square = Square { piece: None };

impl Default for Square {
    fn default() -> Self {
        Self {
            piece: None,
        }
    }
}

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

    pub fn column(mut self, piece: Piece) -> Self {
        let mut pos = piece.get_pos();
        while pos.get_row() > 0 {
            pos = pos.next_below()
        }

        for _ in 0..8 {
            *self.board.get_square(pos) = Square::from(piece.move_to(pos));
            pos = pos.next_above();
        }

        self
    }

    pub fn piece(mut self, piece: Piece) -> Self {
        let pos = piece.get_pos();
        *self.board.get_square(pos) = Square::from(piece);
        self
    }

    pub fn enable_castling(mut self) -> Self {
        self.board.black_castling_rights.enable_all();
        self.board.white_castling_rights.enable_all();
        self
    }

    pub fn disable_castling(mut self) -> Self {
        self.board.black_castling_rights.disable_all();
        self.board.white_castling_rights.disable_all();
        self
    }

    pub fn enable_queenside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.enable_queenside(),
            BLACK => self.board.black_castling_rights.enable_queenside(),
        }
        self
    }

    pub fn disable_queenside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.disable_queenside(),
            BLACK => self.board.black_castling_rights.disable_queenside(),
        }
        self
    }

    pub fn enable_kingside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.enable_kingside(),
            BLACK => self.board.black_castling_rights.enable_kingside(),
        }
        self
    }

    pub fn disable_kingside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.disable_kingside(),
            BLACK => self.board.black_castling_rights.disable_kingside(),
        }
        self
    }

    pub fn set_en_passant(mut self, position: Option<Position>) -> Self {
        self.board.en_passant = position;
        self
    }

    pub fn set_turn(mut self, color: Color) -> Self {
        self.board = self.board.set_turn(color);
        self
    }

    pub fn build(self) -> Board {
        self.board
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CastlingRights {
    kingside: bool,
    queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            kingside: true,
            queenside: true,
        }
    }
}

impl CastlingRights {
    pub fn can_kingside_castle(&self) -> bool {
        self.kingside
    }

    pub fn can_queenside_castle(&self) -> bool {
        self.queenside
    }

    fn disable_kingside(&mut self) {
        self.kingside = false
    }

    fn disable_queenside(&mut self) {
        self.queenside = false
    }

    fn disable_all(&mut self) {
        self.disable_kingside();
        self.disable_queenside()
    }

    fn enable_kingside(&mut self) {
        self.kingside = true
    }

    fn enable_queenside(&mut self) {
        self.queenside = true
    }

    fn enable_all(&mut self) {
        self.enable_kingside();
        self.enable_queenside()
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board {
    squares: [Square; 64],

    en_passant: Option<Position>,

    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,

    turn: Color,
}

impl Evaluate for Board {
    fn value_for(&self, color: Color) -> f64 {
        todo!()
    }

    fn get_current_player_color(&self) -> Color {
        todo!()
    }

    fn get_legal_moves(&self) -> Vec<crate::Move> {
        todo!()
    }

    fn apply_eval_move(&self, m: crate::Move) -> Self {
        todo!()
    }
    //...
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
        .enable_castling()
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
            en_passant: None,

            white_castling_rights: CastlingRights::default(),
            black_castling_rights: CastlingRights::default(),

            turn: WHITE,
        }
    }

    pub fn rating_bar() {

    }

    #[inline]
    pub fn get_turn_color(&self) -> Color {
        self.turn
    }

    pub fn get_en_passant(&self) -> Option<Position> {
        self.en_passant
    }

    pub fn remove_all(&self, color: Color) -> Self{
        let mut result = *self;
        for square in &mut result.squares {
            if let Some(piece) = square.get_piece() {
                if piece.get_color() == color {
                    *square = EMPTY_SQUARE
                }
            }
        }
        result
    }

    #[inline]
    pub fn set_turn(&self, color: Color) -> Self {
        let mut result = *self;
        result.turn = color;
        result
    }

    pub fn get_material_advantage(&self, color: Color) -> i32 {
        self.squares
            .iter()
            .map(|square| match square.get_piece() {
                Some(piece) => {
                    if piece.get_color() == color {
                        piece.get_material_value()
                    } else {
                        -piece.get_material_value()
                    }
                }
                None => 0,
            })
            .sum()
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

    #[inline]
    fn add_piece(&mut self, piece: Piece) {
        let pos = piece.get_pos();
        *self.get_square(pos) = Square::from(piece);
    }

    // Does a square have any piece?
    #[inline]
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.is_off_board() {
            return None;
        }
        self.squares[((7 - pos.get_row()) * 8 + pos.get_col()) as usize].get_piece()
    }

    #[inline]
    pub fn has_ally_piece(&self, pos: Position, ally_color: Color) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_color() == ally_color
        } else {
            false
        }
    }

    // If a square at a given position has an enemy piece from a given
    // ally color, return true. Otherwise, return false.
    //
    // For example, if a square has a black piece, and this method is called
    // upon it with an `ally_color` of `Color::White`, then it will return true.
    // If called with `Color::Black` upon the same square, however, it will return false.
    #[inline]
    pub fn has_enemy_piece(&self, pos: Position, ally_color: Color) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_color() == !ally_color
        } else {
            false
        }
    }

    // If a square at a given position has any piece, return true
    #[inline]
    pub fn has_piece(&self, pos: Position) -> bool {
        self.get_piece(pos) != None
    }

    pub fn has_no_piece(&self, pos: Position) -> bool {
        self.get_piece(pos) == None
    }

    pub fn get_king_pos(&self, color: Color) -> Option<Position> {
        let mut king_pos = None;
        for square in &self.squares {
            if let Some(Piece::King(c, pos)) = square.get_piece() {
                if c == color {
                    king_pos = Some(pos)
                }
            }
        }
        king_pos
    }

    // Is a square threatened by an enemy piece?
    pub fn is_threatened(&self, pos: Position, ally_color: Color) -> bool {
        for (i, square) in self.squares.iter().enumerate() {
            let row = 7 - i / 8;
            let col = i % 8;
            let square_pos = Position::new(row as i32, col as i32);
            if !square_pos.is_orthogonal_to(pos)
                && !square_pos.is_diagonal_to(pos)
                && !square_pos.is_knight_move(pos) {
                    continue;
            }

            if let Some(piece) = square.get_piece() {
                if piece.get_color() == ally_color {
                    continue;
                }
                if piece.is_leagal_attack(pos, self) {
                    return true
                }
            }
        }
        false
    }

    // Is the king of a given color in check
    pub fn is_in_check(&self, color: Color) -> bool {
        if let Some(king_pos) = self.get_king_pos(color) {
            self.is_threatened(king_pos, color)
        } else {
            false
        }
    }

    fn move_piece(&self, from: Position, to: Position, promotion: Option<Piece>) -> Self {
        let mut result = *self;
        result.en_passant = None;

        if from.is_off_board() || to.is_off_board() {
            return result;
        }

        let form_square = result.get_square(from);
        if let Some(mut piece) = form_square.get_piece() {
            *form_square = EMPTY_SQUARE;

            if piece.is_pawn() && (to.get_row() == 0 || to.get_row() == 7) {
                piece = match promotion {

                    Some(promotion) => {
                        if promotion.is_king() || promotion.is_pawn() {
                            Piece::Queen(piece.get_color(), piece.get_pos())
                        } else {
                            promotion
                                .with_color(piece.get_color())
                                .move_to(piece.get_pos())
                        }
                    }

                    // queen by default
                    None => Piece::Queen(piece.get_color(), piece.get_pos()),
                }
            }

            if piece.is_starting_pawn() && (from.get_row() - to.get_row()).abs() == 2 {
                result.en_passant = Some(to.pawn_back(piece.get_color()))
            }

            result.add_piece(piece.move_to(to));

            let castling_rights= match piece.get_color() {
                WHITE => &mut result.white_castling_rights,
                BLACK => &mut result.black_castling_rights,
            };

            if piece.is_king() {
                castling_rights.disable_all();
            } else if piece.is_queenside_rook() {
                castling_rights.disable_queenside();
            } else if piece.is_kingside_rook() {
                castling_rights.disable_kingside();
            }
        }

        result
    }

    // Can a given player castle kingside?
    pub fn can_kingside_castle(&self, color: Color) -> bool {
        let right_of_king = Position::king_pos(color).next_right();
        match color {
            WHITE => {
                self.has_no_piece(Position::new(0, 5))
                    && self.has_no_piece(Position::new(0, 6))
                    && self.get_piece(Position::new(0, 7))
                        == Some(Piece::Rook(color, Position::new(0, 7)))
                    && self.white_castling_rights.can_kingside_castle()
                    && !self.is_in_check(color)
                    && !self.is_threatened(right_of_king, color)
                    && !self.is_threatened(right_of_king.next_right(), color)
            }
            BLACK => {
                self.has_no_piece(Position::new(7, 5))
                    && self.has_no_piece(Position::new(7, 6))
                    && self.get_piece(Position::new(7, 7))
                        == Some(Piece::Rook(color, Position::new(7, 7)))
                    && self.black_castling_rights.can_kingside_castle()
                    && !self.is_in_check(color)
                    && !self.is_threatened(right_of_king, color)
                    && !self.is_threatened(right_of_king.next_right(), color)
            }
        }
    }

    pub fn can_queenside_castle(&self, color: Color) -> bool {
        match color {
            WHITE => {
                self.has_no_piece(Position::new(0, 1))
                    && self.has_no_piece(Position::new(0, 2))
                    && self.has_no_piece(Position::new(0, 3))
                    && self.get_piece(Position::new(0, 0))
                        == Some(Piece::Rook(color, Position::new(0, 0)))
                    && self.white_castling_rights.can_queenside_castle()
                    && !self.is_in_check(color)
                    && !self.is_threatened(Position::queen_pos(color), color)
            }
            BLACK => {
                self.has_no_piece(Position::new(7, 1))
                    && self.has_no_piece(Position::new(7, 2))
                    && self.has_no_piece(Position::new(7, 3))
                    && self.get_piece(Position::new(7, 0))
                        == Some(Piece::Rook(color, Position::new(7, 0)))
                    && self.black_castling_rights.can_queenside_castle()
                    && !self.is_in_check(color)
                    && !self.is_threatened(Position::queen_pos(color), color)
            }
        }
    }

    pub fn get_castling_rights(&self, color: Color) -> CastlingRights {
        match color {
            WHITE => self.white_castling_rights,
            BLACK => self.black_castling_rights,
        }
    }

    pub(crate) fn is_legal_move(&self, m: Move, player_color: Color) -> bool {
        match m {
            Move::KingSideCastle => self.can_kingside_castle(player_color),
            Move::QueenSideCastle => self.can_queenside_castle(player_color),
            Move::Piece(from, to) => match self.get_piece(from) {
                Some(Piece::Pawn(c, pos)) => {
                    let piece = Piece::Pawn(c, pos);
                    ((if let Some(en_passant) = self.en_passant {
                        (en_passant == from.pawn_up(ally_color).next_left()
                            || en_passant == from.pawn_up(player_color).next_left()
                                && en_passant == to)
                            && c == player_color
                    } else {
                        false
                    }) || piece.is_legal_move(to, self) && piece.get_color() == player_color)
                        && !self.apply_move(m).is_in_check(player_color)
                }

            },
            Move::Promotion(_, _, _) => todo!(),
            Move::Resign => todo!(),
        }
    }
    
    //is_legal_move()

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