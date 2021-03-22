use std::convert::{TryFrom, TryInto};

use crate::{
    board::{Color, Square, White},
    piece::PieceType,
};

use anyhow::{bail, Error};

pub const A1: Position = Position::new(7, 0);
pub const A2: Position = Position::new(6, 0);
pub const A3: Position = Position::new(5, 0);
pub const A4: Position = Position::new(4, 0);
pub const A5: Position = Position::new(3, 0);
pub const A6: Position = Position::new(2, 0);
pub const A7: Position = Position::new(1, 0);
pub const A8: Position = Position::new(0, 0);

pub const B1: Position = Position::new(7, 1);
pub const B2: Position = Position::new(6, 1);
pub const B3: Position = Position::new(5, 1);
pub const B4: Position = Position::new(4, 1);
pub const B5: Position = Position::new(3, 1);
pub const B6: Position = Position::new(2, 1);
pub const B7: Position = Position::new(1, 1);
pub const B8: Position = Position::new(0, 1);

pub const C1: Position = Position::new(7, 2);
pub const C2: Position = Position::new(6, 2);
pub const C3: Position = Position::new(5, 2);
pub const C4: Position = Position::new(4, 2);
pub const C5: Position = Position::new(3, 2);
pub const C6: Position = Position::new(2, 2);
pub const C7: Position = Position::new(1, 2);
pub const C8: Position = Position::new(0, 2);

pub const D1: Position = Position::new(7, 3);
pub const D2: Position = Position::new(6, 3);
pub const D3: Position = Position::new(5, 3);
pub const D4: Position = Position::new(4, 3);
pub const D5: Position = Position::new(3, 3);
pub const D6: Position = Position::new(2, 3);
pub const D7: Position = Position::new(1, 3);
pub const D8: Position = Position::new(0, 3);

pub const E1: Position = Position::new(7, 4);
pub const E2: Position = Position::new(6, 4);
pub const E3: Position = Position::new(5, 4);
pub const E4: Position = Position::new(4, 4);
pub const E5: Position = Position::new(3, 4);
pub const E6: Position = Position::new(2, 4);
pub const E7: Position = Position::new(1, 4);
pub const E8: Position = Position::new(0, 4);

pub const F1: Position = Position::new(7, 5);
pub const F2: Position = Position::new(6, 5);
pub const F3: Position = Position::new(5, 5);
pub const F4: Position = Position::new(4, 5);
pub const F5: Position = Position::new(3, 5);
pub const F6: Position = Position::new(2, 5);
pub const F7: Position = Position::new(1, 5);
pub const F8: Position = Position::new(0, 5);

pub const G1: Position = Position::new(7, 6);
pub const G2: Position = Position::new(6, 6);
pub const G3: Position = Position::new(5, 6);
pub const G4: Position = Position::new(4, 6);
pub const G5: Position = Position::new(3, 6);
pub const G6: Position = Position::new(2, 6);
pub const G7: Position = Position::new(1, 6);
pub const G8: Position = Position::new(0, 6);

pub const H1: Position = Position::new(7, 7);
pub const H2: Position = Position::new(6, 7);
pub const H3: Position = Position::new(5, 7);
pub const H4: Position = Position::new(4, 7);
pub const H5: Position = Position::new(3, 7);
pub const H6: Position = Position::new(2, 7);
pub const H7: Position = Position::new(1, 7);
pub const H8: Position = Position::new(0, 7);

#[derive(Clone, Copy, PartialEq, Default, Hash, Debug)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn is_on_board(&self) -> bool {
        -1 < self.row && self.row < 8 && -1 < self.col && self.col < 8
    }

    pub fn is_there_a_piece(self, board: &[Square; 64]) -> bool {
        board[usize::from(self)].is_some()
    }

    pub fn is_there_a_piece_color(self, board: &[Square; 64], color: Color) -> bool {
        if let Some(piece) = board[usize::from(self)] {
            piece.color == color
        } else {
            false
        }
    }

    pub fn is_there_a_piece_type_color(
        self,
        board: &[Square; 64],
        piece_type: PieceType,
        color: Color,
    ) -> bool {
        if let Some(piece) = board[usize::from(self)] {
            piece.r#type == piece_type && piece.color == color
        } else {
            false
        }
    }

    pub fn can_en_passant(self, en_passant: Option<Position>) -> bool {
        if let Some(en_passant) = en_passant {
            if self == en_passant {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_pawn_starting_pos(self, color: Color) -> bool {
        let starting_row = if color == White { 6 } else { 1 };
        self.row == starting_row
    }

    pub fn is_pawn_promotion_pos(self, color: Color) -> bool {
        let promotion_row = if color == White { 0 } else { 7 };
        self.row == promotion_row
    }

    pub fn is_kingside_rook(self) -> bool {
        self.col == 7
    }

    pub fn is_queenside_rook(self) -> bool {
        self.col == 0
    }

    pub fn up_color(self, color: Color, num: i32) -> Self {
        let num = if color == White { -num } else { num };
        Self {
            row: self.row + num,
            ..self
        }
    }

    pub fn down_color(self, color: Color, num: i32) -> Self {
        let num = if color == White { num } else { -num };
        Self {
            row: self.row + num,
            ..self
        }
    }

    pub fn up(self, num: i32) -> Self {
        Self {
            row: self.row - num,
            ..self
        }
    }

    pub fn down(self, num: i32) -> Self {
        Self {
            row: self.row + num,
            ..self
        }
    }

    pub fn right(self, num: i32) -> Self {
        Self {
            col: self.col + num,
            ..self
        }
    }

    pub fn left(self, num: i32) -> Self {
        Self {
            col: self.col - num,
            ..self
        }
    }
}

impl From<Position> for usize {
    fn from(pos: Position) -> Self {
        (pos.row * 8 + pos.col) as usize
    }
}

impl From<usize> for Position {
    fn from(pos: usize) -> Self {
        Self {
            row: (pos / 8) as i32,
            col: (pos % 8) as i32,
        }
    }
}

impl TryFrom<&str> for Position {
    type Error = Error;

    fn try_from(pos: &str) -> Result<Self, Self::Error> {
        let mut res = Position::default();

        res.col = match pos.as_bytes().get(0) {
            Some(&b'a') => 0,
            Some(&b'b') => 1,
            Some(&b'c') => 2,
            Some(&b'd') => 3,
            Some(&b'e') => 4,
            Some(&b'f') => 5,
            Some(&b'g') => 6,
            Some(&b'h') => 7,
            _ => bail!("error parsing position: {}", pos),
        };

        res.row = match pos.as_bytes().get(1) {
            Some(&b'8') => 0,
            Some(&b'7') => 1,
            Some(&b'6') => 2,
            Some(&b'5') => 3,
            Some(&b'4') => 4,
            Some(&b'3') => 5,
            Some(&b'2') => 6,
            Some(&b'1') => 7,
            _ => bail!("error parsing position: {}", pos),
        };

        Ok(res)
    }
}

impl Into<String> for Position {
    fn into(self) -> String {
        format!(
            "{}{}",
            match self.col {
                0 => "a",
                1 => "b",
                2 => "c",
                3 => "d",
                4 => "e",
                5 => "f",
                6 => "g",
                7 => "h",
                _ => panic!("position col error"),
            },
            match self.row {
                7 => "1",
                6 => "2",
                5 => "3",
                4 => "4",
                3 => "5",
                2 => "6",
                1 => "7",
                0 => "8",
                _ => panic!("position col error"),
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub prom: Option<PieceType>,
}

impl TryFrom<String> for Move {
    type Error = Error;

    fn try_from(r#move: String) -> Result<Self, Self::Error> {
        match r#move.as_str() {
            _ => Ok(Move {
                from: (&r#move[0..2]).try_into()?,
                to: (&r#move[2..4]).try_into()?,
                prom: match r#move.as_bytes().get(4) {
                    Some(&b'k') => Some(PieceType::Knight),
                    Some(&b'b') => Some(PieceType::Bishop),
                    Some(&b'r') => Some(PieceType::Rook),
                    Some(&b'q') => Some(PieceType::Queen),
                    Some(_) => bail!("error parsing move: {}", r#move),
                    None => None,
                },
            }),
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = Error;

    fn try_from(r#move: &str) -> Result<Self, Self::Error> {
        match r#move {
            _ => Ok(Move {
                from: (&r#move[0..2]).try_into()?,
                to: (&r#move[2..4]).try_into()?,
                prom: match r#move.as_bytes().get(4) {
                    Some(&b'k') => Some(PieceType::Knight),
                    Some(&b'b') => Some(PieceType::Bishop),
                    Some(&b'r') => Some(PieceType::Rook),
                    Some(&b'q') => Some(PieceType::Queen),
                    Some(_) => bail!("error parsing move: {}", r#move),
                    None => None,
                },
            }),
        }
    }
}

impl Into<String> for Move {
    fn into(self) -> String {
        format!(
            "{}{}{}",
            Into::<String>::into(self.from),
            Into::<String>::into(self.to),
            match self.prom {
                Some(PieceType::Knight) => "k",
                Some(PieceType::Bishop) => "b",
                Some(PieceType::Rook) => "r",
                Some(PieceType::Queen) => "q",
                _ => ""
            }
        )
    }
}
