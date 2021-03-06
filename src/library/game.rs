pub use std::cell::Ref;
use std::{cell::RefCell, cmp::max, cmp::PartialEq, collections::LinkedList};

pub enum GameOver {
    No,
    WhiteWon,
    BlackWon,
    Stalemate,
}

#[derive(Debug, PartialEq)]
pub enum MoveError {
    None,
    NoneDigitEntered,
    InvalidNumberOfDigits,
    OutOfBounds,
    NoPieceSelected,
    WrongColorSelected,
    PieceCantReachTarget,
    OwnPieceOnTarget,
    NoPathToTarget,
    PieceIsPinned,
    MovingIntoCheck,
    NotMovingOutOfCheck,
    CastlingThroughCheck,
    CastlingNotAvailable,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
    Black,
    White,
    None,
}

#[derive(Clone, Debug)]
pub struct CastleAvailability {
    white_king: bool,
    white_queen: bool,
    black_king: bool,
    black_queen: bool,
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
    None,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Field(pub usize, pub usize);

pub struct PieceInstance {
    piece: Piece,
    field: Field,
}

impl PieceInstance {
    pub fn piece(&self) -> &Piece {
        &self.piece
    }
    pub fn field(&self) -> &Field {
        &self.field
    }
}

pub struct GameState {
    pub selected_field: Option<(usize, usize)>,
    pub active_states: LinkedList<State>,
    pub inactive_states: LinkedList<State>,
}

pub fn handle_state(new_state: Result<State, MoveError>, game: &mut GameState) {
    use MoveError::*;
    match new_state {
        Ok(new_state) => {
            game.active_states.push_back(new_state);
            game.inactive_states.clear();
        },
        Err(OutOfBounds) => println!("Please stay within the bounds 1-8!"),
        Err(NoneDigitEntered) => println!("Please only enter digits!"),
        Err(InvalidNumberOfDigits) => println!("Please enter four digits!"),
        Err(NoPieceSelected) => println!("No piece selected!"),
        Err(WrongColorSelected) => println!("Enemy piece selected!"),
        Err(PieceCantReachTarget) => println!("The selected piece can't reach this field!"),
        Err(OwnPieceOnTarget) => println!("One of your pieces already is on this field!"),
        Err(NoPathToTarget) => println!("The selected piece has no path to this field!"),
        Err(PieceIsPinned) => println!("The selected piece is pinned!"),
        Err(MovingIntoCheck) => println!("You would be moving into check!"),
        Err(NotMovingOutOfCheck) => println!("You need to move out of check!"),
        Err(CastlingThroughCheck) => println!("You would be castling through check!"),
        Err(CastlingNotAvailable) => println!("You can't castle anymore!"),
        Err(None) => println!("Invalid move!"),
    }
}

#[derive(Debug)]
pub struct Move {
    pub piece: Piece,
    pub start_field: Field,
    pub target_field: Field,
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
            PieceType::Rook => String::from("Rook"),
            PieceType::Knight => String::from("Knight"),
            PieceType::Bishop => String::from("Bishop"),
            PieceType::Queen => String::from("Queen"),
            PieceType::King => String::from("King"),
            PieceType::Pawn => String::from("Pawn"),
            PieceType::None => String::from("Empty field"),
        }
    }

    pub fn rank_difference(&self) -> isize {
        self.target_field.0 as isize - self.start_field.0 as isize
    }

    pub fn rank_distance(&self) -> usize {
        isize::abs(self.rank_difference()) as usize
    }

    pub fn file_difference(&self) -> isize {
        self.target_field.1 as isize - self.start_field.1 as isize
    }

    pub fn file_distance(&self) -> usize {
        isize::abs(self.file_difference()) as usize
    }

    pub fn distance(&self) -> usize {
        max(self.rank_distance(), self.file_distance())
    }

    // for now we assume the syntax "AABB", where
    // AA are the indices of the source field (i.e., 52) [offset by 1]
    // BB are the indices of the target field (i.e., 54) [offset by 1]
    // -> 2545 is equivalent to "e4" in standard notation.
    fn new(
        start_field: &Field,
        target_field: &Field,
        position_matrix: Ref<PositionMatrix>,
    ) -> Move {
        let piece = position_matrix.get_piece_on_field(start_field);
        Move {
            piece,
            start_field: start_field.clone(),
            target_field: target_field.clone(),
        }
    }

    #[rustfmt::skip]
    pub fn parse_move_input(player_input: String) -> Result<(Field, Field), MoveError> {
        let move_chars: Vec<char> = player_input
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .collect();

        if move_chars.len() != 4 {
            return Err(MoveError::InvalidNumberOfDigits);
        }
        
        for char in move_chars.iter() {
            if !char.is_ascii_digit() {
                return Err(MoveError::NoneDigitEntered);
            }
        }

        let move_indices: Vec<usize> = move_chars.iter().map(|c| c.to_digit(10).expect("Faulty move string") as usize).collect();

        if move_indices[0] < 1 || 
           move_indices[0] > 8 || 
           move_indices[1] < 1 || 
           move_indices[1] > 8 || 
           move_indices[2] < 1 || 
           move_indices[2] > 8 || 
           move_indices[3] < 1 || 
           move_indices[3] > 8
        {
            return Err(MoveError::OutOfBounds);
        }
        
        let start_field = Field(move_indices[0] - 1, move_indices[1] - 1);
        let target_field = Field(move_indices[2] - 1, move_indices[3] - 1);

        Ok((start_field, target_field))
    }
}

#[derive(Clone, Debug)]
pub struct Position(String);

impl<'a> Position {
    pub fn split(&'a self) -> Vec<&'a str> {
        self.0.split('/').collect()
    }

    pub fn update_from_matrix(&mut self, matrix: Ref<PositionMatrix>) {
        let mut new_position = String::new();
        let mut new_char: char = ' ';
        let numbers = vec!['1', '2', '3', '4', '5', '6', '7'];
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
                        if numbers.iter().any(|n| n == &new_char) {
                            new_char = char::from_digit(
                                new_position.pop().unwrap().to_digit(10).unwrap() + 1_u32,
                                10,
                            )
                            .unwrap();
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

#[derive(Clone, Debug)]
pub struct PositionMatrix(pub Vec<Vec<Piece>>);

impl PositionMatrix {
    fn has_piece_on_field(&self, field: &Field) -> bool {
        *self.0[field.0][field.1].piecetype() != PieceType::None
    }

    fn get_piece_on_field(&self, field: &Field) -> Piece {
        self.0[field.0][field.1]
    }

    fn get_color_of_piece_on_field(&self, field: &Field) -> &Color {
        self.0[field.0][field.1].color()
    }

    fn get_type_of_piece_on_field(&self, field: &Field) -> &PieceType {
        self.0[field.0][field.1].piecetype()
    }

    fn remove_piece_from_field(&mut self, field: &Field) -> Piece {
        let piece = self.0[field.0][field.1];
        self.0[field.0][field.1] = Piece {
            color: Color::None,
            piecetype: PieceType::None,
        };
        piece
    }

    fn place_piece(&mut self, piece: Piece, field: &Field) -> Piece {
        let current_piece: Piece = self.0[field.0][field.1];
        self.0[field.0][field.1] = piece;
        current_piece
    }
}

#[derive(Debug, Clone)]
pub struct State {
    position: RefCell<Position>,
    position_matrix: RefCell<PositionMatrix>,
    turn: Color,
    castle_availability: CastleAvailability,
    en_passant: Option<Field>,
    halfmove_clock: u16,
    fullmove_clock: u16,
}

impl<'a> State {
    pub fn new(fen: Option<String>) -> State {
        let fen_start_string = match fen {
            Some(fen) => fen,
            None => String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        };
        State::load_game_from_fen(fen_start_string)
    }

    pub fn load_game_from_fen(fen_string: String) -> State {
        let game_state_vec: Vec<&str> = fen_string.split(' ').collect();

        let turn: Color = match game_state_vec[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid FEN string, turn value: {}", game_state_vec[1]),
        };

        let castle_availability: CastleAvailability = match game_state_vec[2] {
            "KQkq" => CastleAvailability {
                white_king: true,
                white_queen: true,
                black_king: true,
                black_queen: true,
            },
            "KQk" => CastleAvailability {
                white_king: true,
                white_queen: true,
                black_king: true,
                black_queen: false,
            },
            "KQq" => CastleAvailability {
                white_king: true,
                white_queen: true,
                black_king: false,
                black_queen: true,
            },
            "KQ" => CastleAvailability {
                white_king: true,
                white_queen: true,
                black_king: false,
                black_queen: false,
            },
            "Kkq" => CastleAvailability {
                white_king: true,
                white_queen: false,
                black_king: true,
                black_queen: true,
            },
            "Kk" => CastleAvailability {
                white_king: true,
                white_queen: false,
                black_king: true,
                black_queen: false,
            },
            "Kq" => CastleAvailability {
                white_king: true,
                white_queen: false,
                black_king: false,
                black_queen: true,
            },
            "K" => CastleAvailability {
                white_king: true,
                white_queen: false,
                black_king: false,
                black_queen: false,
            },
            "Qkq" => CastleAvailability {
                white_king: false,
                white_queen: true,
                black_king: true,
                black_queen: true,
            },
            "Qk" => CastleAvailability {
                white_king: false,
                white_queen: true,
                black_king: true,
                black_queen: false,
            },
            "Qq" => CastleAvailability {
                white_king: false,
                white_queen: true,
                black_king: false,
                black_queen: true,
            },
            "Q" => CastleAvailability {
                white_king: false,
                white_queen: true,
                black_king: false,
                black_queen: false,
            },
            "kq" => CastleAvailability {
                white_king: false,
                white_queen: false,
                black_king: true,
                black_queen: true,
            },
            "k" => CastleAvailability {
                white_king: false,
                white_queen: false,
                black_king: true,
                black_queen: false,
            },
            "q" => CastleAvailability {
                white_king: false,
                white_queen: false,
                black_king: false,
                black_queen: true,
            },
            "-" => CastleAvailability {
                white_king: false,
                white_queen: false,
                black_king: false,
                black_queen: false,
            },
            _ => panic!(
                "Invalid FEN string, castle availability value: {}",
                game_state_vec[2]
            ),
        };

        let mut en_passant: Option<Field> = None;
        if game_state_vec[3].len() > 2 {
            panic!(
                "Invalid FEN string, en-passant value: {}",
                game_state_vec[3]
            )
        } else if game_state_vec[3] != "-" {
            let mut field: Field = Field(9, 9);
            let en_passant_chars: Vec<char> = game_state_vec[3].chars().collect();
            match en_passant_chars[0] {
                'a' => field.1 = 0,
                'b' => field.1 = 1,
                'c' => field.1 = 2,
                'd' => field.1 = 3,
                'e' => field.1 = 4,
                'f' => field.1 = 5,
                'g' => field.1 = 6,
                'h' => field.1 = 7,
                _ => panic!(
                    "Invalid FEN string, en-passant value: {}",
                    game_state_vec[3]
                ),
            }
            field.0 = (en_passant_chars[1].to_digit(10).unwrap() - 1) as usize;

            en_passant = Some(field);
        }

        let halfmove_clock: u16 = game_state_vec[4].parse::<u16>().unwrap();
        let fullmove_clock: u16 = game_state_vec[5].parse::<u16>().unwrap();

        State {
            position: RefCell::new(Position(String::from(game_state_vec[0]))),
            position_matrix: State::init_matrix(&Position(String::from(game_state_vec[0]))),
            turn,
            castle_availability,
            en_passant,
            halfmove_clock,
            fullmove_clock,
        }
    }

    pub fn position(&'a self) -> &RefCell<Position> {
        &self.position
    }

    pub fn position_matrix(&self) -> &RefCell<PositionMatrix> {
        &self.position_matrix
    }

    pub fn castle_availability(&self) -> &CastleAvailability {
        &self.castle_availability
    }

    pub fn en_passant(&self) -> &Option<Field> {
        &self.en_passant
    }

    fn init_matrix(start_position: &Position) -> RefCell<PositionMatrix> {
        let mut matrix: Vec<Vec<Piece>> = Vec::new();
        let ranks: Vec<&str> = start_position.split();
        for (i, rank) in ranks.iter().rev().enumerate() {
            matrix.push(Vec::new());
            for char in rank.chars() {
                match char {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        State::push_none(&mut matrix[i], char.to_digit(10).unwrap() as usize)
                    }
                    'r' => matrix[i].push(Piece {
                        color: Color::Black,
                        piecetype: PieceType::Rook,
                    }),
                    'n' => matrix[i].push(Piece {
                        color: Color::Black,
                        piecetype: PieceType::Knight,
                    }),
                    'b' => matrix[i].push(Piece {
                        color: Color::Black,
                        piecetype: PieceType::Bishop,
                    }),
                    'q' => matrix[i].push(Piece {
                        color: Color::Black,
                        piecetype: PieceType::Queen,
                    }),
                    'k' => matrix[i].push(Piece {
                        color: Color::Black,
                        piecetype: PieceType::King,
                    }),
                    'p' => matrix[i].push(Piece {
                        color: Color::Black,
                        piecetype: PieceType::Pawn,
                    }),
                    'R' => matrix[i].push(Piece {
                        color: Color::White,
                        piecetype: PieceType::Rook,
                    }),
                    'N' => matrix[i].push(Piece {
                        color: Color::White,
                        piecetype: PieceType::Knight,
                    }),
                    'B' => matrix[i].push(Piece {
                        color: Color::White,
                        piecetype: PieceType::Bishop,
                    }),
                    'Q' => matrix[i].push(Piece {
                        color: Color::White,
                        piecetype: PieceType::Queen,
                    }),
                    'K' => matrix[i].push(Piece {
                        color: Color::White,
                        piecetype: PieceType::King,
                    }),
                    'P' => matrix[i].push(Piece {
                        color: Color::White,
                        piecetype: PieceType::Pawn,
                    }),
                    _ => panic!("Position is corrupt\n{:?}", rank.chars()),
                };
            }
            assert!(
                matrix[i].len() == 8,
                "Position matrix doesn't have 8 files\n{:?}",
                matrix[i]
            );
        }
        assert!(matrix.len() == 8, "Position matrix doesn't have 8 ranks");
        RefCell::new(PositionMatrix(matrix))
    }

    fn push_none(rank: &mut Vec<Piece>, num: usize) {
        match num {
            0 => {}
            _ => {
                rank.push(Piece {
                    color: Color::None,
                    piecetype: PieceType::None,
                });
                State::push_none(rank, num - 1)
            }
        }
    }

    pub fn turn(&self) -> &Color {
        &self.turn
    }

    pub fn turn_rev(&self) -> &Color {
        match self.turn {
            Color::White => &Color::Black,
            Color::Black => &Color::White,
            _ => panic!("Invalid game state turn 'None'. State: {:?}", self),
        }
    }

    fn player_has_legal_move(&self) -> bool {
        // iterate through the whole field and try to move each of the players piece
        // if one piece can move, return true
        for (i, rank) in self.position_matrix().borrow().0.iter().enumerate() {
            for (j, piece) in rank.iter().enumerate() {
                // ignore empty fields and enemy pieces
                if piece.color() != self.turn() {
                    continue;
                }

                // check each target field for legal moves
                for (k, rank) in self.position_matrix().borrow().0.iter().enumerate() {
                    for (l, piece) in rank.iter().enumerate() {
                        // if the player already has a piece on this field
                        // we can discount it
                        if piece.color() == self.turn() {
                            continue;
                        }

                        let chess_move =
                            Move::new(&Field(i, j), &Field(k, l), self.position_matrix().borrow());

                        match self.is_move_legal(&chess_move) {
                            Ok(_) => return true,
                            Err(_) => continue,
                        }
                    }
                }
            }
        }

        // if no legal moves were found, return false
        false
    }

    pub fn check_game_over(&self) -> GameOver {
        if self.player_has_legal_move() {
            return GameOver::No;
        }

        if !self.is_player_in_check(&Color::None) {
            return GameOver::Stalemate;
        }

        if self.turn() == &Color::White {
            GameOver::BlackWon
        } else {
            GameOver::WhiteWon
        }
    }

    pub fn perform_turn_from_input(
        player_input: String,
        current_state: &State,
    ) -> Result<State, MoveError> {

        // get fields from player input
        let (start_field, target_field) = match Move::parse_move_input(player_input) {
            Ok((start_field, target_field)) => (start_field, target_field),
            Err(e) => return Err(e),
        };

        // make move struct which also includes the piece that is to be moved
        let chess_move = Move::new(
            &start_field,
            &target_field,
            current_state.position_matrix().borrow(),
        );

        // check whether the move is legal
        // TO DO: errors should properly propagated to the UI rather than panicking!
        match current_state.is_move_legal(&chess_move) {            
            Ok(_) => Ok(current_state.execute_move(&chess_move)),
            Err(e) => Err(e),
        }
    }

    fn is_move_legal(&self, chess_move: &Move) -> Result<bool, MoveError> {

        // assert that a piece was selected
        if chess_move.piece().piecetype() == &PieceType::None
            || chess_move.piece().color() == &Color::None
        {
            return Err(MoveError::NoPieceSelected);
        }

        // assert that the piece has the proper color
        if !self.is_players_turn(chess_move.piece().color()) {
            return Err(MoveError::WrongColorSelected);
        }

        // assert that the piece is allowed to move from the start to the target field
        // this includes asserting that castling or en-passant are available
        match self.piece_can_reach_target_field(chess_move) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }

        // assert that none of the player's own pieces are on the target field
        if self
            .position_matrix()
            .borrow()
            .get_color_of_piece_on_field(chess_move.target_field())
            == self.turn()
        {
            return Err(MoveError::OwnPieceOnTarget);
        }
        
        // assert that the way to the target field is not blocked
        if !self.piece_has_path_to_target_field(chess_move) {
            return Err(MoveError::NoPathToTarget);
        }

        // make a 'hypothetical move' and check whether the player would be in check
        let hypothetical_state: State = self.execute_move(chess_move);
        if hypothetical_state.is_player_in_check(self.turn()) {
            if chess_move.piece.piecetype() == &PieceType::King {
                // 1) king would have moved into check
                return Err(MoveError::MovingIntoCheck);
            } else {
                if self.is_player_in_check(self.turn()) {
                    // 2) didn't move out of check
                    return Err(MoveError::NotMovingOutOfCheck);
                } else {
                    // 3) piece moved out of a pin
                    return Err(MoveError::PieceIsPinned);
                }
            } 
        }

        // in case of castling, assert that the kings does not move through check
        if self.is_castling_through_check(chess_move) {
            return Err(MoveError::CastlingThroughCheck);
        }

        // the move is legal, if no condition made it illegal
        Ok(true)
    }

    fn is_players_turn(&self, turn: &Color) -> bool {
        return self.turn() == turn;
    }

    #[rustfmt::skip]
    fn piece_can_reach_target_field(&self, chess_move: &Move) -> Result<bool, MoveError> {
        let rank_diff = chess_move.rank_difference();
        let file_diff = chess_move.file_difference();
        let rank_diff_abs = isize::abs(rank_diff);
        let file_diff_abs = isize::abs(file_diff);
        let color = chess_move.piece().color();
        let matrix = self.position_matrix().borrow();

        match chess_move.piece().piecetype() {
            PieceType::Rook => State::can_reach_target_result(
                    (rank_diff == 0) ^ (file_diff == 0)),
            PieceType::Knight => State::can_reach_target_result(
                    (rank_diff_abs == 2 && file_diff_abs == 1)
                ||  (rank_diff_abs == 1 && file_diff_abs == 2)),
            PieceType::Bishop => State::can_reach_target_result(
                    (rank_diff_abs == file_diff_abs) && (rank_diff != 0)),
            PieceType::Queen => State::can_reach_target_result(
                    ((rank_diff_abs == file_diff_abs) && (rank_diff != 0))
                ||  ((rank_diff == 0) ^ (file_diff == 0))
            ),
            PieceType::King => {
                if rank_diff_abs <= 1 && file_diff_abs <= 1 && (rank_diff_abs + file_diff_abs) >= 1 {
                    Ok(true)
                } else {
                    let is_castling = match *color {
                        Color::White => (
                            chess_move.target_field().0 == 0 && chess_move.target_field().1 == 2, 
                            chess_move.target_field().0 == 0 && chess_move.target_field().1 == 6), 
                        Color::Black => (
                            chess_move.target_field().0 == 7 && chess_move.target_field().1 == 2, 
                            chess_move.target_field().0 == 7 && chess_move.target_field().1 == 6),
                        _ => panic!("Invalid color"),
                    };
                    State::castle_availability_result(color, is_castling, self.castle_availability())
                }
            },
            PieceType::Pawn => {
                if  (file_diff == 0 && rank_diff ==  1 && color == &Color::White)
                ||  (file_diff == 0 && rank_diff == -1 && color == &Color::Black)
                ||  (file_diff == 0 && rank_diff ==  2 && color == &Color::White && chess_move.start_field().0 == 1) 
                ||  (file_diff == 0 && rank_diff == -2 && color == &Color::Black && chess_move.start_field().0 == 6) {
                    return Ok(true)
                } 
                if rank_diff_abs != 1 || file_diff_abs != 1 {
                    return Err(MoveError::PieceCantReachTarget)
                } 
                let opposite_color = match *color {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                    _ => panic!("Invalid color"),
                };
                match self.en_passant() {
                    Some(field) if field == &chess_move.target_field => Ok(true),
                    _ => State::can_reach_target_result(
                        matrix.get_type_of_piece_on_field(chess_move.target_field()) != &PieceType::None
                     && matrix.get_color_of_piece_on_field(chess_move.target_field()) == &opposite_color)
                }
            }
            _ => panic!("Move not properly processed. {:?}", chess_move),
        }
    }
    
    fn can_reach_target_result(can_reach: bool) -> Result<bool, MoveError> {
        if can_reach {
            Ok(true)
        } else {
            Err(MoveError::PieceCantReachTarget)
        }
    }
    
    fn castle_availability_result(color: &Color, is_castling: (bool, bool), castle_available: &CastleAvailability) -> Result<bool, MoveError> {
        let availability = match *color {
            Color::White => (castle_available.white_queen, castle_available.white_king),
            Color::Black => (castle_available.black_queen, castle_available.black_king),
            _ => panic!("Invalid color"), 
        };

        if is_castling.0 || is_castling.1 {
            if is_castling.0 && availability.0 || is_castling.1 && availability.1 {
                Ok(true)
            } else {
                Err(MoveError::CastlingNotAvailable)
            } 
        } else {
            Err(MoveError::PieceCantReachTarget)
        }
    }

    fn piece_has_path_to_target_field(&self, chess_move: &Move) -> bool {
        match chess_move.piece().piecetype() {
            PieceType::Pawn | PieceType::King | PieceType::Knight => true,
            PieceType::Rook | PieceType::Bishop | PieceType::Queen => {
                let rank_diff: isize =
                    chess_move.target_field().0 as isize - chess_move.start_field().0 as isize;
                let file_diff: isize =
                    chess_move.target_field().1 as isize - chess_move.start_field().1 as isize;
                let rank_diff_abs = isize::abs(rank_diff);
                let file_diff_abs = isize::abs(file_diff);

                let rank_direction = match rank_diff {
                    0 => 0,
                    _ => rank_diff / rank_diff_abs,
                };

                let file_direction = match file_diff {
                    0 => 0,
                    _ => file_diff / file_diff_abs,
                };

                for i in 1..max(rank_diff_abs, file_diff_abs) {
                    let rank_index =
                        (chess_move.start_field().0 as isize + i * rank_direction) as usize;
                    let file_index =
                        (chess_move.start_field().1 as isize + i * file_direction) as usize;
                    if self
                        .position_matrix()
                        .borrow()
                        .has_piece_on_field(&Field(rank_index, file_index))
                    {
                        return false;
                    }
                }
                true
            }
            _ => true,
        }
    }

    fn is_castling_through_check(&self, chess_move: &Move) -> bool {
        if chess_move.piece().piecetype() != &PieceType::King {
            return false;
        }

        let file_diff = chess_move.file_difference();
        let file_diff_abs = isize::abs(file_diff);
        if file_diff_abs < 2 {
            return false;
        };

        // for all fields in the kings path
        // check whether an enemy piece is attacking it
        // note that the target field is _not_ checked!
        let enemy_color: Color = match *self.turn() {
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => panic!("No valid player color requested!"),
        };
        let direction = file_diff / file_diff_abs;
        for i in 0..(file_diff_abs + 1) {
            let field: Field = Field(
                chess_move.start_field().0,
                (chess_move.start_field().1 as isize + direction * i) as usize,
            );
            if self.is_players_piece_attacking_field(&enemy_color, &field) {
                return true;
            }
        }
        false
    }

    fn is_players_piece_attacking_field(&self, player: &Color, field: &Field) -> bool {
        for (i, rank) in self.position_matrix().borrow().0.iter().enumerate() {
            for (j, piece) in rank.iter().enumerate() {
                if piece.color() != player {
                    continue;
                }

                let chess_move = Move::new(&Field(i, j), field, self.position_matrix().borrow());
                match State::piece_can_reach_target_field(self, &chess_move) {
                    Ok(_) if State::piece_has_path_to_target_field(self, &chess_move) => return true,
                    _ => continue,
                }
            }
        }

        false
    }

    fn move_rook_when_castling(&self, chess_move: &Move) {
        if chess_move.piece().piecetype() != &PieceType::King {
            return; // not castling
        }

        let distance = chess_move.file_distance();
        if distance < 2 {
            return; // not castling
        }

        let direction = chess_move.file_difference() / distance as isize;

        // remove rook
        let rook_start_field = match direction {
            1 => Field(chess_move.target_field().0, 7),
            -1 => Field(chess_move.target_field().0, 0),
            _ => panic!("Something terrible happened: {:?}", direction),
        };
        let rook = self
            .position_matrix()
            .borrow_mut()
            .remove_piece_from_field(&rook_start_field);

        // place rook again
        let rook_target_field = Field(
            chess_move.target_field().0,
            (chess_move.target_field().1 as isize - direction) as usize,
        );
        self.position_matrix()
            .borrow_mut()
            .place_piece(rook, &rook_target_field);
    }

    fn remove_enemy_pawn_en_passant(&self, chess_move: &Move) {
        if chess_move.piece().piecetype() != &PieceType::Pawn {
            return; // not en-passant
        }

        let target = chess_move.target_field();

        match self.en_passant() {
            None => return, // not en-passant
            Some(field) => {
                // the en passant position in FEN notation is the field
                // behind the pawn that just moved two squares,
                // i.e. the field that the enemy pawn can move
                if field != target {
                    return; // not en-passant
                }
            }
        }

        // we need to remove the pawn from the field
        // just below/above the target field (depending on the color)
        if self.turn() == &Color::White {
            self.position_matrix()
                .borrow_mut()
                .remove_piece_from_field(&Field(target.0 - 1, target.1));
        } else {
            self.position_matrix()
                .borrow_mut()
                .remove_piece_from_field(&Field(target.0 + 1, target.1));
        }
    }

    fn execute_move(&self, chess_move: &Move) -> State {
        // the current board serves as the basis of the next state, but itself is left as-is.
        let mut new_state = self.clone();

        // take the piece that is moving
        new_state
            .position_matrix()
            .borrow_mut()
            .remove_piece_from_field(&chess_move.start_field);

        // place it on the new field and take the piece that was on it
        let captured_piece: Piece = new_state
            .position_matrix()
            .borrow_mut()
            .place_piece(chess_move.piece, &chess_move.target_field);

        // castling and en-passant need to be handled separately
        new_state.move_rook_when_castling(chess_move);
        new_state.update_castling_availability(chess_move);
        new_state.remove_enemy_pawn_en_passant(chess_move);
        new_state.update_en_passant(chess_move);

        // turn the clocks
        if captured_piece.piecetype() != &PieceType::None
            || chess_move.piece().piecetype() == &PieceType::Pawn
        {
            new_state.halfmove_clock = 0;
        } else {
            new_state.halfmove_clock += 1;
        }

        if new_state.turn() == &Color::Black {
            new_state.fullmove_clock += 1;
        }

        new_state.toggle_turn();
        new_state
            .position()
            .borrow_mut()
            .update_from_matrix(new_state.position_matrix().borrow());

        new_state
    }

    fn update_en_passant(&mut self, chess_move: &Move) {
        if chess_move.piece().piecetype() != &PieceType::Pawn || chess_move.rank_distance() < 2 {
            self.en_passant = None;
            return;
        }

        // the en passant field is the field between the pawns starting and target field
        self.en_passant = match *self.turn() {
            Color::White => Some(Field(
                chess_move.start_field().0 + 1,
                chess_move.start_field().1,
            )),
            Color::Black => Some(Field(
                chess_move.start_field().0 - 1,
                chess_move.start_field().1,
            )),
            _ => panic!("Invalid game state, no turn. {:?}", self),
        };
    }

    fn update_castling_availability(&mut self, chess_move: &Move) {
        if chess_move.piece().piecetype() != &PieceType::Rook
            && chess_move.piece().piecetype() != &PieceType::King
        {
            return; // nothing to do here
        }

        if chess_move.piece().piecetype() == &PieceType::King {
            if self.turn() == &Color::Black {
                self.castle_availability.black_king = false;
                self.castle_availability.black_queen = false;
            } else {
                self.castle_availability.white_king = false;
                self.castle_availability.white_queen = false;
            }
            return;
        }

        if chess_move.start_field().1 == 0 {
            if self.turn() == &Color::Black {
                self.castle_availability.black_queen = false;
            } else {
                self.castle_availability.white_queen = false;
            }
            return;
        }

        if chess_move.start_field().1 == 7 {
            if self.turn() == &Color::Black {
                self.castle_availability.black_king = false;
            } else {
                self.castle_availability.white_king = false;
            }
        }
    }

    fn toggle_turn(&mut self) {
        if self.turn == Color::White {
            self.turn = Color::Black;
        } else {
            self.turn = Color::White;
        }
    }

    fn get_king_field(position_matrix: Ref<PositionMatrix>, color: &Color) -> Option<Field> {
        for (i, rank) in position_matrix.0.iter().enumerate() {
            for (j, piece) in rank.iter().enumerate() {
                if piece.color() == color && piece.piecetype() == &PieceType::King {
                    return Some(Field(i, j));
                }
            }
        }
        None
    }

    fn is_player_in_check(&self, color: &Color) -> bool {
        let player_color = match color {
            &Color::None => self.turn(),
            _ => color,
        };

        let enemy_color: Color = match *player_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::None => panic!("Corrupt game state, no turn. State: {:?}", self),
        };

        let king_field: Field =
            State::get_king_field(self.position_matrix().borrow(), player_color).unwrap();

        self.is_players_piece_attacking_field(&enemy_color, &king_field)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_game() {
        let state = State::new(None);
        assert_eq!(
            state.position().borrow().0,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
        assert_eq!(
            state.position_matrix().borrow().0[0][0],
            Piece {
                color: Color::White,
                piecetype: PieceType::Rook
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[0][1],
            Piece {
                color: Color::White,
                piecetype: PieceType::Knight
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[0][2],
            Piece {
                color: Color::White,
                piecetype: PieceType::Bishop
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[0][3],
            Piece {
                color: Color::White,
                piecetype: PieceType::Queen
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[0][4],
            Piece {
                color: Color::White,
                piecetype: PieceType::King
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[0][5],
            Piece {
                color: Color::White,
                piecetype: PieceType::Bishop
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[0][6],
            Piece {
                color: Color::White,
                piecetype: PieceType::Knight
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[0][7],
            Piece {
                color: Color::White,
                piecetype: PieceType::Rook
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[1][0],
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[1][1],
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[1][2],
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[1][3],
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[1][4],
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[1][5],
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[1][6],
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[1][7],
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[6][0],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[6][1],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[6][2],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[6][3],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[6][4],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[6][5],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[6][6],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[6][7],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[7][0],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Rook
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[7][1],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Knight
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[7][2],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Bishop
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[7][3],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Queen
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[7][4],
            Piece {
                color: Color::Black,
                piecetype: PieceType::King
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[7][5],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Bishop
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[7][6],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Knight
            }
        );
        assert_eq!(
            state.position_matrix().borrow().0[7][7],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Rook
            }
        );
    }

    #[test]
    fn matrix_take_piece() {
        let state = State::new(None);
        assert!(
            state.position_matrix().borrow().0[0][0]
                != Piece {
                    color: Color::None,
                    piecetype: PieceType::None
                }
        );
        state
            .position_matrix()
            .borrow_mut()
            .remove_piece_from_field(&Field(0, 0));
        assert!(
            state.position_matrix().borrow().0[0][0]
                == Piece {
                    color: Color::None,
                    piecetype: PieceType::None
                }
        );
    }

    #[test]
    fn new_move() {
        let state = State::new(None);
        let start_field = Field(1, 4);
        let target_field = Field(3, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert_eq!(
            chess_move.piece,
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(chess_move.start_field, Field(1, 4));
        assert_eq!(chess_move.target_field, Field(3, 4));

        let start_field = Field(6, 2);
        let target_field = Field(5, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert_eq!(
            chess_move.piece,
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(chess_move.start_field, Field(6, 2));
        assert_eq!(chess_move.target_field, Field(5, 2));
    }

    #[test]
    fn matrix_execute_move() {
        let state = State::new(None);

        let start_field = Field(1, 4);
        let target_field = Field(3, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        let chess_move_check = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );

        let next_state = state.execute_move(&chess_move);

        assert_eq!(
            next_state.position_matrix().borrow().0[chess_move_check.start_field.0]
                [chess_move_check.start_field.1],
            Piece {
                color: Color::None,
                piecetype: PieceType::None
            }
        );
        assert_eq!(
            next_state.position_matrix().borrow().0[chess_move_check.target_field.0]
                [chess_move_check.target_field.1],
            chess_move_check.piece
        );
    }

    #[test]
    fn update_position_from_matrix() {
        let state = State::new(None);
        assert_eq!(
            state.position().borrow().0,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );

        // make a move
        let start_field = Field(1, 4);
        let target_field = Field(3, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        let mut next_state = state.execute_move(&chess_move);
        next_state
            .position()
            .borrow_mut()
            .update_from_matrix(next_state.position_matrix().borrow());
        assert_eq!(
            next_state.position().borrow().0,
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR"
        );

        // make another move
        let start_field = Field(7, 1);
        let target_field = Field(5, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        next_state = next_state.execute_move(&chess_move);
        next_state
            .position()
            .borrow_mut()
            .update_from_matrix(next_state.position_matrix().borrow());
        assert_eq!(
            next_state.position().borrow().0,
            "r1bqkbnr/pppppppp/2n5/8/4P3/8/PPPP1PPP/RNBQKBNR"
        );
    }

    #[test]
    fn is_move_legal() {
        let state = State::new(None);

        // legal move
        let start_field = Field(0, 1);
        let target_field = Field(2, 2);
        let legal_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.is_move_legal(&legal_move).unwrap());

        // illegal move
        let start_field = Field(7, 1);
        let target_field = Field(5, 2);
        let illegal_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(match state.is_move_legal(&illegal_move) {
            Ok(_) => false,
            Err(_) => true,
        });
    }

    #[test]
    fn is_players_turn() {
        let state = State::new(None);
        assert!(state.is_players_turn(&Color::White));
        assert!(!state.is_players_turn(&Color::Black));
    }

    #[test]
    fn pawn_can_reach_target_field() {
        let state = State::new(None);

        // single up
        let start_field = Field(1, 4);
        let target_field = Field(2, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // double up
        let start_field = Field(1, 4);
        let target_field = Field(3, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // single down
        let start_field = Field(6, 4);
        let target_field = Field(5, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // double down
        let start_field = Field(6, 4);
        let target_field = Field(4, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // single diagonal down (should return false since no enemy piece is there)
        let start_field = Field(6, 4);
        let target_field = Field(5, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // single diagonal up (should return false since no enemy piece is there)
        let start_field = Field(1, 4);
        let target_field = Field(2, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // three up (should return false!)
        let start_field = Field(1, 5);
        let target_field = Field(4, 5);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // two up one to the side (should return false!)
        let start_field = Field(1, 0);
        let target_field = Field(3, 1);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // one down two to the side (should return false!)
        let start_field = Field(6, 6);
        let target_field = Field(5, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));
    }

    #[test]
    fn king_can_reach_target_field() {
        let state = State::new(None);

        // single up
        let start_field = Field(0, 4);
        let target_field = Field(1, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // single right
        let start_field = Field(0, 4);
        let target_field = Field(0, 5);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // double up
        let start_field = Field(0, 4);
        let target_field = Field(2, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // double diagonal
        let start_field = Field(0, 4);
        let target_field = Field(2, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // castle queen-side
        let start_field = Field(0, 4);
        let target_field = Field(0, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // castle king-side
        let start_field = Field(0, 4);
        let target_field = Field(0, 6);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // TODO: assert castling is NOT allowed e.g. when king moved
    }

    #[test]
    fn knight_can_reach_target_field() {
        let state = State::new(None);

        // two up one right
        let start_field = Field(0, 1);
        let target_field = Field(2, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // two up one left
        let start_field = Field(0, 1);
        let target_field = Field(2, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // two right one down
        let start_field = Field(7, 1);
        let target_field = Field(6, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // two right one up
        let start_field = Field(0, 1);
        let target_field = Field(1, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // single diagonal up
        let start_field = Field(0, 1);
        let target_field = Field(1, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // single diagonal down
        let start_field = Field(7, 6);
        let target_field = Field(6, 5);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // double up
        let start_field = Field(0, 1);
        let target_field = Field(2, 1);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // double to side
        let start_field = Field(0, 6);
        let target_field = Field(0, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // single up
        let start_field = Field(0, 6);
        let target_field = Field(1, 6);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));
    }

    #[test]
    fn bishop_can_reach_target_field() {
        let state = State::new(None);

        // one diagonal up
        let start_field = Field(0, 2);
        let target_field = Field(1, 1);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // one diagonal down
        let start_field = Field(7, 2);
        let target_field = Field(6, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // three diagonal up
        let start_field = Field(0, 2);
        let target_field = Field(3, 5);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // five diagonal down
        let start_field = Field(7, 5);
        let target_field = Field(2, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // one to the side
        let start_field = Field(0, 2);
        let target_field = Field(0, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // one up
        let start_field = Field(0, 2);
        let target_field = Field(1, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // one down
        let start_field = Field(7, 2);
        let target_field = Field(6, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // two down, one to the side
        let start_field = Field(7, 2);
        let target_field = Field(5, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));
    }

    #[test]
    fn rook_can_reach_target_field() {
        let state = State::new(None);

        // one up
        let start_field = Field(0, 0);
        let target_field = Field(1, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // four up
        let start_field = Field(0, 0);
        let target_field = Field(4, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // seven down
        let start_field = Field(7, 0);
        let target_field = Field(0, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // one to the side
        let start_field = Field(7, 0);
        let target_field = Field(7, 1);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // five to the side
        let start_field = Field(7, 7);
        let target_field = Field(7, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // one diagonal
        let start_field = Field(0, 0);
        let target_field = Field(1, 1);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // three diagonal
        let start_field = Field(7, 7);
        let target_field = Field(4, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // four to the side three down
        let start_field = Field(7, 7);
        let target_field = Field(4, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));
    }

    #[test]
    fn queen_can_reach_target_field() {
        let state = State::new(None);

        // one diagonal up
        let start_field = Field(0, 3);
        let target_field = Field(1, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // one diagonal down
        let start_field = Field(7, 3);
        let target_field = Field(6, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // three diagonal up
        let start_field = Field(0, 3);
        let target_field = Field(3, 6);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // four diagonal down
        let start_field = Field(7, 3);
        let target_field = Field(3, 7);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // one up
        let start_field = Field(0, 3);
        let target_field = Field(1, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // four up
        let start_field = Field(0, 3);
        let target_field = Field(4, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // seven down
        let start_field = Field(7, 3);
        let target_field = Field(0, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // one to the side
        let start_field = Field(7, 3);
        let target_field = Field(7, 2);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // three to the side
        let start_field = Field(0, 3);
        let target_field = Field(0, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move).unwrap());

        // two down, one to the side
        let start_field = Field(7, 3);
        let target_field = Field(5, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));

        // four to the side three down
        let start_field = Field(7, 3);
        let target_field = Field(3, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(state.piece_can_reach_target_field(&chess_move) == Err(MoveError::PieceCantReachTarget));
    }

    // this test is pretty much a dummy, since the method always returns true
    // for the same reason, there are no tests for king and knight
    #[test]
    fn pawn_has_path_to_target_field() {
        let fen_start_string =
            String::from("r2qkbnr/ppp2ppp/2np4/4p3/3PP1b1/1PP2N2/P4PPP/RNBQKB1R b KQkq d3 0 5");
        let state = State::load_game_from_fen(fen_start_string);
        let start_field = Field(4, 4);
        let target_field = Field(3, 3);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(State::piece_has_path_to_target_field(&state, &chess_move));
    }

    #[test]
    fn rook_has_path_to_target_field() {
        let fen_start_string =
            String::from("r2q1rk1/ppp1bppp/3p1n2/6B1/2BNP3/1P6/P4PPP/RN1K3R w - - 5 11");
        let state = State::load_game_from_fen(fen_start_string);

        let start_field = Field(0, 7);
        let target_field = Field(0, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(State::piece_has_path_to_target_field(&state, &chess_move));

        let start_field = Field(0, 0);
        let target_field = Field(4, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(!State::piece_has_path_to_target_field(&state, &chess_move));

        let start_field = Field(0, 0);
        let target_field = Field(0, 4);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(!State::piece_has_path_to_target_field(&state, &chess_move));
    }

    #[test]
    fn bishop_has_path_to_target_field() {
        let fen_start_string =
            String::from("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3");
        let state = State::load_game_from_fen(fen_start_string);

        let start_field = Field(0, 5);
        let target_field = Field(4, 1);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(State::piece_has_path_to_target_field(&state, &chess_move));

        let new_state = state.execute_move(&chess_move);
        let start_field = Field(7, 2);
        let target_field = Field(3, 6);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            new_state.position_matrix().borrow(),
        );
        assert!(!State::piece_has_path_to_target_field(
            &new_state,
            &chess_move
        ));
    }

    #[test]
    fn queen_has_path_to_target_field() {
        let fen_start_string =
            String::from("r1b1kb1r/pp1ppppp/1q3n2/8/2BQP3/8/PPP2PPP/RNB1K2R w KQkq - 3 7");
        let state = State::load_game_from_fen(fen_start_string);

        let start_field = Field(3, 3);
        let target_field = Field(5, 1);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(State::piece_has_path_to_target_field(&state, &chess_move));

        let start_field = Field(3, 3);
        let target_field = Field(6, 0);
        let chess_move = Move::new(
            &start_field,
            &target_field,
            state.position_matrix().borrow(),
        );
        assert!(!State::piece_has_path_to_target_field(&state, &chess_move));
    }

    /*
       position: RefCell<Position>,
       position_matrix:  RefCell<PositionMatrix>,
       turn: Color,
       castle_availability: CastleAvailability,
       en_passant: Option<Field>,
       halfmove_clock: u16,
       fullmove_clock: u16
    */
    #[test]
    fn load_game_from_fen_new() {
        let fen_start_string =
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let state = State::load_game_from_fen(fen_start_string);
        assert_eq!(
            state.position().borrow().0,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
        assert_eq!(state.turn, Color::White);
        assert!(state.castle_availability().white_king);
        assert!(state.castle_availability().white_queen);
        assert!(state.castle_availability().black_king);
        assert!(state.castle_availability().black_queen);
        assert_eq!(state.en_passant, None);
        assert_eq!(state.halfmove_clock, 0);
        assert_eq!(state.fullmove_clock, 1);
    }

    #[test]
    fn load_game_from_fen_sicilian_alapin() {
        let fen_start_string =
            String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 2");
        let state = State::load_game_from_fen(fen_start_string);
        assert_eq!(
            state.position().borrow().0,
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR"
        );
        assert_eq!(state.turn, Color::Black);
        assert!(state.castle_availability().white_king);
        assert!(state.castle_availability().white_queen);
        assert!(state.castle_availability().black_king);
        assert!(state.castle_availability().black_queen);
        assert_eq!(state.en_passant, None);
        assert_eq!(state.halfmove_clock, 0);
        assert_eq!(state.fullmove_clock, 2);
    }

    #[test]
    fn load_game_from_fen_hikaru_harikrishna() {
        let fen_start_string =
            String::from("r6k/1p1q3p/3r1pp1/b1R1N3/p2PQ3/P5P1/1P3P1P/3R2K1 b - - 0 28");
        let state = State::load_game_from_fen(fen_start_string);
        assert_eq!(
            state.position().borrow().0,
            "r6k/1p1q3p/3r1pp1/b1R1N3/p2PQ3/P5P1/1P3P1P/3R2K1"
        );
        assert_eq!(state.turn, Color::Black);
        assert!(!state.castle_availability().white_king);
        assert!(!state.castle_availability().white_queen);
        assert!(!state.castle_availability().black_king);
        assert!(!state.castle_availability().black_queen);
        assert_eq!(state.en_passant, None);
        assert_eq!(state.halfmove_clock, 0);
        assert_eq!(state.fullmove_clock, 28);
    }

    #[test]
    fn load_game_from_fen_pirc_w_en_passant() {
        let fen_start_string =
            String::from("rnbqkb1r/pp3ppp/3p1n2/2pPp3/4P3/2N5/PPP2PPP/R1BQKBNR w KQkq c6 0 5");
        let state = State::load_game_from_fen(fen_start_string);
        assert_eq!(
            state.position().borrow().0,
            "rnbqkb1r/pp3ppp/3p1n2/2pPp3/4P3/2N5/PPP2PPP/R1BQKBNR"
        );
        assert_eq!(state.turn, Color::White);
        assert!(state.castle_availability().white_king);
        assert!(state.castle_availability().white_queen);
        assert!(state.castle_availability().black_king);
        assert!(state.castle_availability().black_queen);
        assert_eq!(state.en_passant, Some(Field(5, 2)));
        assert_eq!(state.halfmove_clock, 0);
        assert_eq!(state.fullmove_clock, 5);
    }

    #[test]
    fn load_game_from_fen_castling_partially_available() {
        let fen_start_string =
            String::from("rnbq1rk1/p3bppp/2pp1n2/4p1B1/2B1P3/2N5/PPP2PPP/R2QK1NR w KQ - 4 8");
        let state = State::load_game_from_fen(fen_start_string);
        assert_eq!(
            state.position().borrow().0,
            "rnbq1rk1/p3bppp/2pp1n2/4p1B1/2B1P3/2N5/PPP2PPP/R2QK1NR"
        );
        assert_eq!(state.turn, Color::White);
        assert!(state.castle_availability().white_king);
        assert!(state.castle_availability().white_queen);
        assert!(!state.castle_availability().black_king);
        assert!(!state.castle_availability().black_queen);
        assert_eq!(state.en_passant, None);
        assert_eq!(state.halfmove_clock, 4);
        assert_eq!(state.fullmove_clock, 8);
    }

    #[test]
    fn is_player_in_check_01() {
        let fen_string =
            String::from("r1b1kb1r/pp1ppppp/1q3n2/8/2BQP3/8/PPP2PPP/RNB1K2R w KQkq - 3 7");
        let state = State::load_game_from_fen(fen_string);
        assert!(!state.is_player_in_check(&Color::White));
        assert!(!state.is_player_in_check(&Color::Black));
    }

    #[test]
    fn is_player_in_check_02() {
        let fen_string =
            String::from("r1b1kb1r/pp1Qpppp/1q3n2/8/2B1P3/8/PPP2PPP/RNB1K2R b KQkq - 0 7");
        let state = State::load_game_from_fen(fen_string);
        assert!(!state.is_player_in_check(&Color::White));
        assert!(state.is_player_in_check(&Color::Black));
    }

    #[test]
    fn is_player_in_check_03() {
        let fen_string =
            String::from("r3kb1r/pp1bpppp/5n2/6B1/1qB1P3/8/PPP2PPP/RN2K2R w KQkq - 2 9");
        let state = State::load_game_from_fen(fen_string);
        assert!(state.is_player_in_check(&Color::White));
        assert!(!state.is_player_in_check(&Color::Black));
    }

    #[test]
    fn is_move_legal_pin() {
        let fen_string =
            String::from("r1bqkbnr/ppp2ppp/2np4/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 1 4");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(5, 2), &Field(3, 3), state.position_matrix().borrow());
        assert!(match state.is_move_legal(&chess_move) {
            Ok(_) => false,
            Err(e) if e == MoveError::PieceIsPinned => true,
            Err(_) => false,
        });
    }

    #[test]
    fn is_move_legal_no_pin() {
        let fen_string =
            String::from("r2qkbnr/pppb1ppp/2np4/1B2p3/4P3/2N2N1P/PPPP1PP1/R1BQK2R b KQkq - 0 5");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(5, 2), &Field(3, 3), state.position_matrix().borrow());
        assert!(state.is_move_legal(&chess_move).unwrap());
    }

    #[test]
    fn is_move_legal_king_runs_into_check() {
        let fen_string =
            String::from("r2qkbnr/pppb1ppp/3p4/1B2p3/3nP3/2N2N1P/PPPP1PP1/R1BQK2R w KQkq - 1 6");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(0, 4), &Field(1, 4), state.position_matrix().borrow());
        assert!(match state.is_move_legal(&chess_move) {
            Ok(_) => false,
            Err(e) if e == MoveError::MovingIntoCheck => true,
            Err(_) => false,
        });
    }

    #[test]
    fn is_move_legal_king_does_not_run_into_check() {
        let fen_string =
            String::from("r2qkbnr/pppb1ppp/2np4/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQK2R w KQkq - 2 5");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(0, 4), &Field(1, 4), state.position_matrix().borrow());
        assert!(state.is_move_legal(&chess_move).unwrap());
    }

    #[test]
    fn is_players_piece_attacking_field_pawn() {
        let fen_string =
            String::from("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");
        let state = State::load_game_from_fen(fen_string);
        assert!(state.is_players_piece_attacking_field(&Color::White, &Field(4, 3)));
    }

    #[test]
    fn is_players_piece_attacking_field_knight() {
        let fen_string =
            String::from("r1bqkbnr/ppp1pppp/2n5/1B6/8/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 5 5");
        let state = State::load_game_from_fen(fen_string);
        assert!(state.is_players_piece_attacking_field(&Color::Black, &Field(3, 3)));
    }

    #[test]
    fn is_players_piece_attacking_field_bishop() {
        let fen_string =
            String::from("r1bqkb1r/ppp1pppp/2n2n2/1B6/8/2N2N2/PPPP1PPP/R1BQK2R w KQkq - 6 6");
        let state = State::load_game_from_fen(fen_string);
        assert!(state.is_players_piece_attacking_field(&Color::White, &Field(5, 2)));
    }

    #[test]
    fn is_players_piece_attacking_field_bishop_blocked() {
        let fen_string =
            String::from("r1bqkb1r/ppp1pppp/2n2n2/1B6/8/2N2N2/PPPP1PPP/R1BQK2R w KQkq - 6 6");
        let state = State::load_game_from_fen(fen_string);
        assert!(!state.is_players_piece_attacking_field(&Color::White, &Field(7, 4)));
    }

    #[test]
    fn is_players_piece_attacking_field_rook_blocked() {
        let fen_string =
            String::from("r1bqkb1r/ppp1pppp/2n2n2/1B6/8/2N2N2/PPPP1PPP/R1BQ1RK1 b kq - 7 6");
        let state = State::load_game_from_fen(fen_string);
        assert!(!state.is_players_piece_attacking_field(&Color::Black, &Field(1, 7)));
    }

    #[test]
    fn is_castling_through_check_no_check() {
        let fen_string =
            String::from("r1bqkb1r/ppp1pppp/2n2n2/1B6/8/2N2N2/PPPP1PPP/R1BQK2R w KQkq - 6 6");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(0, 4), &Field(0, 6), state.position_matrix().borrow());
        assert!(!state.is_castling_through_check(&chess_move));
    }

    #[test]
    fn is_castling_through_check_in_check() {
        let fen_string =
            String::from("r1bqk2r/ppp2ppp/2B1pn2/8/1b6/2N2N2/PPPP1PPP/R1B1QRK1 b kq - 0 8");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(7, 4), &Field(7, 6), state.position_matrix().borrow());
        assert!(state.is_castling_through_check(&chess_move));
    }

    #[test]
    fn is_castling_through_check_through_check() {
        let fen_string =
            String::from("r2qk2r/p1p2ppp/b1p1pn2/8/8/BPP2N2/P1P2PPP/R3QRK1 b kq - 2 11");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(7, 4), &Field(7, 6), state.position_matrix().borrow());
        assert!(state.is_castling_through_check(&chess_move));
    }

    #[test]
    fn castle_white_kingside() {
        let fen_string =
            String::from("r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(0, 4), &Field(0, 6), state.position_matrix().borrow());
        let new_state = state.execute_move(&chess_move);
        new_state
            .position()
            .borrow_mut()
            .update_from_matrix(new_state.position_matrix().borrow());
        assert_eq!(
            new_state.position().borrow().0,
            "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1"
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[0][5],
            Piece {
                color: Color::White,
                piecetype: PieceType::Rook
            }
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[0][6],
            Piece {
                color: Color::White,
                piecetype: PieceType::King
            }
        );
    }

    #[test]
    fn castle_white_queenside() {
        let fen_string =
            String::from("rn1qk2r/pppbbppp/5n2/4p3/N2p4/1P1P4/PBPQPPPP/R3KBNR w KQkq - 3 7");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(0, 4), &Field(0, 2), state.position_matrix().borrow());
        let new_state = state.execute_move(&chess_move);
        new_state
            .position()
            .borrow_mut()
            .update_from_matrix(new_state.position_matrix().borrow());
        assert_eq!(
            new_state.position().borrow().0,
            "rn1qk2r/pppbbppp/5n2/4p3/N2p4/1P1P4/PBPQPPPP/2KR1BNR"
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[0][2],
            Piece {
                color: Color::White,
                piecetype: PieceType::King
            }
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[0][3],
            Piece {
                color: Color::White,
                piecetype: PieceType::Rook
            }
        );
    }

    #[test]
    fn castle_black_kingside() {
        let fen_string =
            String::from("rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(7, 4), &Field(7, 6), state.position_matrix().borrow());
        let new_state = state.execute_move(&chess_move);
        new_state
            .position()
            .borrow_mut()
            .update_from_matrix(new_state.position_matrix().borrow());
        assert_eq!(
            new_state.position().borrow().0,
            "rnbq1rk1/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1"
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[7][5],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Rook
            }
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[7][6],
            Piece {
                color: Color::Black,
                piecetype: PieceType::King
            }
        );
    }

    #[test]
    fn castle_black_queenside() {
        let fen_string =
            String::from("r3kbnr/pbpqpppp/1pnp4/8/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 b kq - 0 6");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(7, 4), &Field(7, 2), state.position_matrix().borrow());
        let new_state = state.execute_move(&chess_move);
        new_state
            .position()
            .borrow_mut()
            .update_from_matrix(new_state.position_matrix().borrow());
        assert_eq!(
            new_state.position().borrow().0,
            "2kr1bnr/pbpqpppp/1pnp4/8/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1"
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[7][2],
            Piece {
                color: Color::Black,
                piecetype: PieceType::King
            }
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[7][3],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Rook
            }
        );
    }

    #[test]
    fn en_passant_white() {
        let fen_string =
            String::from("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3");
        let state = State::load_game_from_fen(fen_string);

        // the left pawn moved the move before
        // en passant is only possible on f6
        let chess_move = Move::new(&Field(4, 4), &Field(5, 3), state.position_matrix().borrow());
        assert!(match state.is_move_legal(&chess_move) {
            Ok(_) => false,
            Err(e) if e == MoveError::PieceCantReachTarget => true,
            Err(_) => false,
        });
        let chess_move = Move::new(&Field(4, 4), &Field(5, 5), state.position_matrix().borrow());
        assert!(state.is_move_legal(&chess_move).unwrap());

        // check whether the en passant move execution works
        let new_state = state.execute_move(&chess_move);
        new_state
            .position()
            .borrow_mut()
            .update_from_matrix(new_state.position_matrix().borrow());
        assert_eq!(
            new_state.position_matrix().borrow().0[5][5],
            Piece {
                color: Color::White,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[4][5],
            Piece {
                color: Color::None,
                piecetype: PieceType::None
            }
        );
        assert_eq!(
            new_state.position().borrow().0,
            "rnbqkbnr/ppp1p1pp/5P2/3p4/8/8/PPPP1PPP/RNBQKBNR"
        );
    }

    #[test]
    fn en_passant_black() {
        let fen_string =
            String::from("rnbq1rk1/1p1pppbp/5np1/2p5/pPB1P3/2NP1N2/P1PB1PPP/R2Q1RK1 b - b3 0 8");
        let state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(3, 0), &Field(2, 1), state.position_matrix().borrow());
        assert!(state.is_move_legal(&chess_move).unwrap());
        let new_state = state.execute_move(&chess_move);
        new_state
            .position()
            .borrow_mut()
            .update_from_matrix(new_state.position_matrix().borrow());
        assert_eq!(
            new_state.position_matrix().borrow().0[2][1],
            Piece {
                color: Color::Black,
                piecetype: PieceType::Pawn
            }
        );
        assert_eq!(
            new_state.position_matrix().borrow().0[3][1],
            Piece {
                color: Color::None,
                piecetype: PieceType::None
            }
        );
        assert_eq!(
            new_state.position().borrow().0,
            "rnbq1rk1/1p1pppbp/5np1/2p5/2B1P3/1pNP1N2/P1PB1PPP/R2Q1RK1"
        );
    }

    #[test]
    fn player_has_legal_move_yes() {
        let fen_string =
            String::from("rnbq1rk1/1p1pppbp/5np1/2p5/pPB1P3/2NP1N2/P1PB1PPP/R2Q1RK1 b - b3 0 8");
        let state = State::load_game_from_fen(fen_string);
        assert!(state.player_has_legal_move());
    }

    #[test]
    fn player_has_legal_move_stalemate() {
        let fen_string = String::from("1Q6/8/8/8/3K4/8/p7/k7 b - - 0 1");
        let state = State::load_game_from_fen(fen_string);
        assert!(!state.player_has_legal_move());
    }

    #[test]
    fn player_has_legal_move_checkmate() {
        let fen_string = String::from("8/8/4kB1P/PP1p3R/6N1/8/1r6/2r3K1 w - - 0 1");
        let state = State::load_game_from_fen(fen_string);
        assert!(!state.player_has_legal_move());
    }

    #[test]
    fn castle_availability_white_king() {
        let fen_string =
            String::from("r1bq1rk1/pp1pppbp/2n2np1/2p5/2B1P3/1P3N2/PBPPQPPP/RN2K2R w KQ - 3 7");
        let mut state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(0, 7), &Field(0, 6), state.position_matrix().borrow());
        state.update_castling_availability(&chess_move);
        assert!(!state.castle_availability.white_king);
        assert!(state.castle_availability.white_queen);
        assert!(!state.castle_availability.black_king);
        assert!(!state.castle_availability.black_queen);
    }

    #[test]
    fn castle_availability_white_queen() {
        let fen_string =
            String::from("r1b2rk1/pp1pppbp/1qn2np1/2p5/2B1P3/1PN2N2/PBPPQPPP/R3K2R w KQ - 5 8");
        let mut state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(0, 0), &Field(0, 1), state.position_matrix().borrow());
        state.update_castling_availability(&chess_move);
        assert!(state.castle_availability.white_king);
        assert!(!state.castle_availability.white_queen);
        assert!(!state.castle_availability.black_king);
        assert!(!state.castle_availability.black_queen);
    }

    #[test]
    fn castle_availability_white_both() {
        let fen_string =
            String::from("rnbqk1nr/pppp1ppp/8/2b1p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 2 3");
        let mut state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(0, 4), &Field(1, 4), state.position_matrix().borrow());
        state.update_castling_availability(&chess_move);
        assert!(!state.castle_availability.white_king);
        assert!(!state.castle_availability.white_queen);
        assert!(state.castle_availability.black_king);
        assert!(state.castle_availability.black_queen);
    }

    #[test]
    fn castle_availability_black_king() {
        let fen_string =
            String::from("rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4");
        let mut state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(7, 7), &Field(7, 5), state.position_matrix().borrow());
        state.update_castling_availability(&chess_move);
        assert!(!state.castle_availability.white_king);
        assert!(!state.castle_availability.white_queen);
        assert!(!state.castle_availability.black_king);
        assert!(state.castle_availability.black_queen);
    }

    #[test]
    fn castle_availability_black_queen() {
        let fen_string =
            String::from("r3kbnr/pbqppppp/1pn5/2p5/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 b kq - 0 6");
        let mut state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(7, 0), &Field(7, 1), state.position_matrix().borrow());
        state.update_castling_availability(&chess_move);
        assert!(!state.castle_availability.white_king);
        assert!(!state.castle_availability.white_queen);
        assert!(state.castle_availability.black_king);
        assert!(!state.castle_availability.black_queen);
    }

    #[test]
    fn castle_availability_black_both() {
        let fen_string =
            String::from("r3kbnr/pbqppppp/1pn5/2p5/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 b kq - 0 6");
        let mut state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(7, 4), &Field(7, 3), state.position_matrix().borrow());
        state.update_castling_availability(&chess_move);
        assert!(!state.castle_availability.white_king);
        assert!(!state.castle_availability.white_queen);
        assert!(!state.castle_availability.black_king);
        assert!(!state.castle_availability.black_queen);
    }

    #[test]
    fn update_en_passant_white() {
        let fen_string =
            String::from("r1b2rk1/pp2ppb1/1qnp1npp/2p5/2B1P3/2N1QN2/PPPP1PPP/R1BR3K w - - 4 10");
        let mut state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(1, 3), &Field(3, 3), state.position_matrix().borrow());
        state.update_en_passant(&chess_move);
        assert_eq!(state.en_passant, Some(Field(2, 3)));
    }

    #[test]
    fn update_en_passant_black() {
        let fen_string =
            String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        let mut state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(6, 2), &Field(4, 2), state.position_matrix().borrow());
        state.update_en_passant(&chess_move);
        assert_eq!(state.en_passant, Some(Field(5, 2)));
    }

    #[test]
    fn update_en_passant_none() {
        let fen_string =
            String::from("r1b2rk1/pp2ppb1/1qnp1npp/2p5/2BPP3/2N1QN2/PPP2PPP/R1BR3K b - d3 0 10");
        let mut state = State::load_game_from_fen(fen_string);
        let chess_move = Move::new(&Field(7, 2), &Field(3, 6), state.position_matrix().borrow());
        state.update_en_passant(&chess_move);
        assert_eq!(state.en_passant, None);
    }

    #[test]
    fn update_fullmove() {
        let fen_string =
            String::from("r1bqk2r/pp1pnpbp/2n1p1p1/8/2BPP3/5N2/PP3PPP/RNBQ1RK1 w kq - 1 8");
        let mut state = State::load_game_from_fen(fen_string);
        assert_eq!(state.fullmove_clock, 8);
        let chess_move = Move::new(&Field(0, 1), &Field(2, 2), state.position_matrix().borrow());
        state = state.execute_move(&chess_move);
        state
            .position()
            .borrow_mut()
            .update_from_matrix(state.position_matrix().borrow());
        assert_eq!(state.fullmove_clock, 8);
        let chess_move = Move::new(&Field(7, 4), &Field(7, 6), state.position_matrix().borrow());
        state = state.execute_move(&chess_move);
        state
            .position()
            .borrow_mut()
            .update_from_matrix(state.position_matrix().borrow());
        assert_eq!(state.fullmove_clock, 9);
    }

    #[test]
    fn update_halfmove_add() {
        let fen_string =
            String::from("r1bq1rk1/pp1pnpbp/2n1p1p1/8/2BPP3/2N2N2/PP3PPP/R1BQ1RK1 w - - 3 9");
        let mut state = State::load_game_from_fen(fen_string);
        assert_eq!(state.halfmove_clock, 3);
        let chess_move = Move::new(&Field(0, 2), &Field(4, 6), state.position_matrix().borrow());
        state = state.execute_move(&chess_move);
        state
            .position()
            .borrow_mut()
            .update_from_matrix(state.position_matrix().borrow());
        assert_eq!(state.halfmove_clock, 4);
    }

    #[test]
    fn update_fullmove_reset_pawn() {
        let fen_string =
            String::from("r1bq1rk1/pp1pnpbp/2n1p1p1/6B1/2BPP3/2N2N2/PP3PPP/R2Q1RK1 b - - 4 9");
        let mut state = State::load_game_from_fen(fen_string);
        assert_eq!(state.halfmove_clock, 4);
        let chess_move = Move::new(&Field(6, 0), &Field(5, 0), state.position_matrix().borrow());
        state = state.execute_move(&chess_move);
        state
            .position()
            .borrow_mut()
            .update_from_matrix(state.position_matrix().borrow());
        assert_eq!(state.halfmove_clock, 0);
    }

    #[test]
    fn update_fullmove_reset_capture() {
        let fen_string =
            String::from("r1bqr1k1/3pnpbp/p1n1p1p1/1p4B1/3PP3/P1N2N2/BP3PPP/R2Q1RK1 w - - 2 12");
        let mut state = State::load_game_from_fen(fen_string);
        assert_eq!(state.halfmove_clock, 2);
        let chess_move = Move::new(&Field(4, 6), &Field(6, 4), state.position_matrix().borrow());
        state = state.execute_move(&chess_move);
        state
            .position()
            .borrow_mut()
            .update_from_matrix(state.position_matrix().borrow());
        assert_eq!(state.halfmove_clock, 0);
    }

    // M1
    // 8/8/4kB1P/PP1p3R/6N1/2r5/1r6/6K1 b - - 0 1

    // Stalemate 1
    // 8/2Q5/8/8/3K4/8/p7/k7 w - - 0 1
}
