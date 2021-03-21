use core::panic;
use std::{fmt::Display, ops::Not, sync::atomic::AtomicI32};

use crate::{fen, negamax, piece::*, position::*};

use anyhow::Result;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}
pub use Color::*;

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            White => Black,
            Black => White,
        }
    }
}

pub type Square = Option<Piece>;

#[derive(Clone, Copy, Debug)]
pub struct CastleRight {
    kingside: bool,
    queenside: bool,
}

impl CastleRight {
    pub fn new() -> Self {
        Self {
            kingside: false,
            queenside: false,
        }
    }

    fn off(&mut self) {
        self.kingside = false;
        self.queenside = false;
    }

    pub fn set_kingside_castle_on(&mut self) {
        self.kingside = true
    }

    pub fn set_queenside_castle_on(&mut self) {
        self.queenside = true
    }

    pub fn can_castle(self) -> bool {
        self.kingside || self.queenside
    }

    pub fn can_kingside_castle(self) -> bool {
        self.kingside
    }

    pub fn can_queenside_castle(self) -> bool {
        self.queenside
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Chess {
    pub board: [Square; 64],

    pub white_castle: CastleRight,
    pub black_castle: CastleRight,

    pub en_passant: Option<Position>,
    pub turn: Color,
}

impl Chess {
    pub fn new() -> Self {
        fen::parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self> {
        fen::parse(fen)
    }

    pub fn set_at(&mut self, pos: Position, mut piece: Piece) {
        piece.pos = pos;
        self.board[usize::from(pos)] = Some(piece)
    }

    pub fn remove_at(&mut self, pos: Position) {
        self.board[usize::from(pos)] = None
    }

    fn take_at(&mut self, pos: Position) -> Option<Piece> {
        self.board[usize::from(pos)].take()
    }

    fn get_all_pieces_turn(&self) -> Vec<Piece> {
        self.board
            .iter()
            .filter_map(|&square| square)
            .filter(|piece| piece.color == self.turn)
            .collect()
    }

    fn get_all_pieces(&self) -> Vec<Piece> {
        self.board.iter().filter_map(|&square| square).collect()
    }

    pub fn gen_moves(&self) -> Vec<Move> {
        self.get_all_pieces_turn()
            .iter()
            .flat_map(|piece| piece.gen_moves(&self))
            .collect()
    }

    pub fn gen_legal_moves(&self) -> Vec<Move> {
        self.gen_moves()
            .iter()
            .filter(|&&r#move| !self.set_move(r#move).is_check())
            .map(|&r#move| r#move)
            .collect()
    }

    pub fn get_king(&self) -> Option<Piece> {
        self.get_all_pieces_turn()
            .iter()
            .find(|&&piece| piece.r#type == King && piece.color == self.turn)
            .map(|&piece| piece)
    }

    pub fn is_check(&self) -> bool {
        if let Some(king) = self.get_king() {
            king.am_i_being_attacked(&self.board)
        } else {
            false
        }
    }

    pub fn set_move(mut self, r#move: Move) -> Self {
        self.apply_move(r#move);
        self
    }

    pub fn set(&mut self, r#move: Move) {
        self.apply_move(r#move);
        self.turn = !self.turn;
    }

    //TODO: test if a move is legal
    pub fn apply_move(&mut self, r#move: Move) {
        let Move { from, to, prom } = r#move;

        if let Some(mut piece) = self.take_at(from) {
            match piece.r#type {
                King => {
                    if self.turn == White && self.white_castle.can_castle() {
                        if to == G1 {
                            if let Some(rook) = self.take_at(piece.pos.right(3)) {
                                self.set_at(piece.pos.right(2), piece);
                                self.set_at(piece.pos.right(1), rook);

                                self.white_castle.off()
                            } else {
                                panic!("must be a rook for castle or no kingside right")
                            }
                        } else if to == C1 {
                            if let Some(rook) = self.take_at(piece.pos.left(4)) {
                                self.set_at(piece.pos.left(2), piece);
                                self.set_at(piece.pos.left(1), rook);

                                self.white_castle.off()
                            } else {
                                panic!("must be a rook for castle or no queenside right")
                            }
                        } else {
                            self.white_castle.off()
                        }
                    } else if self.turn == Black && self.black_castle.can_castle() {
                        if to == G8 {
                            if let Some(rook) = self.take_at(piece.pos.right(3)) {
                                self.set_at(piece.pos.right(2), piece);
                                self.set_at(piece.pos.right(1), rook);

                                self.black_castle.off()
                            } else {
                                panic!("must be a rook for castle or no kingside right")
                            }
                        } else if to == C8 {
                            if let Some(rook) = self.take_at(piece.pos.left(4)) {
                                self.set_at(piece.pos.left(2), piece);
                                self.set_at(piece.pos.left(1), rook);

                                self.black_castle.off()
                            } else {
                                panic!("must be a rook for castle or no queenside right")
                            }
                        } else {
                            self.black_castle.off()
                        }
                    }
                }
                Rook => {
                    if self.turn == White {
                        if self.white_castle.can_kingside_castle() && piece.pos.is_kingside_rook() {
                            self.white_castle.kingside = false;
                        } else if self.white_castle.can_queenside_castle()
                            && piece.pos.is_queenside_rook()
                        {
                            self.white_castle.queenside = false;
                        }
                    } else {
                        if self.black_castle.can_kingside_castle() && piece.pos.is_kingside_rook() {
                            self.black_castle.kingside = false;
                        } else if self.black_castle.can_queenside_castle()
                            && piece.pos.is_queenside_rook()
                        {
                            self.black_castle.queenside = false;
                        }
                    }
                }
                Pawn => {
                    if let Some(en_passant) = self.en_passant {
                        if to == en_passant {
                            self.remove_at(en_passant.up_color(!self.turn, 1))
                        }
                    }
                    if (from.row - to.row).abs() == 2 {
                        self.en_passant = Some(to.down_color(self.turn, 1));
                        self.set_at(to, piece);
                        return;
                    }
                }
                _ => (),
            }

            if let Some(piece_type) = prom {
                piece.r#type = piece_type
            }

            if let Some(piece_captured) = self.take_at(to) {
                if piece_captured.r#type == Rook {
                    if self.turn == White {
                        if self.black_castle.can_kingside_castle()
                            && piece_captured.pos.is_kingside_rook()
                        {
                            self.black_castle.kingside = false;
                        } else if self.black_castle.can_queenside_castle()
                            && piece_captured.pos.is_queenside_rook()
                        {
                            self.black_castle.queenside = false;
                        }
                    } else {
                        if self.white_castle.can_kingside_castle()
                            && piece_captured.pos.is_kingside_rook()
                        {
                            self.white_castle.kingside = false;
                        } else if self.white_castle.can_queenside_castle()
                            && piece_captured.pos.is_queenside_rook()
                        {
                            self.white_castle.queenside = false;
                        }
                    }
                }
            }

            self.set_at(to, piece);
            self.en_passant = None
        }
    }

    pub fn invert_turn(mut self) -> Self {
        self.turn = !self.turn;
        self
    }

    pub fn evaluate(&self, color: Color) -> i32 {
        self.get_all_pieces()
            .iter()
            .map(|piece| {
                if piece.color == color {
                    piece.evaluate()
                } else {
                    -piece.evaluate()
                }
            })
            .sum()
    }

    pub fn get_best_move(&self, depth: i32) -> Option<Move> {
        let num = AtomicI32::new(0);
        let test = self
            .gen_legal_moves()
            .par_iter()
            .map(|&legal_move| {
                let eval = !negamax(
                    self.set_move(legal_move).invert_turn(),
                    i32::MIN,
                    i32::MAX,
                    depth,
                    !self.turn,
                    &num,
                );
                (eval, legal_move)
            })
            .max_by(|(fst_eval, _), (snd_eval, _)| fst_eval.cmp(snd_eval))
            .map(|(_, best_move)| best_move);
        //println!("{}", num.load(Relaxed));
        test
    }

    pub fn get_best_move_uci(&self, depth: i32) -> Option<String> {
        let mov = self.get_best_move(depth);
        match mov {
            Some(Move { from, to, prom }) => Some(format!(
                "{}{}{}",
                Into::<String>::into(from),
                Into::<String>::into(to),
                match prom {
                    Some(prom) => {
                        match prom {
                            Knight => "k",
                            Bishop => "b",
                            Rook => "r",
                            Queen => "q",
                            _ => panic!("you can promote to a king"),
                        }
                    }
                    None => "",
                }
            )),
            None => None,
        }
    }
}

impl Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("8");
        for (i, square) in self.board.iter().enumerate() {
            if i != 0 && i % 8 == 0 {
                print!("\n{}", 8 - i / 8)
            };

            if let Some(piece) = square {
                write!(f, "{}", piece)?
            } else {
                print!(" ")
            }
        }
        print!("\n ABCDEFGH");

        Ok(())
    }
}
