mod board;
mod fen;
mod piece;
mod position;

use std::{
    cmp::max,
    sync::atomic::{AtomicI32, Ordering::Relaxed},
};

pub use board::Chess;
pub use board::Color;
pub use piece::{Piece, PieceType};
pub use position::*;

fn negamax(
    chess: Chess,
    mut alpha: i32,
    beta: i32,
    depth: i32,
    turn: Color,
    num: &AtomicI32,
) -> i32 {
    num.fetch_add(1, Relaxed);
    if depth == 0 {
        return chess.evaluate(turn);
    }

    let legal_moves = chess.gen_legal_moves();
    if legal_moves.is_empty() {
        if !chess.is_check() {
            return 0;
        } else {
            return i32::MIN;
        }
    }

    let mut ordered_moves = legal_moves
        .iter()
        .map(|&r#move| {
            (
                {
                    let Move { from, to, prom } = r#move;
                    let mut value = 0;
                    let aggressor = chess.board[usize::from(from)].unwrap();

                    if let Some(victim) = chess.board[usize::from(to)] {
                        value += 10 * victim.r#type.evaluate_material()
                            - aggressor.r#type.evaluate_material()
                    }

                    if let Some(prom_piece) = prom {
                        value += prom_piece.evaluate_material()
                    }

                    value
                },
                r#move,
            )
        })
        .collect::<Vec<(i32, Move)>>();

    ordered_moves.sort_unstable_by(|(a, _), (b, _)| b.cmp(a));

    for (_, legal_move) in ordered_moves {
        let eval = !negamax(
            chess.set_move(legal_move).invert_turn(),
            !beta,
            !alpha,
            depth - 1,
            !turn,
            num,
        );
        if eval >= beta {
            return beta;
        }

        alpha = max(eval, alpha);
    }
    alpha
}
