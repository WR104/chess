use crate::{board::Board, game::Move};
use core::convert::TryFrom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

pub const WHITE: Color = Color::White;
pub const BLACK: Color = Color::Black;

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::White => "White",
                Self::Black => "Black",
            }
        )
    }
}

//A color can be inverted using the "!" operator.
impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

/* =================================================================================
=================================================================================*/

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    row: i32,
    col: i32,
}

impl core::fmt::Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}{}",
            match self.col {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => '?',
            },
            self.row + 1
        )
    }
}

impl Position {
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    #[inline]
    pub const fn king_pos(color: Color) -> Self {
        match color {
            Color::White => Self::new(0, 4),
            Color::Black => Self::new(7, 4),
        }
    }

    #[inline]
    pub const fn queen_pos(color: Color) -> Self {
        match color {
            Color::White => Self::new(0, 3),
            Color::Black => Self::new(7, 3),
        }
    }
    //Parse a position from PGN. for example: 'e4' and 'D8'
    pub fn pgn(s: &str) -> Result<Self, String> {
        let s = s.trim().to_lowercase();
        let col = s.chars().next().ok_or(format!("invalid pgn `{}`", s))?;
        let row = s
            .chars()
            .nth(1)
            .ok_or(format!("invalid pgn '{}'", s))?
            .to_string()
            .parse::<u32>()
            .map_err(|_| format!("invalid pgn `{}`", s))? as i32;
        let c = match col {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err(format!("invalid column character in pgn `{}`", col)),
        };

        if 1 <= row || row <= 8 {
            Ok(Self::new(row - 1, c))
        } else {
            Err(format!("invalid row number `{}`", row))
        }
    }

    //Is this position a valid spot on the board?
    #[inline]
    pub fn is_on_board(&self) -> bool {
        !self.is_off_board()
    }

    //Is this position NOT a valid spot on the board?
    #[inline]
    pub fn is_off_board(&self) -> bool {
        self.row < 0 || self.row > 7 || self.col < 0 || self.col > 7
    }

    #[inline]
    pub fn get_row(&self) -> i32 {
        self.row
    }

    #[inline]
    pub fn get_col(&self) -> i32 {
        self.col
    }

    #[inline]
    fn add_row(&self, drow: i32) -> Self {
        let mut result = *self;
        result.row += drow;
        result
    }

    #[inline]
    fn add_col(&self, dcol: i32) -> Self {
        let mut result = *self;
        result.col += dcol;
        result
    }

    //Is this position diagonal to another position?
    pub fn is_diagonal_to(&self, other: Self) -> bool {
        (self.col - other.col).abs() == (self.row - other.row).abs()
    }

    //Get the diagonal distance between two positions?
    fn diagonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs()
    }

    //Is this position orthogonal to another position?
    pub fn is_orthogonal_to(&self, other: Self) -> bool {
        (self.col == other.col) || (self.row == other.row)
    }

    //get the orthogonal distance between two positions
    fn orthogonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs() + (self.row - other.row).abs()
    }

    //Is this position adjacent to another position?
    pub fn is_adjacent_to(&self, other: Self) -> bool {
        if self.is_orthogonal_to(other) {
            self.orthogonal_distance(other) == 1
        } else if self.is_diagonal_to(other) {
            self.diagonal_distance(other) == 1
        } else {
            false
        }
    }

    //Is this position beneath another position on the board?
    //Pieces "beneath" other pieces on the board have lower ranks.
    //for example, A7 is below A8
    pub fn is_below(&self, other: Self) -> bool {
        self.row < other.row
    }

    pub fn is_above(&self, other: Self) -> bool {
        self.row > other.row
    }

    //Is this position left of another position on the board?
    //Pieces "left of" other pieces on the board have a lower
    //lexigraphical column character
    //for example, A8 is left of B8
    pub fn is_left_of(&self, other: Self) -> bool {
        self.col < other.col
    }

    pub fn is_right_of(&self, other: Self) -> bool {
        self.col > other.col
    }

    //Get the position directly below this position.
    //
    //IMPORTANT NOTE: This will NOT check for positions
    //off of the board! You could easily get an invalid
    //position if you do not check with the `is_on_board`
    //method
    pub fn next_below(&self) -> Self {
        Self::new(self.row - 1, self.col)
    }

    // Get the position directly above this position.
    //
    // IMPORTANT NOTE: This will NOT check for positions
    // off of the board! You could easily get an invalid
    // position if you do not check with the `is_on_board`
    // method!
    #[inline]
    pub fn next_above(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    //Get the next square upwards from a respective player's
    //pawn.
    //
    // IMPORTANT NOTE: This will NOT check for positions
    // off of the board! You could easily get an invalid
    // position if you do not check with the `is_on_board`
    // method!
    pub fn pawn_up(&self, ally_color: Color) -> Self {
        match ally_color {
            Color::White => self.next_above(),
            Color::Black => self.next_below(),
        }
    }

    //Get the next square backward from a respective player's
    //pawn.
    //
    // IMPORTANT NOTE: This will NOT check for positions
    // off of the board! You could easily get an invalid
    // position if you do not check with the `is_on_board`
    // method!
    pub fn pawn_back(&self, ally_color: Color) -> Self {
        self.pawn_up(!ally_color)
    }

    //Get the position directly below this position.

    // Get the position directly left of this position.
    //
    // IMPORTANT NOTE: This will NOT check for positions
    // off of the board! You could easily get an invalid
    // position if you do not check with the `is_on_board`
    // method!
    #[inline]
    pub fn next_left(&self) -> Self {
        Self::new(self.row, self.col - 1)
    }

    // Get the position directly right of this position.
    //
    // IMPORTANT NOTE: This will NOT check for positions
    // off of the board! You could easily get an invalid
    // position if you do not check with the `is_on_board`
    // method!
    #[inline]
    pub fn next_right(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    #[inline]
    pub fn is_starting_pawn(&self, color: Color) -> bool {
        match color {
            Color::White => self.row == 1,
            Color::Black => self.row == 6,
        }
    }

    //Is this the starting position of the kingside rook?
    pub fn is_kingside_rook(&self) -> bool {
        (self.row == 0 || self.row == 7) && self.col == 7
    }

    //Is this the starting position of the queenside rook?
    pub fn is_queenside_rook(&self) -> bool {
        (self.row == 0 || self.row == 7) && self.col == 0
    }

    // Get the list of positions from this position to another
    // position, moving diagonally.
    //
    // This does _not_ include the `from` position, and includes the `to` position.
    pub fn diagonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_diagonal_to(to) {
            return Vec::new();
        }

        let row_step;
        let col_step;
        if self.is_left_of(to) {
            col_step = 1;
        } else {
            col_step = -1;
        }

        if self.is_below(to) {
            row_step = 1;
        } else {
            row_step = -1;
        }

        let mut acc = *self;
        let mut result = Vec::new();
        for _ in 0..self.diagonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    //Get the list of positions from this position to another
    //position, moving orthogonally.
    //
    //This does _not_ include the `from` position, and includes the `to` position.
    pub fn orthogonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_orthogonal_to(to) {
            return Vec::new();
        }
        let mut row_step = 0;
        let mut col_step = 0;
        if self.is_left_of(to) {
            col_step = 1;
        } else if self.is_right_of(to) {
            col_step = -1;
        } else if self.is_above(to) {
            row_step = -1;
        } else if self.is_below(to) {
            row_step = 1;
        }

        let mut acc = *self;
        let mut result = Vec::new();

        for _ in 0..self.orthogonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    #[inline]
    pub fn is_knight_move(&self, other: Self) -> bool {
        (self.row - other.row).abs() == 2 && (self.col - other.col).abs() == 1
            || (self.row - other.row).abs() == 1 && (self.col - other.col).abs() == 2
    }
}

/* =================================================================================
=================================================================================*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    King(Color, Position),
    Queen(Color, Position),
    Rook(Color, Position),
    Bishop(Color, Position),
    Knight(Color, Position),
    Pawn(Color, Position),
}

impl TryFrom<&str> for Piece {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let color = Color::Black;
        let position = Position::new(-1, -1);
        match name {
            "king" => Ok(Self::King(color, position)),
            "queen" => Ok(Self::Queen(color, position)),
            "rook" => Ok(Self::Rook(color, position)),
            "bishop" => Ok(Self::Bishop(color, position)),
            "knight" => Ok(Self::Knight(color, position)),
            "pawn" => Ok(Self::Pawn(color, position)),
            _ => Err(String::from("invalid piece name")),
        }
    }
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
    pub fn get_type(&self) -> &str {
        match self {
            Self::King(_, _) => "K",
            Self::Queen(_, _) => "Q",
            Self::Rook(_, _) => "R",
            Self::Bishop(_, _) => "B",
            Self::Knight(_, _) => "N",
            Self::Pawn(_, _) => "P",
        } 
    }

    #[inline]
    pub fn get_material_value(&self) -> i32 {
        match self {
            Self::King(_, _) => 9999,
            Self::Queen(_, _) => 9,
            Self::Rook(_, _) => 5,
            Self::Bishop(_, _) => 3,
            Self::Knight(_, _) => 3,
            Self::Pawn(_, _) => 1,
        }
    }

    // Get the weighted value of a piece. This simply factors in position
    // to the pieces value. For example, a knight that is in the center is
    // more favorable than a knight on the side of the board. Similarly,
    // a king in the center of the board is highly unfavorable compared to
    // a king its respective side.
    //
    // Additionally, the weighted value of the piece is 10 times greater than
    // its material value, plus or minus a weight ranging between 5.0 and -5.0.
    #[inline]
    pub fn get_weighted_value(&self) -> f64 {
        let weights = match self {
            Self::King(c, _) => match c {
                Color::White => WHITE_KING_POSITION_WEIGHTS,
                Color::Black => BLACK_KING_POSITION_WEIGHTS,
            },
            Self::Queen(c, _) => match c {
                Color::White => WHITE_QUEEN_POSITION_WEIGHTS,
                Color::Black => BLACK_QUEEN_POSITION_WEIGHTS,
            },
            Self::Rook(c, _) => match c {
                Color::White => WHITE_ROOK_POSITION_WEIGHTS,
                Color::Black => BLACK_ROOK_POSITION_WEIGHTS,
            },
            Self::Bishop(c, _) => match c {
                Color::White => WHITE_BISHOP_POSITION_WEIGHTS,
                Color::Black => BLACK_BISHOP_POSITION_WEIGHTS,
            },
            Self::Knight(c, _) => match c {
                Color::White => WHITE_KNIGHT_POSITION_WEIGHTS,
                Color::Black => BLACK_KNIGHT_POSITION_WEIGHTS,
            },
            Self::Pawn(c, _) => match c {
                Color::White => WHITE_PAWN_POSITION_WEIGHTS,
                Color::Black => BLACK_PAWN_POSITION_WEIGHTS,
            },
        };
        weights[(7 - self.get_pos().get_row()) as usize][self.get_pos().get_col() as usize]
            + (self.get_material_value() * 10) as f64
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        match self {
            Self::King(c, _)
            | Self::Queen(c, _)
            | Self::Rook(c, _)
            | Self::Bishop(c, _)
            | Self::Knight(c, _)
            | Self::Pawn(c, _) => *c,
        }
    }

    #[inline]
    pub fn with_color(&self, color: Color) -> Self {
        match *self {
            Self::King(_, pos) => Self::King(color, pos),
            Self::Queen(_, pos) => Self::Queen(color, pos),
            Self::Rook(_, pos) => Self::Rook(color, pos),
            Self::Bishop(_, pos) => Self::Bishop(color, pos),
            Self::Knight(_, pos) => Self::Knight(color, pos),
            Self::Pawn(_, pos) => Self::Pawn(color, pos),
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
            | Self::Pawn(_, p) => *p,
        }
    }

    #[inline]
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King(_, _))
    }

    #[inline]
    pub fn is_queen(&self) -> bool {
        matches!(self, Self::Queen(_, _))
    }

    #[inline]
    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook(_, _))
    }

    #[inline]
    pub fn is_bishop(&self) -> bool {
        matches!(self, Self::Bishop(_, _))
    }

    #[inline]
    pub fn is_knight(&self) -> bool {
        matches!(self, Self::Knight(_, _))
    }

    #[inline]
    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn(_, _))
    }

    // A starting pawn is a pawn that has not been pushed
    #[inline]
    pub fn is_starting_pawn(&self) -> bool {
        if let Self::Pawn(c, pos) = self {
            pos.is_starting_pawn(*c)
        } else {
            false
        }
    }

    pub fn is_queenside_rook(&self) -> bool {
        if let Self::Rook(_, pos) = self {
            pos.is_queenside_rook()
        } else {
            false
        }
    }

    pub fn is_kingside_rook(&self) -> bool {
        if let Self::Rook(_, pos) = self {
            pos.is_kingside_rook()
        } else {
            false
        }
    }

    // Change the position of this piece to a new position.
    // For example, `Pawn(Color::White, E4).move_to(E5)` will result in
    // `Pawn(Color::White, E5)`. This does not check for move legality,
    // it merely creates a new piece with the same color and type, but
    // with a new position.
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

    #[inline]
    pub(crate) fn get_legal_moves(&self, board: &Board) -> Vec<Move> {
        let mut result = Vec::new();
        match *self {
            Self::Pawn(ally_color, pos) => {
                let up = pos.pawn_up(ally_color);
                let next_up = up.pawn_up(ally_color);
                let up_left = up.next_left();
                let up_right = up.next_right();

                if let Some(en_passant) = board.get_en_passant() {
                    if en_passant == up_left || en_passant == up_right {
                        result.push(Move::Piece(pos, en_passant));
                    }
                }

                if next_up.is_on_board()
                    && self.is_starting_pawn()
                    && board.has_no_piece(up)
                    && board.has_no_piece(next_up)
                {
                    result.push(Move::Piece(pos, next_up))
                }

                if up.is_on_board() && board.has_no_piece(up) {
                    result.push(Move::Piece(pos, up))
                }

                if up_left.is_on_board() && board.has_enemy_piece(up_left, ally_color) {
                    result.push(Move::Piece(pos, up_left))
                }

                if up_right.is_on_board() && board.has_enemy_piece(up_right, ally_color) {
                    result.push(Move::Piece(pos, up_right))
                }
            }

            Self::King(ally_color, pos) => {
                for p in &[
                    pos.next_left(),
                    pos.next_right(),
                    pos.next_above(),
                    pos.next_below(),
                    pos.next_left().next_above(),
                    pos.next_left().next_below(),
                    pos.next_right().next_above(),
                    pos.next_right().next_below(),
                ] {
                    if p.is_on_board() && !board.has_ally_piece(*p, ally_color) {
                        result.push(Move::Piece(pos, *p))
                    }
                }
                if board.can_kingside_castle(ally_color) {
                    result.push(Move::KingSideCastle);
                } else if board.can_queenside_castle(ally_color) {
                    result.push(Move::QueenSideCastle);
                }
            }

            Self::Queen(ally_color, pos) => {
                for row in 0..8 {
                    let new_pos = Position::new(row, pos.get_col());
                    if new_pos != pos
                        && !board.has_ally_piece(new_pos, ally_color)
                        && new_pos.is_orthogonal_to(pos)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
                for col in 0..8 {
                    let new_pos = Position::new(pos.get_row(), col);
                    if new_pos != pos
                        && !board.has_ally_piece(new_pos, ally_color)
                        && new_pos.is_orthogonal_to(pos)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }

                for row in 0..8 {
                    for col in 0..8 {
                        let new_pos = Position::new(row, col);
                        if new_pos != pos
                            && !board.has_ally_piece(new_pos, ally_color)
                            && new_pos.is_diagonal_to(pos)
                        {
                            result.push(Move::Piece(pos, new_pos));
                        }
                    }
                }
            }

            Self::Rook(ally_color, pos) => {
                for row in 0..8 {
                    let new_pos = Position::new(row, pos.get_col());
                    if new_pos != pos
                        && !board.has_ally_piece(new_pos, ally_color)
                        && new_pos.is_orthogonal_to(pos)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
                for col in 0..8 {
                    let new_pos = Position::new(pos.get_row(), col);
                    if new_pos != pos
                        && !board.has_ally_piece(new_pos, ally_color)
                        && new_pos.is_orthogonal_to(pos)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
            }

            Self::Bishop(ally_color, pos) => {
                for row in 0..8 {
                    for col in 0..8 {
                        let new_pos = Position::new(row, col);
                        if new_pos != pos
                            && !board.has_ally_piece(new_pos, ally_color)
                            && new_pos.is_diagonal_to(pos)
                        {
                            result.push(Move::Piece(pos, new_pos));
                        }
                    }
                }
            }

            Self::Knight(ally_color, pos) => {
                for p in &[
                    pos.next_left().next_left().next_above(),
                    pos.next_left().next_above().next_above(),
                    pos.next_left().next_left().next_below(),
                    pos.next_left().next_below().next_below(),
                    pos.next_right().next_right().next_above(),
                    pos.next_right().next_above().next_above(),
                    pos.next_right().next_right().next_below(),
                    pos.next_right().next_below().next_below(),
                ] {
                    if p.is_on_board() && !board.has_ally_piece(*p, ally_color) {
                        result.push(Move::Piece(pos, *p))
                    }
                }
            }
        }

        let color = self.get_color();
        result
            .into_iter()
            .filter(|x| match x {
                Move::Piece(from, to) => {
                    if from.is_on_board() && to.is_on_board() {
                        board.is_legal_move(*x, color)
                    } else {
                        false
                    }
                }
                _ => board.is_legal_move(*x, color),
            })
            .collect::<Vec<Move>>()
    }

    // Verify that moving to a new position is a legal move.
    pub(crate) fn is_legal_move(&self, new_pos: Position, board: &Board) -> bool {
        if board.has_ally_piece(new_pos, self.get_color()) || new_pos.is_off_board() {
            return false;
        }

        match *self {
            Self::Pawn(ally_color, pos) => {
                let up = pos.pawn_up(ally_color);
                let up_left = up.next_left();
                let up_right = up.next_right();

                (if let Some(en_passant) = board.get_en_passant() {
                    (en_passant == up_left || en_passant == up_right) && (new_pos == en_passant)
                } else {
                    false
                }) || (self.is_starting_pawn()
                    && board.has_no_piece(new_pos)
                    && board.has_no_piece(up)
                    && new_pos == up.pawn_up(ally_color))
                    || (board.has_enemy_piece(new_pos, ally_color) && new_pos == up_left)
                    || (board.has_enemy_piece(new_pos, ally_color) && new_pos == up_right)
                    || (board.has_no_piece(new_pos) && new_pos == up)
            }

            Self::King(_, pos) => pos.is_adjacent_to(new_pos),

            Self::Queen(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Rook(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Bishop(_, pos) => {
                if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Knight(_, pos) => pos.is_knight_move(new_pos),
        }
    }

    // Verify that attacking a given square is a legal move.
    pub(crate) fn is_legal_attack(&self, new_pos: Position, board: &Board) -> bool {
        if board.has_ally_piece(new_pos, self.get_color()) || new_pos.is_off_board() {
            return false;
        }

        match *self {
            Self::Pawn(ally_color, pos) => {
                let up = pos.pawn_up(ally_color);
                (if let Some(en_passant) = board.get_en_passant() {
                    (en_passant == up.next_left() || en_passant == up.next_right())
                        && (new_pos == en_passant)
                } else {
                    false
                }) || new_pos == up.next_left()
                    || new_pos == up.next_right()
            }

            Self::King(_, pos) => pos.is_adjacent_to(new_pos),

            Self::Queen(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Rook(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Bishop(_, pos) => {
                if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Knight(_, pos) => pos.is_knight_move(new_pos),
        }
    }
}

const WHITE_KING_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
    [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0],
];

const BLACK_KING_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0],
    [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
    [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [-2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
];

const WHITE_QUEEN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, -0.0, -1.0, -0.5, -0.5, -0.5, -1.0, -2.0],
];
const BLACK_QUEEN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-1.0, -0.0, -1.0, -0.5, -0.5, -0.5, -1.0, -2.0],
    [-1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
];

const WHITE_ROOK_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0],
];

const BLACK_ROOK_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

const WHITE_BISHOP_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -1.0],
    [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
    [-1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0],
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
];

const BLACK_BISHOP_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [-1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
    [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -1.0],
    [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
];

const WHITE_KNIGHT_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0],
    [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0],
    [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0],
    [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0],
    [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0],
    [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
];

const BLACK_KNIGHT_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0],
    [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0],
    [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0],
    [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0],
    [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0],
    [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
];

const WHITE_PAWN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
    [1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0],
    [0.5, 0.5, 1.0, 2.5, 2.5, 1.0, 0.5, 0.5],
    [0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    [0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5],
    [0.5, 1.5, -1.0, -2.0, -2.0, 1.0, 1.5, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

const BLACK_PAWN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 1.5, -1.0, -2.0, -2.0, 1.0, 1.5, 0.5],
    [0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5],
    [0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    [0.5, 0.5, 1.0, 2.5, 2.5, 1.0, 0.5, 0.5],
    [1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0],
    [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];
