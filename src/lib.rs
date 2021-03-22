mod board;
mod fen;
mod piece;
mod position;

use std::{
    cmp::max,
};

pub use board::Chess;
pub use board::Color;
pub use piece::{Piece, PieceType};
pub use position::*;

fn negamax(chess: Chess, mut alpha: i32, beta: i32, depth: i32, turn: Color) -> i32 {
    if depth == 0 {
        return chess.evaluate(turn);
    }

    let legal_moves = chess.gen_legal_moves();
    if legal_moves.is_empty() {
        if !chess.is_check() {
            return 0;
        } else {
            return (i32::MIN / 2) - depth;
        }
    }

    let ordered_moves = chess.sort_moves(legal_moves);

    for (_, legal_move) in ordered_moves {
        let eval = !negamax(
            chess.set_move(legal_move).invert_turn(),
            !beta,
            !alpha,
            depth - 1,
            !turn,
        );

        if eval >= beta {
            return beta;
        }

        alpha = max(eval, alpha);
    }
    alpha
}
