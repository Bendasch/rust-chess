use crate::library::game::*;
use std::fmt;

const BLACK_ROOK: char    = '\u{265C}';
const BLACK_KNIGHT: char  = '\u{265E}';
const BLACK_BISHOP: char  = '\u{265D}';
const BLACK_QUEEN: char   = '\u{265B}';
const BLACK_KING: char    = '\u{265A}';
const BLACK_PAWN: char    = '\u{265F}';
const WHITE_ROOK: char    = '\u{2656}';
const WHITE_KNIGHT: char  = '\u{2658}';
const WHITE_BISHOP: char  = '\u{2657}';
const WHITE_QUEEN: char   = '\u{2655}';
const WHITE_KING: char    = '\u{2654}';
const WHITE_PAWN: char    = '\u{2659}';
//const EMPTY: char         = '\u{0020}';

pub fn run() {
    let mut game = Game::new();
    draw_board(&game.position());
    draw_who_to_move(&game.turn());
    game.next_move();
}

fn draw_board(position: &str) {
    let split = position.split("/");
    for line in split {
        let mut output_row = String::new();
        for char in line.chars() {
            match char {
                'r' => output_row.push(BLACK_ROOK),
                'n' => output_row.push(BLACK_KNIGHT),
                'b' => output_row.push(BLACK_BISHOP),
                'q' => output_row.push(BLACK_QUEEN),
                'k' => output_row.push(BLACK_KING),
                'p' => output_row.push(BLACK_PAWN),
                'R' => output_row.push(WHITE_ROOK),
                'N' => output_row.push(WHITE_KNIGHT),
                'B' => output_row.push(WHITE_BISHOP),
                'Q' => output_row.push(WHITE_QUEEN),
                'K' => output_row.push(WHITE_KING),
                'P' => output_row.push(WHITE_PAWN),
                '1' => output_row.push_str(" "),
                '2' => output_row.push_str("  "),
                '3' => output_row.push_str("   "),
                '4' => output_row.push_str("    "),
                '5' => output_row.push_str("     "),
                '6' => output_row.push_str("      "),
                '7' => output_row.push_str("       "),
                '8' => output_row.push_str("        "),
                _ => panic!("Board position is corrupt!")
            }
            output_row.push(' ');
        }
        println!("{}", output_row);
    }
}

fn draw_who_to_move(turn: &Turn) {
    match turn {
        Turn::Black => println!("Black to move..."),
        Turn::White => println!("White to move..."),
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}


impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Piece::BlackRook    => write!(f, "black rook"),
            Piece::BlackKnight  => write!(f, "black knight"),
            Piece::BlackBishop  => write!(f, "black bishop"),
            Piece::BlackQueen   => write!(f, "black queen"), 
            Piece::BlackKing    => write!(f, "black king"),
            Piece::BlackPawn    => write!(f, "black pawn"),
            Piece::WhiteRook    => write!(f, "white rook"),
            Piece::WhiteKnight  => write!(f, "white knight"),
            Piece::WhiteBishop  => write!(f, "white bishop"),
            Piece::WhiteQueen   => write!(f, "white queen"), 
            Piece::WhiteKing    => write!(f, "white king"),
            Piece::WhitePawn    => write!(f, "white pawn")
        }
    }
}