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

    fn new(move_string: String) -> Move {
        assert!(Move::is_valid_move_string(&move_string));
        let move_chars: Vec<char> = move_string.chars().rev().collect();
        let row = match move_chars[0].to_digit(10) {
            Some(row) =>  row,
            None => panic!("Invalid move!")
        };
        let field = Field(move_chars[1], row as u8);

        Move { 
            piece: Piece::WhitePawn, 
            start_field: Field('e', 2),
            target_field: field 
        }
    }

    fn is_valid_move_string(move_string: &str) -> bool {
        move_string.len() < 4
        // actual logic require later
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
        move_string = move_string.trim().to_string();

        let chess_move = Move::new(move_string);
        self.make_move(chess_move);
    }

    fn is_legal_move(&self, _chess_move: Move) -> bool {
        // to do
        true
    }

    fn make_move(&mut self, chess_move: Move) {
        println!("About to move the {} from {} to {}.", chess_move.piece, chess_move.start_field, chess_move.target_field);
    }
}
