#![allow(dead_code)]
use std::io;
use std::cmp::PartialEq;
use std::cell::RefCell;

#[derive(PartialEq, Copy, Clone, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct Field(pub usize, pub usize);

pub struct PieceInstance {
    piece: Piece,
    field: Field
}
impl PieceInstance {
    pub fn piece(&self) -> &Piece { &self.piece }
    pub fn field(&self) -> &Field { &self.field }
}

#[derive(Debug)]
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
    // AA are the indices of the source field (i.e., 52) [offset by 1]
    // BB are the indices of the target field (i.e., 54) [offset by 1]   
    // -> 2545 is equivalent to "e4" in standard notation.
    fn new(move_string: &mut str, game: &mut Game) -> Move {
        
        // transform to vector of characters and strip newlines or carriage returns
        let move_indices: Vec<usize> = move_string.chars().filter(
            |c| *c != '\n' && *c != '\r'
        ).map(
            |c| c.to_digit(10).expect("Faulty move string") as usize
        ).collect();
        
        let start_field = Field(move_indices[0]-1, move_indices[1]-1);
        let piece = game.position_matrix().borrow().get_piece_from_field(&start_field);
        Move {
            piece: piece,
            start_field: start_field,
            target_field: Field(move_indices[2]-1, move_indices[3]-1)
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
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
#[derive(PartialEq, Copy, Clone, Debug)]
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
    pub fn split(&'a self) -> Vec<&'a str> {
        self.0.split("/").collect()
    }

    pub fn update_from_matrix(&mut self) {
        // TO DO!
    }
}

#[derive(Clone)]
pub struct PositionMatrix(Vec<Vec<Piece>>);

impl PositionMatrix {
    
    fn has_piece_on_field(&self, field: &Field) -> bool {
        *self.0[field.0][field.1].piecetype() != PieceType::None
    }

    fn get_piece_from_field(&self, field: &Field) -> Piece {
        self.0[field.0][field.1]
    }

    fn empty_field(&mut self, field: Field) {
        self.0[field.0][field.1] = Piece{color: Color::None, piecetype: PieceType::None};
    }

    fn place_piece(&mut self, piece: Piece, field: Field) {
        self.0[field.0][field.1] = piece;
    }
}

pub struct Game<'a> {
    position: Position<'a>,
    position_matrix:  RefCell<PositionMatrix>,
    turn: Color,
    castle_availability: CastleAvailability,
    en_passant: Option<Field>,
    halfmove_clock: u16,
    fullmove_clock: u16
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let start_position = Position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        Game {
            position: start_position,
            position_matrix: Game::init_matrix(&start_position),
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

    pub fn position_matrix(&self) -> &RefCell<PositionMatrix> {
        &self.position_matrix
    }

    fn init_matrix(start_position: &Position) -> RefCell<PositionMatrix> {
        let mut matrix: Vec<Vec<Piece>> = Vec::new();
        let ranks: Vec<&str> = start_position.split();
        for (i, rank) in ranks.iter().rev().enumerate() {
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
                    'P' => matrix[i].push(Piece{color: Color::White, piecetype: PieceType::Pawn}),
                    _   => panic!("Position is corrupt\n{:?}", rank.chars())
                };                
            }
            assert!(matrix[i].len() == 8, "Position matrix doesn't have 8 files\n{:?}", matrix[i]);
        }
        assert!(matrix.len() == 8, "Position matrix doesn't have 8 ranks");
        RefCell::new(PositionMatrix(matrix))
    }

    fn push_none(rank: &mut Vec<Piece>, num: usize) {
        match num {
            0 => return,
            _ => {  
                rank.push(Piece{color: Color::None, piecetype: PieceType::None}); 
                Game::push_none(rank, num-1)
            }
        }
    }

    pub fn turn(&self) -> &Color {
        &self.turn
    }
    
    pub fn next_move(&mut self) {
       
        // get the player input
        let mut move_string = String::new();
        io::stdin().read_line(&mut move_string).unwrap();

        // execute the players move
        let chess_move = Move::new(&mut move_string, self);
        self.execute_move(chess_move);

        // parse position string
        self.position.update_from_matrix();
    }

    fn execute_move(&mut self, chess_move: Move) {

        // 1) remove the piece from it's current field
        self.position_matrix().borrow_mut().empty_field(chess_move.start_field);
    
        // 2) replace the new field with the piece
        self.position_matrix().borrow_mut().place_piece(chess_move.piece, chess_move.target_field);
    }

    fn piece_on_field(&self, field: &Field) -> bool {
        self.position_matrix().borrow().has_piece_on_field(field)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn new_game() {
        let game = Game::new();
        assert_eq!(game.position().0, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert!(game.position_matrix().borrow().0[0][0] == Piece{color: Color::White, piecetype: PieceType::Rook});
        assert!(game.position_matrix().borrow().0[0][1] == Piece{color: Color::White, piecetype: PieceType::Knight});
        assert!(game.position_matrix().borrow().0[0][2] == Piece{color: Color::White, piecetype: PieceType::Bishop});
        assert!(game.position_matrix().borrow().0[0][3] == Piece{color: Color::White, piecetype: PieceType::Queen});
        assert!(game.position_matrix().borrow().0[0][4] == Piece{color: Color::White, piecetype: PieceType::King});
        assert!(game.position_matrix().borrow().0[0][5] == Piece{color: Color::White, piecetype: PieceType::Bishop});
        assert!(game.position_matrix().borrow().0[0][6] == Piece{color: Color::White, piecetype: PieceType::Knight});
        assert!(game.position_matrix().borrow().0[0][7] == Piece{color: Color::White, piecetype: PieceType::Rook});
        assert!(game.position_matrix().borrow().0[1][0] == Piece{color: Color::White, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[1][1] == Piece{color: Color::White, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[1][2] == Piece{color: Color::White, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[1][3] == Piece{color: Color::White, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[1][4] == Piece{color: Color::White, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[1][5] == Piece{color: Color::White, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[1][6] == Piece{color: Color::White, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[1][7] == Piece{color: Color::White, piecetype: PieceType::Pawn});        
        assert!(game.position_matrix().borrow().0[6][0] == Piece{color: Color::Black, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[6][1] == Piece{color: Color::Black, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[6][2] == Piece{color: Color::Black, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[6][3] == Piece{color: Color::Black, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[6][4] == Piece{color: Color::Black, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[6][5] == Piece{color: Color::Black, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[6][6] == Piece{color: Color::Black, piecetype: PieceType::Pawn});
        assert!(game.position_matrix().borrow().0[6][7] == Piece{color: Color::Black, piecetype: PieceType::Pawn});        
        assert!(game.position_matrix().borrow().0[7][0] == Piece{color: Color::Black, piecetype: PieceType::Rook});
        assert!(game.position_matrix().borrow().0[7][1] == Piece{color: Color::Black, piecetype: PieceType::Knight});
        assert!(game.position_matrix().borrow().0[7][2] == Piece{color: Color::Black, piecetype: PieceType::Bishop});
        assert!(game.position_matrix().borrow().0[7][3] == Piece{color: Color::Black, piecetype: PieceType::Queen});
        assert!(game.position_matrix().borrow().0[7][4] == Piece{color: Color::Black, piecetype: PieceType::King});
        assert!(game.position_matrix().borrow().0[7][5] == Piece{color: Color::Black, piecetype: PieceType::Bishop});
        assert!(game.position_matrix().borrow().0[7][6] == Piece{color: Color::Black, piecetype: PieceType::Knight});
        assert!(game.position_matrix().borrow().0[7][7] == Piece{color: Color::Black, piecetype: PieceType::Rook});
    }

    #[test]
    fn matrix_empty_field() {
        let game = Game::new();
        assert!(game.piece_on_field(&Field(0,0)));
        game.position_matrix().borrow_mut().empty_field(Field(0,0));
        assert!(!game.piece_on_field(&Field(0,0)));
    }

    #[test]
    fn new_move() {
        let mut game = Game::new();
        let mut move_string = String::from("2545");
        let chess_move = Move::new(&mut move_string, &mut game);
        assert_eq!(chess_move.piece, Piece{color: Color::White, piecetype: PieceType::Pawn});
        assert_eq!(chess_move.start_field, Field(1,4));
        assert_eq!(chess_move.target_field, Field(3,4));

        let mut move_string = String::from("7363");
        let chess_move = Move::new(&mut move_string, &mut game);
        assert_eq!(chess_move.piece, Piece{color: Color::Black, piecetype: PieceType::Pawn});
        assert_eq!(chess_move.start_field, Field(6,2));
        assert_eq!(chess_move.target_field, Field(5,2));
    }

    #[test]
    fn matrix_execute_move() {

        let mut game = Game::new();
        let mut move_string = String::from("7363");
        let chess_move = Move::new(&mut move_string, &mut game);
        let mut move_string = String::from("7363");
        let chess_move_check = Move::new(&mut move_string, &mut game);

        game.execute_move(chess_move);
        assert_eq!(game.position_matrix().borrow().0[chess_move_check.target_field.0][chess_move_check.target_field.1], chess_move_check.piece);
    }
}