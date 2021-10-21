#![allow(dead_code)]
use std::io;
use std::cmp::PartialEq;
use std::cell::RefCell;
use std::cell::Ref;

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

    pub fn piece_string(&self) -> String {
        match self.piece.piecetype() {
            PieceType::Rook     => String::from("Rook"),
            PieceType::Knight   => String::from("Knight"),
            PieceType::Bishop   => String::from("Bishop"),
            PieceType::Queen    => String::from("Queen"), 
            PieceType::King     => String::from("King"),
            PieceType::Pawn     => String::from("Pawn"),
            PieceType::None     => String::from("Empty field")            
        }
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

        // assert that the moves are within the bounds of the field
        assert!(1 <= move_indices[0] && move_indices[0] <= 8);
        assert!(1 <= move_indices[1] && move_indices[1] <= 8);
        assert!(1 <= move_indices[2] && move_indices[2] <= 8);
        assert!(1 <= move_indices[3] && move_indices[3] <= 8);
        
        let start_field = Field(move_indices[0]-1, move_indices[1]-1);
        let target_field = Field(move_indices[2]-1, move_indices[3]-1);

        let piece = game.position_matrix().borrow().get_piece_from_field(&start_field);
        Move {
            piece: piece,
            start_field: start_field,
            target_field: target_field
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
#[derive(Clone)]
pub struct Position(String);

impl <'a> Position {
    pub fn split(&'a self) -> Vec<&'a str> {
        self.0.split("/").collect()
    }

    pub fn update_from_matrix(&mut self, matrix: Ref<PositionMatrix>) {
        let mut new_position = String::new();
        let mut new_char: char = ' ';
        let numbers = vec!['1','2','3','4','5','6','7'];
        for (index, rank) in matrix.0.iter().rev().enumerate() {
            for piece in rank {
                match piece.piecetype {
                    PieceType::Rook if piece.color == Color::White => new_char = 'R',
                    PieceType::Rook => new_char = 'r',
                    PieceType::Knight if piece.color == Color::White => new_char = 'N',
                    PieceType::Knight => new_char = 'n',
                    PieceType::Bishop if piece.color == Color::White => new_char = 'B',
                    PieceType::Bishop => new_char = 'b',
                    PieceType::Queen if piece.color == Color::White => new_char = 'Q', 
                    PieceType::Queen => new_char = 'q', 
                    PieceType::King if piece.color == Color::White => new_char = 'K',
                    PieceType::King => new_char = 'k',
                    PieceType::Pawn if piece.color == Color::White => new_char = 'P',
                    PieceType::Pawn => new_char = 'p',
                    PieceType::None => {
                        if numbers.iter().any(|n| n==&new_char) {
                            new_char = char::from_digit(new_position.pop().unwrap().to_digit(10).unwrap() + 1 as u32, 10).unwrap();
                        } else {
                            new_char = '1';
                        }
                    }
                }
                new_position.push(new_char);
            }
            if index < rank.len() - 1 {
                new_char = '/';
                new_position.push(new_char);
            }
        }
        self.0 = new_position;
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

pub struct Game {
    position: RefCell<Position>,
    position_matrix:  RefCell<PositionMatrix>,
    turn: Color,
    castle_availability: CastleAvailability,
    en_passant: Option<Field>,
    halfmove_clock: u16,
    fullmove_clock: u16
}

impl<'a> Game {
    pub fn new() -> Game {
        let start_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        Game {
            position: RefCell::new(Position(String::from(start_string))),
            position_matrix: Game::init_matrix(&Position(String::from(start_string))),
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

    pub fn position(&'a self) -> &RefCell<Position> {
        &self.position
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
        if !self.move_is_legal(&chess_move) { panic!(
                "Illegal move. {} cant move from {:?} to {:?}", 
                chess_move.piece_string(), 
                chess_move.start_field(), 
                chess_move.target_field()
        );}

        self.execute_move(chess_move);

        // parse position string
        self.position().borrow_mut().update_from_matrix(self.position_matrix().borrow());
    }
    
    fn move_is_legal(&mut self, chess_move: &Move) -> bool {

        // assert that a piece was selected
        if chess_move.piece().piecetype() == &PieceType::None || chess_move.piece().color() == &Color::None {
            return false;
        }

        // assert that the piece has the proper color
        if !self.is_players_turn(&chess_move.piece().color()) {
            return false;
        }
        
        // assert that the piece is allowed to move from the start to the target field 
        // this includes asserting that castling or en-passant are available
        if !self.piece_can_reach_target_field(&chess_move) { 
            return false;
        }

        // assert that none of the player's own pieces are on the target field

        // assert that the way to the target field is not blocked

        // assert that the player's king is not in check after the planned move
        // this both prevents pieces moving from pins and moves during checks which don't stop those checks
        
        // in case of castling, assert that the kings does not need to move through check

        true
    }

    fn is_players_turn(&self, turn: &Color) -> bool {
        return self.turn() == turn
    }

    fn piece_can_reach_target_field(&self, chess_move: &Move) -> bool {
        
        let rank_diff: isize = chess_move.target_field().0 as isize - chess_move.start_field().0 as isize;
        let file_diff: isize = chess_move.target_field().1 as isize - chess_move.start_field().1 as isize;
        let rank_diff_abs = isize::abs(rank_diff);
        let file_diff_abs = isize::abs(file_diff);

        match chess_move.piece().piecetype() {
            PieceType::Rook => (rank_diff == 0) ^ (file_diff == 0),
            PieceType::Knight => (rank_diff_abs == 1 && file_diff_abs == 2) || (rank_diff_abs == 2 && file_diff_abs == 1),
            PieceType::Bishop => (rank_diff == file_diff) && (rank_diff != 0),
            PieceType::Queen => ((rank_diff == file_diff) && (rank_diff != 0)) || ((rank_diff == 0) ^ (file_diff == 0)), 
            PieceType::King => rank_diff_abs <= 1 && file_diff_abs <= 1 && (rank_diff_abs + file_diff_abs) >= 1,
            PieceType::Pawn if chess_move.piece().color() == &Color::White => { 
                (rank_diff == 1 && file_diff_abs <= 1) || (rank_diff == 2 && chess_move.start_field().0 == 1 && file_diff == 0)
            },
            PieceType::Pawn if chess_move.piece().color() == &Color::Black => { 
                (rank_diff == -1 && file_diff_abs <= 1) || (rank_diff == -2 && chess_move.start_field().0 == 6 && file_diff == 0)
            },
            _ => panic!("Move not properly processed. {:?}", chess_move) 
        }
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
        assert_eq!(game.position().borrow().0, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
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
        let mut move_string = String::from("2545");
        let chess_move = Move::new(&mut move_string, &mut game);
        let mut move_string = String::from("2545");
        let chess_move_check = Move::new(&mut move_string, &mut game);

        game.execute_move(chess_move);
        assert_eq!(game.position_matrix().borrow().0[chess_move_check.target_field.0][chess_move_check.target_field.1], chess_move_check.piece);
    }

    #[test]
    fn update_position_from_matrix() {

        let mut game = Game::new();
        assert_eq!(game.position().borrow().0, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        
        // make a move 
        let mut move_string = String::from("2545");
        let chess_move = Move::new(&mut move_string, &mut game);
        game.execute_move(chess_move);
        game.position().borrow_mut().update_from_matrix(game.position_matrix().borrow());
        assert_eq!(game.position().borrow().0, "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR");
        
        // make another move        
        let mut move_string = String::from("8263");
        let chess_move = Move::new(&mut move_string, &mut game);
        game.execute_move(chess_move);
        game.position().borrow_mut().update_from_matrix(game.position_matrix().borrow());
        assert_eq!(game.position().borrow().0, "r1bqkbnr/pppppppp/2n5/8/4P3/8/PPPP1PPP/RNBQKBNR");
    }
    

    #[test]
    fn move_is_legal() {
        let mut game = Game::new();

        // legal move
        let mut move_string = String::from("1233");
        let legal_move = Move::new(&mut move_string, &mut game);
        assert!(game.move_is_legal(&legal_move));

        // illegal move
        let mut move_string = String::from("8263");
        let illegal_move = Move::new(&mut move_string, &mut game);
        assert!(!game.move_is_legal(&illegal_move));        
    }

    #[test]
    fn is_players_turn() {
        let game = Game::new();
        assert!(game.is_players_turn(&Color::White));
        assert!(!game.is_players_turn(&Color::Black));
    }

    #[test]
    fn pawn_can_reach_target_field() {       
        let mut game = Game::new();
        
        // single up        
        let mut move_string = String::from("2535");
        let chess_move = Move::new(&mut move_string, &mut game);   
        assert!(game.piece_can_reach_target_field(&chess_move));
    
        // double up
        let mut move_string = String::from("2545");
        let chess_move = Move::new(&mut move_string, &mut game);   
        assert!(game.piece_can_reach_target_field(&chess_move));

        // single vertical up
        let mut move_string = String::from("2534");
        let chess_move = Move::new(&mut move_string, &mut game);   
        assert!(game.piece_can_reach_target_field(&chess_move));

        // single down
        let mut move_string = String::from("7555");
        let chess_move = Move::new(&mut move_string, &mut game);   
        assert!(game.piece_can_reach_target_field(&chess_move));

        // double down
        let mut move_string = String::from("7555");
        let chess_move = Move::new(&mut move_string, &mut game);   
        assert!(game.piece_can_reach_target_field(&chess_move));
        
        // single vertical down
        let mut move_string = String::from("7564");
        let chess_move = Move::new(&mut move_string, &mut game);   
        assert!(game.piece_can_reach_target_field(&chess_move));

        // three up (should fail!)
        let mut move_string = String::from("2656");
        let chess_move = Move::new(&mut move_string, &mut game);   
        assert!(!game.piece_can_reach_target_field(&chess_move));

        // two up one to the side (should fail!)
        let mut move_string = String::from("2142");
        let chess_move = Move::new(&mut move_string, &mut game);   
        assert!(!game.piece_can_reach_target_field(&chess_move));

        // one down two to the side (should fail!)
        let mut move_string = String::from("7765");
        let chess_move = Move::new(&mut move_string, &mut game);   
        assert!(!game.piece_can_reach_target_field(&chess_move));
    }
}