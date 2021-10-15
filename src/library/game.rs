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

    fn new(move_string: &mut str, game: &Game) -> Move {
        
        // transform to vector of characters and strip newlines or carriage returns
        let mut move_chars: Vec<char> = move_string.chars().rev().filter(
            |c| *c != '\n' && *c != '\r'
        ).collect();

        // (1) the last two chars are interpreted as the target field
        let target_field = Move::get_target_field(&mut move_chars);
        
        // two characters at the most can be left,
        // otherwise the move is not valid!
        assert!(move_chars.len() <= 2);
        
        // (2) get the piece that was moved (incl. descriminator) 
        let (piece, specifier) = Move::get_piece(&mut move_chars, &game.turn);

        let start_field = Move::get_start_field(
            specifier,
            &piece,
            &target_field,
            game
        );
        
        Move { 
            piece: piece, 
            start_field: start_field,
            target_field: target_field 
        }
    }

    fn get_target_field(move_chars: &mut Vec<char>) -> Field {
        
        let file = match move_chars.remove(0).to_digit(10) {
            Some(rank) =>  rank as usize,
            None => panic!("
            Invalid move!")
        };

        let rank= match move_chars.remove(0) {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            _ => panic!("Invalid move!")
        };

        Field(rank, file) 
    }

    fn get_start_field(
        specifier: Option<char>, 
        piece: &Piece, 
        target_field: &Field,
        game: &Game, 
    ) -> Field {
        
        let field: Field; 
        
        // (1) get all remaining instances of >piece<
        let instances = Move::get_piece_instances(piece, game);

        // (2) check which instances can potentially reach >target_field<
        let valid_instances = instances.iter().filter(|piece| {
            Move::can_reach(piece, target_field, game.position_matrix().as_ref().unwrap())
        }).collect::<Vec<_>>();

        // (3) if multiple, check whether the specifier removes ambiguity
        match valid_instances.len() {
            0 => panic!("No movable piece could be identified - invalid move?!"),
            1 => *valid_instances[0].field(),
            _ => Move::determine_field_by_specifier(valid_instances, specifier)
        }
    }

    fn get_piece_instances(piece: &Piece, game: &Game) -> Vec<PieceInstance>
    {
        let mut instances: Vec<PieceInstance> = Vec::new();
        let matrix = game.position_matrix().as_ref().unwrap();
        let mut rank_index: usize = 0;
        for rank in matrix {
            rank_index += 1;
            for file_index  in (0..7) {
                if rank[file_index] == *piece {
                    instances.push(PieceInstance{ 
                        piece: *piece,
                        field: Field(rank_index, file_index)
                    })
                }               
            }
        }
        instances
    }

    fn get_piece(move_chars: &mut Vec<char>, turn: &Color) -> (Piece, Option<char>) {
        
        // if no chars are left, the piece has to be a pawn
        if move_chars.len() == 0 {
            return (Piece{color: *turn, piecetype: PieceType::Pawn}, None)
        } 

        // first, pop the specifier off the array
        let mut specifier   = None;
        if move_chars.len() == 2 {
            specifier = Some(move_chars.remove(0));
        }

        // otherwise we expect the piece to be contained
        // in the remaining move chars
        match move_chars.remove(0) {
            'p' => return(Piece{color: *turn, piecetype: PieceType::Pawn}, specifier),
            'r' => return(Piece{color: *turn, piecetype: PieceType::Rook}, specifier),
            'n' => return(Piece{color: *turn, piecetype: PieceType::Knight}, specifier),
            'b' => return(Piece{color: *turn, piecetype: PieceType::Bishop}, specifier),
            'q' => return(Piece{color: *turn, piecetype: PieceType::Queen}, specifier),
            'k' => return(Piece{color: *turn, piecetype: PieceType::King}, specifier),
            _ => panic!("Invalid move!")  
        }
    }

    fn can_reach(
        pieceInstance: &PieceInstance, 
        target_field: &Field,
        position_matrix: &Vec<Vec<Piece>>
    ) -> bool {
        let &piece = pieceInstance.piece();
        match piece.piecetype() {
            PieceType::Rook    => Move::rook_can_reach(piece.color(), target_field, position_matrix),
            PieceType::Knight  => Move::knight_can_reach(piece.color(), target_field, position_matrix),
            PieceType::Bishop  => Move::bishop_can_reach(piece.color(), target_field, position_matrix),
            PieceType::Queen   => Move::queen_can_reach(piece.color(), target_field, position_matrix),
            PieceType::King    => Move::king_can_reach(piece.color(), target_field, position_matrix),
            PieceType::Pawn    => Move::pawn_can_reach(piece.color(), target_field, position_matrix),
            PieceType::None    => false
        }
    }

    fn determine_field_by_specifier(valid_instances: Vec<&PieceInstance>, specifier: Option<char>) -> Field {
        
        let field: Field;

        let specifier = match specifier {
            Some(char) => char,
            None => panic!("No specifier was provided, move ambiguous!")
        };


        field
    }

    fn rook_can_reach(color: &Color, target_field: &Field, position_matrix: &Vec<Vec<Piece>>) -> bool {true}
    fn knight_can_reach(color: &Color, target_field: &Field, position_matrix: &Vec<Vec<Piece>>) -> bool {true}
    fn bishop_can_reach(color: &Color, target_field: &Field, position_matrix: &Vec<Vec<Piece>>) -> bool {true}
    fn queen_can_reach(color: &Color, target_field: &Field, position_matrix: &Vec<Vec<Piece>>) -> bool {true} 
    fn king_can_reach(color: &Color, target_field: &Field, position_matrix: &Vec<Vec<Piece>>) -> bool {true}
    fn pawn_can_reach(color: &Color, target_field: &Field, position_matrix: &Vec<Vec<Piece>>) -> bool {true}
     
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
pub struct Game<'a> {
    position: Position<'a>,
    position_matrix: Option<Vec<Vec<Piece>>>,
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

    pub fn position_matrix(&self) -> &Option<Vec<Vec<Piece>>> {
        match self.position_matrix {
            Some(_) => &self.position_matrix,
            None => panic!("position matrix should have been buffered!")
        }
    }

    fn buffer_matrix(&mut self) -> &Option<Vec<Vec<Piece>>> {
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
        self.position_matrix = Some(matrix);
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
        let mut move_string = String::new();
        io::stdin().read_line(&mut move_string).unwrap();

        self.buffer_matrix();
        let chess_move = Move::new(&mut move_string, &self);
        self.make_move(chess_move);
    }

    fn make_move(&mut self, chess_move: Move) {
        println!("About to move the {} from {} to {}.", chess_move.piece, chess_move.start_field, chess_move.target_field);
    }
}
