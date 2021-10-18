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
    draw_board(game.position());
    draw_who_to_move(&game.turn());
    loop {
        game.next_move();
    }
}

fn draw_board(position: Position) {
    let split = position.split();
    for line in split {
        let mut output_rank = String::new();
        for char in line.chars() {
            match char {
                'r' => output_rank.push(BLACK_ROOK),
                'n' => output_rank.push(BLACK_KNIGHT),
                'b' => output_rank.push(BLACK_BISHOP),
                'q' => output_rank.push(BLACK_QUEEN),
                'k' => output_rank.push(BLACK_KING),
                'p' => output_rank.push(BLACK_PAWN),
                'R' => output_rank.push(WHITE_ROOK),
                'N' => output_rank.push(WHITE_KNIGHT),
                'B' => output_rank.push(WHITE_BISHOP),
                'Q' => output_rank.push(WHITE_QUEEN),
                'K' => output_rank.push(WHITE_KING),
                'P' => output_rank.push(WHITE_PAWN),
                '1' => output_rank.push_str(" "),
                '2' => output_rank.push_str("  "),
                '3' => output_rank.push_str("   "),
                '4' => output_rank.push_str("    "),
                '5' => output_rank.push_str("     "),
                '6' => output_rank.push_str("      "),
                '7' => output_rank.push_str("       "),
                '8' => output_rank.push_str("        "),
                _ => panic!("Board position is corrupt!")
            }
            output_rank.push(' ');
        }
        println!("{}", output_rank);
    }
}

fn draw_who_to_move(turn: &Color) {
    match *turn {
        Color::Black => println!("Black to move..."),
        Color::White => println!("White to move..."),
        _ => return
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let color= match self.color() {
            Color::Black => "black",
            Color::White => "white",
            Color::None => panic!("Corrupt game state - each player needs a color!")
        };

        match self.piecetype() {
            PieceType::Rook    => write!(f, "{} rook", color),
            PieceType::Knight  => write!(f, "{} knight", color),
            PieceType::Bishop  => write!(f, "{} bishop", color),
            PieceType::Queen   => write!(f, "{} queen", color), 
            PieceType::King    => write!(f, "{} king", color),
            PieceType::Pawn    => write!(f, "{} pawn", color),
            PieceType::None    => panic!("A piece needs to be selected to move!"),   
        }
    }
}