use std::{convert::TryInto, fs::File, io::{Write, stdin}};

use anyhow::Result;
use chess_engine2::{
    Chess,
    Color::{Black, White},
};

struct UCI {
    chess: Chess,
}

impl UCI {
    fn new() -> Self {
        UCI {
            chess: Chess::new(),
        }
    }

    fn run(&mut self, file: &mut File) -> Result<()> {
        println!("id name basic chess engine");
        println!("id author Federico Gaggero");
        println!("uciok");
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            file.write(input.as_bytes())?;
            let tokens: Vec<&str> = input.split_ascii_whitespace().collect();
            match tokens[0] {
                "quit" => break,
                "stop" => (),
                "isready" => println!("readyok"),
                "ucinewgame" => self.ucinewgame(),
                "position" => self.position(&tokens, file)?,
                "go" => self.go(&tokens)?,
                _ => continue,
            }
        }
        Ok(())
    }

    fn ucinewgame(&mut self) {
        self.chess = Chess::new();
    }

    fn position(&mut self, tokens: &[&str], file: &mut File) -> Result<()> {
        let mut is_fen = false;
        let mut fen_tokens = Vec::new();
        let mut moves = Vec::new();
        for &token in &tokens[1..] {
            match token {
                "startpos" => {
                    fen_tokens.push("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                }
                "fen" => is_fen = true,
                "moves" => is_fen = false,
                _ => {
                    if is_fen {
                        fen_tokens.push(token)
                    } else {
                        moves.push(token)
                    }
                }
            }
        }

        if !fen_tokens.is_empty() {
            self.chess = Chess::from_fen(&fen_tokens.join(" "))?;
        }

        for r#move in moves {
            file.write(r#move.as_bytes())?;
            self.chess.set(r#move.try_into()?)
        }

        Ok(())
    }

    fn go(&mut self, tokens: &[&str]) -> Result<()> {
        let mut time = u64::MAX;
        let mut is_movetime = false;
        let side = self.chess.turn;

        for &token in &tokens[1..] {
            match token {
                "wtime" if side == White => is_movetime = true,
                "btime" if side == Black => is_movetime = true,
                "movetime" => is_movetime = true,
                _ => {
                    if is_movetime {
                        time = token.parse()?;
                        is_movetime = false
                    }
                }
            }
        }

        eprint!("time: {}ms", time);

        if let Some(r#move) = self.chess.get_best_move_uci(4) {
            println!("bestmove {}", r#move)
        } else {
            println!("bestmove 0000")
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let mut file = File::create("/home/gaggix/Documenti/Rust/chess_engine2/lichess/lichess-bot/engines/log")?;
    let mut uci = UCI::new();
    match uci.run(&mut file) {
        Ok(_) => (),
        Err(e) => {file.write(e.to_string().as_bytes())?;}
    };

    Ok(())
}
