#![allow(dead_code)]
use std::io;

pub enum Turn {
    Black,
    White
}

pub struct CastleAvailability {
    white_king: bool,
    white_queen: bool,
    black_king: bool,
    black_queen: bool,
}

pub struct Field(pub char, pub u8);

pub struct Move {
    pub piece: Piece,
    pub start_field: Field,
    pub target_field: Field
}

impl Move {

    fn new(move_string: &mut str) -> Move {
        
        // transform to vector of characters and strip newlines or carriage returns
        let mut move_chars: Vec<char> = move_string.chars().rev().filter(
            |c| *c != '\n' && *c != '\r'
        ).collect();

        println!("{:?}", move_chars);
        let target_field = Move::get_target_field(&mut move_chars);
        println!("{:?}", move_chars); 

        Move { 
            piece: Piece::WhitePawn, 
            start_field: Field('e', 2),
            target_field: target_field 
        }
    }

    fn get_target_field(move_chars: &mut Vec<char>) -> Field {
        let row = match move_chars[0].to_digit(10) {
            Some(row) =>  row,
            None => panic!("Invalid move!")
        };
        Field(move_chars[1], row as u8)
    }

}
pub enum Piece {        
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen, 
    BlackKing,
    BlackPawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen, 
    WhiteKing,
    WhitePawn
}

pub struct Game<'a> {
    position: &'a str,
    turn: Turn,
    castle_availability: CastleAvailability,
    en_passant: Option<Field>,
    halfmove_clock: u16,
    fullmove_clock: u16
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            turn: Turn::White,
            castle_availability: CastleAvailability { 
                white_king: true, 
                white_queen: true, 
                black_king: true, 
                black_queen: true 
            },
            en_passant: None,
            halfmove_clock: 0,
            fullmove_clock: 1
        }
    }

    pub fn position(&self) -> &'a str {
        self.position
    }

    pub fn turn(&self) -> &Turn {
        &self.turn
    }
    
    pub fn next_move(&mut self) {
        let mut move_string = String::new();
        io::stdin().read_line(&mut move_string).unwrap();

        let chess_move = Move::new(&mut move_string);
        self.make_move(chess_move);
    }

    fn make_move(&mut self, chess_move: Move) {
        println!("About to move the {} from {} to {}.", chess_move.piece, chess_move.start_field, chess_move.target_field);
    }
}
