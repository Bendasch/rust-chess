use crate::library::game::*;
use std::{collections::LinkedList, fmt, io};

// somehow the black ascii chess pieces look like white and vice versa...
// depending on the console, they made need to be swapped (again)
const WHITE_ROOK: char = '\u{265C}';
const WHITE_KNIGHT: char = '\u{265E}';
const WHITE_BISHOP: char = '\u{265D}';
const WHITE_QUEEN: char = '\u{265B}';
const WHITE_KING: char = '\u{265A}';
const WHITE_PAWN: char = '\u{265F}';
const BLACK_ROOK: char = '\u{2656}';
const BLACK_KNIGHT: char = '\u{2658}';
const BLACK_BISHOP: char = '\u{2657}';
const BLACK_QUEEN: char = '\u{2655}';
const BLACK_KING: char = '\u{2654}';
const BLACK_PAWN: char = '\u{2659}';

/*
    This is the main function for the CLI version of the game.

    The game is a linked list of board states (one for each half-move). It can be initialized using a FEN string
    (e.g., "rn1qk2r/pppbbppp/5n2/4p3/N2p4/1P1P4/PBPQPPPP/R3KBNR w KQkq - 3 7") if a specific position should be
    loaded. Otherwise a new game is created. In both cases, the game starts as a list containing one board state.

    The game can be played by the user by entering a move in the form:
        <start row index><start column index><target row index><target column index>
    For example, the starting move "e4" would be entered as "2545". In the future parsing of moves in more intuitive
    notation may be supported.
*/
pub fn run(fen: Option<String>) {
    // start from a provided or an initial position
    let mut game: LinkedList<State> = LinkedList::new();
    game.push_back(State::new(fen));

    loop {
        let current_state = game.back().unwrap();

        // start by showing the current board state
        draw_board(current_state.position().borrow());

        // who to move?
        match current_state.check_game_over() {
            GameOver::BlackWon => {
                println!("Checkmate, black won!");
                return;
            }
            GameOver::WhiteWon => {
                println!("Checkmate, white won!");
                return;
            }
            GameOver::Stalemate => {
                println!("Stalemate!");
                return;
            }

            _ => draw_who_to_move(current_state.turn()),
        }

        // get input from player
        let mut move_string = String::new();
        io::stdin().read_line(&mut move_string).unwrap();

        // execute the move according to the players input.
        // the resulting state is appended to the game list.
        let new_state = State::perform_turn_from_input(move_string, current_state);
        drop(current_state);
        game.push_back(new_state);
    }
}

fn draw_board(position: Ref<Position>) {
    let split: Vec<&str> = position.split();

    println!("\n   1 2 3 4 5 6 7 8");
    println!("  -----------------");

    for (i, line) in split.iter().enumerate() {
        let mut output_rank = String::new();
        let rank_index = char::from_digit(8 - i as u32, 10).unwrap();

        output_rank.push(rank_index);
        output_rank.push_str("| ");

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
                '2' => output_rank.push_str("   "),
                '3' => output_rank.push_str("     "),
                '4' => output_rank.push_str("       "),
                '5' => output_rank.push_str("         "),
                '6' => output_rank.push_str("           "),
                '7' => output_rank.push_str("             "),
                '8' => output_rank.push_str("               "),
                _ => panic!("Board position is corrupt!"),
            }
            output_rank.push(' ');
        }

        output_rank.push_str(" |");
        output_rank.push(rank_index);

        println!("{}", output_rank);
    }

    println!("  -----------------");
    println!("   1 2 3 4 5 6 7 8");
}

fn draw_who_to_move(turn: &Color) {
    match *turn {
        Color::Black => println!("Black to move..."),
        Color::White => println!("White to move..."),
        _ => {}
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self.color() {
            Color::Black => "black",
            Color::White => "white",
            Color::None => panic!("Corrupt game state - each player needs a color!"),
        };

        match self.piecetype() {
            PieceType::Rook => write!(f, "{} rook", color),
            PieceType::Knight => write!(f, "{} knight", color),
            PieceType::Bishop => write!(f, "{} bishop", color),
            PieceType::Queen => write!(f, "{} queen", color),
            PieceType::King => write!(f, "{} king", color),
            PieceType::Pawn => write!(f, "{} pawn", color),
            PieceType::None => panic!("A piece needs to be selected to move!"),
        }
    }
}
