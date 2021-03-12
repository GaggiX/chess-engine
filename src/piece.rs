use std::fmt::Display;

use crate::position::{Move, Position};
use crate::{
    board::{
        Color::{self, *},
        Square,
    },
    Chess,
};

const PAWN_SQUARE_TABLE_WHITE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];

const PAWN_SQUARE_TABLE_BLACK: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0, 0,
    20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50, 50,
    50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0,
];

const KNIGHT_SQUARE_TABLE_WHITE: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

const KNIGHT_SQUARE_TABLE_BLACK: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 5, 5, 0, -20, -40, -30, 5, 10, 15, 15, 10,
    5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 10, 15, 15, 10,
    0, -30, -40, -20, 0, 0, 0, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

const BISHOP_SQUARE_TABLE_WHITE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

const BISHOP_SQUARE_TABLE_BLACK: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 5, 0, 0, 0, 0, 5, -10, -10, 10, 10, 10, 10, 10,
    10, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 0, 0, 0, 0, 0, 0, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

const ROOK_SQUARE_TABLE_WHITE: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];

const ROOK_SQUARE_TABLE_BLACK: [i32; 64] = [
    0, 0, 0, 5, 5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0,
    0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10, 10, 10, 5, 0, 0,
    0, 0, 0, 0, 0, 0,
];

const QUEEN_SQUARE_TABLE_WHITE: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

const QUEEN_SQUARE_TABLE_BLACK: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 5, 0, -10, -10, 0, 5, 5, 5, 5, 5, -10,
    -5, 0, 5, 5, 5, 5, 0, 0, -5, 0, 5, 5, 5, 5, 0, -5, -10, 0, 5, 5, 5, 5, 0, -10, -10, 0, 0, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

const KING_SQUARE_TABLE_WHITE: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

const KING_SQUARE_TABLE_BLACK: [i32; 64] = [
    20, 30, 10, 0, 0, 10, 30, 20, 20, 20, 0, 0, 0, 0, 20, 20, -10, -20, -20, -20, -20, -20, -20,
    -10, -20, -30, -30, -40, -40, -30, -30, -20, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
    -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50,
    -40, -40, -30,
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
pub use PieceType::*;

impl PieceType {
    pub fn evaluate_material(&self) -> i32 {
        match self {
            King => 20000,
            Queen => 900,
            Rook => 500,
            Bishop => 330,
            Knight => 320,
            Pawn => 100,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub r#type: PieceType,
    pub pos: Position,
    pub color: Color,
}

impl Piece {
    pub fn new(r#type: PieceType, pos: Position, color: Color) -> Self {
        Self { r#type, pos, color }
    }

    pub fn gen_moves(&self, chess: &Chess) -> Vec<Move> {
        let mut moves = Vec::new();

        match self.r#type {
            King => {
                let king_poses = [
                    self.pos.up(1),
                    self.pos.down(1),
                    self.pos.left(1),
                    self.pos.right(1),
                    self.pos.up(1).right(1),
                    self.pos.down(1).right(1),
                    self.pos.down(1).left(1),
                    self.pos.up(1).left(1),
                ];

                for &king_pos in king_poses.iter() {
                    if king_pos.is_on_board()
                        && !king_pos.is_there_a_piece_color(&chess.board, self.color)
                    {
                        moves.push(Move {
                            from: self.pos,
                            to: king_pos,
                            prom: None,
                        })
                    }
                }

                let castle_right = if chess.turn == White {
                    chess.white_castle
                } else {
                    chess.black_castle
                };

                if castle_right.can_castle() {
                    if castle_right.can_kingside_castle()
                        && !self.pos.right(1).is_there_a_piece(&chess.board)
                        && !self.pos.right(2).is_there_a_piece(&chess.board)
                        && !self.am_i_being_attacked(&chess.board)
                        && !self.piece_right(1).am_i_being_attacked(&chess.board)
                    {
                        moves.push(Move {
                            from: self.pos,
                            to: self.pos.right(2),
                            prom: None,
                        })
                    }
                    if castle_right.can_queenside_castle()
                        && !self.pos.left(1).is_there_a_piece(&chess.board)
                        && !self.pos.left(2).is_there_a_piece(&chess.board)
                        && !self.pos.left(3).is_there_a_piece(&chess.board)
                        && !self.am_i_being_attacked(&chess.board)
                        && !self.piece_left(1).am_i_being_attacked(&chess.board)
                    {
                        moves.push(Move {
                            from: self.pos,
                            to: self.pos.left(2),
                            prom: None,
                        })
                    }
                }
            }
            Queen => {
                let queen_moves = [
                    |pos: Position| pos.up(1),
                    |pos: Position| pos.down(1),
                    |pos: Position| pos.left(1),
                    |pos: Position| pos.right(1),
                    |pos: Position| pos.up(1).right(1),
                    |pos: Position| pos.down(1).right(1),
                    |pos: Position| pos.down(1).left(1),
                    |pos: Position| pos.up(1).left(1),
                ];

                for &queen_move in queen_moves.iter() {
                    let mut new_pos = queen_move(self.pos);

                    while new_pos.is_on_board()
                        && !new_pos.is_there_a_piece_color(&chess.board, self.color)
                    {
                        moves.push(Move {
                            from: self.pos,
                            to: new_pos,
                            prom: None,
                        });

                        if new_pos.is_there_a_piece_color(&chess.board, !self.color) {
                            break;
                        }

                        new_pos = queen_move(new_pos);
                    }
                }
            }
            Rook => {
                let rook_moves = [
                    |pos: Position| pos.up(1),
                    |pos: Position| pos.down(1),
                    |pos: Position| pos.left(1),
                    |pos: Position| pos.right(1),
                ];

                for &rook_move in rook_moves.iter() {
                    let mut new_pos = rook_move(self.pos);

                    while new_pos.is_on_board()
                        && !new_pos.is_there_a_piece_color(&chess.board, self.color)
                    {
                        moves.push(Move {
                            from: self.pos,
                            to: new_pos,
                            prom: None,
                        });

                        if new_pos.is_there_a_piece_color(&chess.board, !self.color) {
                            break;
                        }

                        new_pos = rook_move(new_pos);
                    }
                }
            }
            Bishop => {
                let bishop_moves = [
                    |pos: Position| pos.up(1).right(1),
                    |pos: Position| pos.down(1).right(1),
                    |pos: Position| pos.down(1).left(1),
                    |pos: Position| pos.up(1).left(1),
                ];

                for &bishop_move in bishop_moves.iter() {
                    let mut new_pos = bishop_move(self.pos);

                    while new_pos.is_on_board()
                        && !new_pos.is_there_a_piece_color(&chess.board, self.color)
                    {
                        moves.push(Move {
                            from: self.pos,
                            to: new_pos,
                            prom: None,
                        });

                        if new_pos.is_there_a_piece_color(&chess.board, !self.color) {
                            break;
                        }

                        new_pos = bishop_move(new_pos);
                    }
                }
            }
            Knight => {
                let knight_poses = [
                    self.pos.up(2).left(1),
                    self.pos.up(2).right(1),
                    self.pos.right(2).up(1),
                    self.pos.right(2).down(1),
                    self.pos.down(2).left(1),
                    self.pos.down(2).right(1),
                    self.pos.left(2).up(1),
                    self.pos.left(2).down(1),
                ];

                for &knight_pos in knight_poses.iter() {
                    if knight_pos.is_on_board()
                        && !knight_pos.is_there_a_piece_color(&chess.board, self.color)
                    {
                        moves.push(Move {
                            from: self.pos,
                            to: knight_pos,
                            prom: None,
                        })
                    }
                }
            }
            Pawn => {
                let pawn_up_one = self.pos.up_color(self.color, 1);
                let pawn_up_two = self.pos.up_color(self.color, 2);
                let pawn_right = pawn_up_one.right(1);
                let pawn_left = pawn_up_one.left(1);

                if pawn_up_one.is_on_board() && !pawn_up_one.is_there_a_piece(&chess.board) {
                    if pawn_up_one.is_pawn_promotion_pos(self.color) {
                        moves.extend_from_slice(&[
                            Move {
                                from: self.pos,
                                to: pawn_up_one,
                                prom: Some(Queen),
                            },
                            Move {
                                from: self.pos,
                                to: pawn_up_one,
                                prom: Some(Rook),
                            },
                            Move {
                                from: self.pos,
                                to: pawn_up_one,
                                prom: Some(Bishop),
                            },
                            Move {
                                from: self.pos,
                                to: pawn_up_one,
                                prom: Some(Knight),
                            },
                        ])
                    } else {
                        moves.push(Move {
                            from: self.pos,
                            to: pawn_up_one,
                            prom: None,
                        });

                        if pawn_up_two.is_on_board()
                            && !pawn_up_two.is_there_a_piece(&chess.board)
                            && self.pos.is_pawn_starting_pos(self.color)
                        {
                            moves.push(Move {
                                from: self.pos,
                                to: pawn_up_two,
                                prom: None,
                            });
                        }
                    }
                }

                if pawn_right.is_on_board()
                    && (pawn_right.is_there_a_piece_color(&chess.board, !self.color)
                        || pawn_right.can_en_passant(chess.en_passant))
                {
                    moves.push(Move {
                        from: self.pos,
                        to: pawn_right,
                        prom: None,
                    });
                }

                if pawn_left.is_on_board()
                    && (pawn_left.is_there_a_piece_color(&chess.board, !self.color)
                        || pawn_left.can_en_passant(chess.en_passant))
                {
                    moves.push(Move {
                        from: self.pos,
                        to: pawn_left,
                        prom: None,
                    });
                }
            }
        }

        moves
    }

    pub fn am_i_being_attacked(&self, board: &[Square; 64]) -> bool {
        let pawn_left = self.pos.up_color(self.color, 1).left(1);
        let pawn_right = self.pos.up_color(self.color, 1).right(1);

        let knight_poses = [
            self.pos.up(2).left(1),
            self.pos.up(2).right(1),
            self.pos.right(2).up(1),
            self.pos.right(2).down(1),
            self.pos.down(2).left(1),
            self.pos.down(2).right(1),
            self.pos.left(2).up(1),
            self.pos.left(2).down(1),
        ];

        let king_poses = [
            self.pos.up(1).right(1),
            self.pos.down(1).right(1),
            self.pos.down(1).left(1),
            self.pos.up(1).left(1),
            self.pos.up(1),
            self.pos.down(1),
            self.pos.left(1),
            self.pos.right(1),
        ];

        let bishop_moves = [
            |pos: Position| pos.up(1).right(1),
            |pos: Position| pos.down(1).right(1),
            |pos: Position| pos.down(1).left(1),
            |pos: Position| pos.up(1).left(1),
        ];

        let rook_moves = [
            |pos: Position| pos.up(1),
            |pos: Position| pos.down(1),
            |pos: Position| pos.left(1),
            |pos: Position| pos.right(1),
        ];

        if pawn_left.is_on_board()
            && pawn_left.is_there_a_piece_type_color(board, Pawn, !self.color)
        {
            return true;
        }

        if pawn_right.is_on_board()
            && pawn_right.is_there_a_piece_type_color(board, Pawn, !self.color)
        {
            return true;
        }

        for &knight_pos in knight_poses.iter() {
            if knight_pos.is_on_board()
                && knight_pos.is_there_a_piece_type_color(board, Knight, !self.color)
            {
                return true;
            }
        }

        for &king_pos in king_poses.iter() {
            if king_pos.is_on_board()
                && king_pos.is_there_a_piece_type_color(board, King, !self.color)
            {
                return true;
            }
        }

        for &bishop_move in bishop_moves.iter() {
            let mut new_pos = bishop_move(self.pos);

            while new_pos.is_on_board() && !new_pos.is_there_a_piece_color(board, self.color) {
                if new_pos.is_there_a_piece_type_color(board, Bishop, !self.color)
                    || new_pos.is_there_a_piece_type_color(board, Queen, !self.color)
                {
                    return true;
                } else if new_pos.is_there_a_piece_color(board, !self.color) {
                    break;
                } else {
                    new_pos = bishop_move(new_pos);
                }
            }
        }

        for &rook_move in rook_moves.iter() {
            let mut new_pos = rook_move(self.pos);

            while new_pos.is_on_board() && !new_pos.is_there_a_piece_color(board, self.color) {
                if new_pos.is_there_a_piece_type_color(board, Rook, !self.color)
                    || new_pos.is_there_a_piece_type_color(board, Queen, !self.color)
                {
                    return true;
                } else if new_pos.is_there_a_piece_color(board, !self.color) {
                    break;
                } else {
                    new_pos = rook_move(new_pos);
                }
            }
        }

        false
    }

    pub fn evaluate(&self) -> i32 {
        let material_value = self.r#type.evaluate_material();

        let square_table = match (self.r#type, self.color) {
            (King, White) => KING_SQUARE_TABLE_WHITE,
            (King, Black) => KING_SQUARE_TABLE_BLACK,
            (Queen, White) => QUEEN_SQUARE_TABLE_WHITE,
            (Queen, Black) => QUEEN_SQUARE_TABLE_BLACK,
            (Rook, White) => ROOK_SQUARE_TABLE_WHITE,
            (Rook, Black) => ROOK_SQUARE_TABLE_BLACK,
            (Bishop, White) => BISHOP_SQUARE_TABLE_WHITE,
            (Bishop, Black) => BISHOP_SQUARE_TABLE_BLACK,
            (Knight, White) => KNIGHT_SQUARE_TABLE_WHITE,
            (Knight, Black) => KNIGHT_SQUARE_TABLE_BLACK,
            (Pawn, White) => PAWN_SQUARE_TABLE_WHITE,
            (Pawn, Black) => PAWN_SQUARE_TABLE_BLACK,
        };

        material_value + square_table[usize::from(self.pos)]
    }

    fn piece_right(mut self, num: i32) -> Self {
        self.pos = self.pos.right(num);
        self
    }

    fn piece_left(mut self, num: i32) -> Self {
        self.pos = self.pos.left(num);
        self
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.r#type, self.color) {
            (King, Color::White) => write!(f, "♔"),
            (King, Color::Black) => write!(f, "♚"),
            (Queen, Color::White) => write!(f, "♕"),
            (Queen, Color::Black) => write!(f, "♛"),
            (Rook, Color::White) => write!(f, "♖"),
            (Rook, Color::Black) => write!(f, "♜"),
            (Bishop, Color::White) => write!(f, "♗"),
            (Bishop, Color::Black) => write!(f, "♝"),
            (Knight, Color::White) => write!(f, "♘"),
            (Knight, Color::Black) => write!(f, "♞"),
            (Pawn, Color::White) => write!(f, "♙"),
            (Pawn, Color::Black) => write!(f, "♟︎"),
        }
    }
}
