use std::convert::TryInto;

use crate::{
    board::{CastleRight, Chess},
    Color::*,
    {Piece, PieceType::*}
};
use anyhow::{bail, Result};

pub fn parse(fen: &str) -> Result<Chess> {
    let mut chess = Chess {
        board: [None; 64],
        white_castle: CastleRight::new(),
        black_castle: CastleRight::new(),
        en_passant: None,
        turn: White,
    };

    let tokens: Vec<&str> = fen.split_ascii_whitespace().collect();

    if tokens.len() != 6 {
        bail!("Error parsing FEN: {}", fen)
    }

    for (i, rank) in tokens[0].split("/").enumerate() {
        let mut col = 0;
        for chr in rank.chars() {
            if let Some(num) = chr.to_digit(10) {
                col += num
            } else {
                let index = i * 8 + col as usize;
                chess.board[index] = Some(match chr {
                    'P' => {Piece::new(Pawn, index.into(), White)}
                    'N' => {Piece::new(Knight, index.into(), White)}
                    'B' => {Piece::new(Bishop, index.into(), White)}
                    'R' => {Piece::new(Rook, index.into(), White)}
                    'Q' => {Piece::new(Queen, index.into(), White)}
                    'K' => {Piece::new(King, index.into(), White)}
                    'p' => {Piece::new(Pawn, index.into(), Black)}
                    'n' => {Piece::new(Knight, index.into(), Black)}
                    'b' => {Piece::new(Bishop, index.into(), Black)}
                    'r' => {Piece::new(Rook, index.into(), Black)}
                    'q' => {Piece::new(Queen, index.into(), Black)}
                    'k' => {Piece::new(King, index.into(), Black)}
                    _ => bail!("found illegal char: {}", chr),
                });
                col += 1
            }
        }
    }

    chess.turn = match tokens[1].as_bytes()[0] as char {
        'w' => White,
        'b' => Black,
        chr @ _ => bail!("found illegal char: {}", chr)
    };

    if tokens[2].contains('K') {
        chess.white_castle.set_kingside_castle_on()
    }
    if tokens[2].contains('Q') {
        chess.white_castle.set_queenside_castle_on()
    }
    if tokens[2].contains('k') {
        chess.black_castle.set_kingside_castle_on()
    }
    if tokens[2].contains('q') {
        chess.black_castle.set_queenside_castle_on()
    }

    if let Ok(en_passant) = tokens[3].try_into() {
        chess.en_passant = Some(en_passant)
    } 

    Ok(chess)
}
