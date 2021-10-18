#![allow(dead_code)]
use std::io;
use std::cmp::PartialEq;
use std::str::Split;

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    White,
    None
}

pub struct CastleAvailability {
    white_king: bool,
    white_queen: bool,
    black_king: bool,
    black_queen: bool,
}

#[derive(PartialEq)]
pub struct Field(pub usize, pub usize);

pub struct PieceInstance {
    piece: Piece,
    field: Field
}
impl PieceInstance {
    pub fn piece(&self) -> &Piece { &self.piece }
    pub fn field(&self) -> &Field { &self.field }
}
pub struct Move {
    pub piece: Piece,
    pub start_field: Field,
    pub target_field: Field
}

impl Move {

    pub fn piece(&self) -> &Piece {
        &self.piece
    }

    pub fn start_field(&self) -> &Field {
        &self.start_field
    }

    pub fn target_field(&self) -> &Field {
        &self.target_field
    }

    // for now we assume the syntax "AABB", where
    // AA are the indices of the source field (i.e., 52)
    // BB are the indices of the target field (i.e., 54)
    // -> 5254 is equivalent to "e4" in standard notation.
    fn new(move_string: &mut str, game: &Game) -> Move {
        
        // transform to vector of characters and strip newlines or carriage returns
        let mut move_chars: Vec<char> = move_string.chars().filter(
            |c| *c != '\n' && *c != '\r'
        ).collect();

        // get the piece from the poisition matrix


        // 
        Move {
            piece: 
            start_field: move_chars[0..1]
            target_field: move_chars[2..3]
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct Piece {  
    color: Color,
    piecetype: PieceType,
}

impl Piece {
    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn piecetype(&self) -> &PieceType {
        &self.piecetype
    }
}
#[derive(PartialEq, Copy, Clone)]
pub enum PieceType {
    Rook,
    Knight,
    Bishop,
    Queen, 
    King,
    Pawn,
    None
}
#[derive(Clone,Copy)]
pub struct Position<'a>(&'a str);

impl<'a> Position<'a> {
    pub fn split(&'a self) -> Split<&'a str> {
        self.0.split("/") 
    }
}

pub struct PositionMatrix(Option<Vec<Vec<Piece>>>);

impl PositionMatrix {
    
    fn has_piece_on_field(&self, field: &Field) -> bool {
        match self.0 {
            Some(matrix) => *matrix[field.0][field.1].piecetype() != PieceType::None, 
            None => panic!("The matrix should always be prepared at this point!") 
        }
    }

    fn get_piece_from_field(&self, field: &Field) -> &Piece {
        match self.0 {
            Some(matrix) => &matrix[field.0][field.1], 
            None => panic!("The matrix should always be prepared at this point!") 
        }
    }
}

pub struct Game<'a> {
    position: Position<'a>,
    position_matrix: PositionMatrix,
    turn: Color,
    castle_availability: CastleAvailability,
    en_passant: Option<Field>,
    halfmove_clock: u16,
    fullmove_clock: u16
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            position: Position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            position_matrix: None,
            turn: Color::White,
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

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn position_matrix(&self) -> &PositionMatrix {
        match self.position_matrix {
            Some(_) => &self.position_matrix,
            None => panic!("position matrix should have been buffered!")
        }
    }

    fn buffer_matrix(&mut self) -> &PositionMatrix {
        let mut matrix: Vec<Vec<Piece>> = Vec::new();
        let ranks = self.position.split();
        for (i, rank) in ranks.enumerate() {
            matrix.push(Vec::new());
            for char in rank.chars() {
                match char {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' 
                        => Game::push_none(&mut matrix[i], char.to_digit(10).unwrap() as usize),
                    'r' => matrix[i].push(Piece{color: Color::Black, piecetype: PieceType::Rook}),
                    'n' => matrix[i].push(Piece{color: Color::Black, piecetype: PieceType::Knight}),
                    'b' => matrix[i].push(Piece{color: Color::Black, piecetype: PieceType::Bishop}),
                    'q' => matrix[i].push(Piece{color: Color::Black, piecetype: PieceType::Queen}),
                    'k' => matrix[i].push(Piece{color: Color::Black, piecetype: PieceType::King}),
                    'p' => matrix[i].push(Piece{color: Color::Black, piecetype: PieceType::Pawn}),
                    'R' => matrix[i].push(Piece{color: Color::White, piecetype: PieceType::Rook}),
                    'N' => matrix[i].push(Piece{color: Color::White, piecetype: PieceType::Knight}),
                    'B' => matrix[i].push(Piece{color: Color::White, piecetype: PieceType::Bishop}),
                    'Q' => matrix[i].push(Piece{color: Color::White, piecetype: PieceType::Queen}),
                    'K' => matrix[i].push(Piece{color: Color::White, piecetype: PieceType::King}),
                    _   => panic!("Position is corrupt")
                };                
            }
            assert!(matrix[i].len() == 8, "Position matrix doesn't have 8 files");
        }
        assert!(matrix.len() == 8, "Position matrix doesn't have 8 ranks");
        self.position_matrix = PositionMatrix(Some(matrix));
        &self.position_matrix
    }

    fn push_none(rank: &mut Vec<Piece>, num: usize) {
        match num {
            0 => return,
            1 => rank.push(Piece{color: Color::None, piecetype: PieceType::None}),
            _ => Game::push_none(rank, num-1)
        }
    }

    pub fn turn(&self) -> &Color {
        &self.turn
    }
    
    pub fn next_move(&mut self) {

        // transform the position string into a matrix 
        self.buffer_matrix();
        
        // get the player input
        let mut move_string = String::new();
        io::stdin().read_line(&mut move_string).unwrap();

        // execute the players move
        let chess_move = Move::new(&mut move_string, &self);
        self.execute_move(chess_move);
    }

    fn execute_move(&self, chess_move: Move) {

        // (1) destroy enemy piece on the target field
        // [note that we assume the move to be valid at this point]
        if self.piece_on_field(chess_move.target_field()) {
            println!("Hi, there is a piece on this field!")
        }

        // (2) place the piece on the field

    }

    fn piece_on_field(&self, field: &Field) -> bool {
        self.position_matrix().has_piece_on_field(field)
    }
}
