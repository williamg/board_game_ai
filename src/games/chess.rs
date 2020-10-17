use chess::{Board, ChessMove, Color, MoveGen, Square, Piece, BoardStatus};

use crate::core;
use crate::playground;
use crate::strategy;

use std::io;

pub struct Chess {
}

#[derive(Clone)]
pub struct ChessState {
    pub board: Board,
    pub num_moves: u8
}

#[derive(PartialEq,Eq)]
pub struct ChessAction {
    pub chess_move: ChessMove
}

impl core::Game for Chess {
    type State = ChessState;
    type Action = ChessAction;

    fn name(&self) -> String {
        return "Chess".to_string();
    }

    fn init(&self) -> Self::State {
        let state = Self::State {
            board: Board::default(),
            num_moves: 0
        };

        return state;
    }

    fn player(&self, state: &Self::State) -> core::Player {
        return if state.board.side_to_move () == Color::White
            { core::Player::Player1 } else { core::Player::Player2 };
    }

    fn actions(&self, state: &Self::State) -> Vec<Self::Action> {
        let movegen = MoveGen::new_legal(&state.board);

        let mut actions: Vec<Self::Action> = Vec::new();

        for chess_move in movegen {
            actions.push (ChessAction { chess_move: chess_move });
        }

        return actions;
    }

    fn play(&self, action: &Self::Action, state: &Self::State) -> Self::State {
        let new_board = state.board.make_move_new(action.chess_move);

        return ChessState {
            board: new_board,
            num_moves: state.num_moves + (self.player(state) == core::Player::Player2) as u8
        };
    }

    fn status(&self, state: &Self::State) -> core::GameStatus {
        return match state.board.status() {
            BoardStatus::Ongoing => if state.num_moves > 50
                { core::GameStatus::Draw } else {core::GameStatus::InProgress},
            BoardStatus::Stalemate => core::GameStatus::Draw,
            BoardStatus::Checkmate => if state.board.side_to_move () == Color::White
            { core::GameStatus::Player2Win } else { core::GameStatus::Player1Win }
        }
    }
}

pub struct ChessParser {}

impl core::ActionParser for ChessParser {
    type Game = Chess;

    fn read_action(&self) -> <Chess as core::Game>::Action {
        loop {
            println!("Enter [src][dst][promo?] [e.g. e3e4, a7a8q]");

            let mut move_str = String::new();
            io::stdin()
                .read_line (&mut move_str)
                .expect("Failed to read line");

            let bytes = move_str.into_bytes();

            if bytes.len() < 4 || bytes.len() > 5 {
                continue;
            }

            let src_col = bytes[0];
            let src_row = bytes[1];
            let dst_col = bytes[2];
            let dst_row = bytes[3];

            fn to_square(col: u8, row: u8) -> Option<Square> {
                match (col as char, row as char) {
                    ('a', '1') => Some(Square::A1),
                    ('a', '2') => Some(Square::A2),
                    ('a', '3') => Some(Square::A3),
                    ('a', '4') => Some(Square::A4),
                    ('a', '5') => Some(Square::A5),
                    ('a', '6') => Some(Square::A6),
                    ('a', '7') => Some(Square::A7),
                    ('a', '8') => Some(Square::A8),
                    ('b', '1') => Some(Square::B1),
                    ('b', '2') => Some(Square::B2),
                    ('b', '3') => Some(Square::B3),
                    ('b', '4') => Some(Square::B4),
                    ('b', '5') => Some(Square::B5),
                    ('b', '6') => Some(Square::B6),
                    ('b', '7') => Some(Square::B7),
                    ('b', '8') => Some(Square::B8),
                    ('c', '1') => Some(Square::C1),
                    ('c', '2') => Some(Square::C2),
                    ('c', '3') => Some(Square::C3),
                    ('c', '4') => Some(Square::C4),
                    ('c', '5') => Some(Square::C5),
                    ('c', '6') => Some(Square::C6),
                    ('c', '7') => Some(Square::C7),
                    ('c', '8') => Some(Square::C8),
                    ('d', '1') => Some(Square::D1),
                    ('d', '2') => Some(Square::D2),
                    ('d', '3') => Some(Square::D3),
                    ('d', '4') => Some(Square::D4),
                    ('d', '5') => Some(Square::D5),
                    ('d', '6') => Some(Square::D6),
                    ('d', '7') => Some(Square::D7),
                    ('d', '8') => Some(Square::D8),
                    ('e', '1') => Some(Square::E1),
                    ('e', '2') => Some(Square::E2),
                    ('e', '3') => Some(Square::E3),
                    ('e', '4') => Some(Square::E4),
                    ('e', '5') => Some(Square::E5),
                    ('e', '6') => Some(Square::E6),
                    ('e', '7') => Some(Square::E7),
                    ('e', '8') => Some(Square::E8),
                    ('f', '1') => Some(Square::F1),
                    ('f', '2') => Some(Square::F2),
                    ('f', '3') => Some(Square::F3),
                    ('f', '4') => Some(Square::F4),
                    ('f', '5') => Some(Square::F5),
                    ('f', '6') => Some(Square::F6),
                    ('f', '7') => Some(Square::F7),
                    ('f', '8') => Some(Square::F8),
                    ('g', '1') => Some(Square::G1),
                    ('g', '2') => Some(Square::G2),
                    ('g', '3') => Some(Square::G3),
                    ('g', '4') => Some(Square::G4),
                    ('g', '5') => Some(Square::G5),
                    ('g', '6') => Some(Square::G6),
                    ('g', '7') => Some(Square::G7),
                    ('g', '8') => Some(Square::G8),
                    ('h', '1') => Some(Square::H1),
                    ('h', '2') => Some(Square::H2),
                    ('h', '3') => Some(Square::H3),
                    ('h', '4') => Some(Square::H4),
                    ('h', '5') => Some(Square::H5),
                    ('h', '6') => Some(Square::H6),
                    ('h', '7') => Some(Square::H7),
                    ('h', '8') => Some(Square::H8),
                    _ => None
                }
            }

            let src_square = to_square(src_col, src_row);
            let dst_square = to_square(dst_col, dst_row);

            if src_square == None || dst_square == None {
                continue;
            }

            let mut promo = None;

            if bytes.len() == 5 {
                promo = match bytes[4] as char {
                   'K' => Some(Piece::King),
                   'Q' => Some(Piece::Queen),
                   'R' => Some(Piece::Rook),
                   'N' => Some(Piece::Knight),
                    _ => None
                }
            }

            return ChessAction {
                chess_move: ChessMove::new(src_square.unwrap(), dst_square.unwrap(), promo)
            };
        }
    }
}

impl playground::PlaygroundUtils for Chess {
    fn strategies(&self) -> Vec<Box<dyn core::Strategy<Self>>> {
        return vec![
            Box::new(strategy::HumanStrategy { parser: ChessParser {} }),
            Box::new(strategy::RandomStrategy {})
        ];
    }

    fn serialize_state(&self, state: &ChessState) -> String {
        fn piece_string(board: &Board, square: Square) -> String{
            return match board.piece_on(square) {
                None => "_".to_string(),
                Some(p) => {
                    let piece_str = match p {
                        Piece::Pawn => "p",
                        Piece::Rook => "r",
                        Piece::Bishop => "b",
                        Piece::Knight => "n",
                        Piece::Queen => "q",
                        Piece::King => "k",
                    };

                    if board.color_on(square).unwrap() == Color::White {
                        piece_str.to_uppercase() } else { piece_str.to_string() }
                }
            };
        }

        return format!("
            a b c d e f g h  \n
          1|{}|{}|{}|{}|{}|{}|{}|{}|1\n
          2|{}|{}|{}|{}|{}|{}|{}|{}|2\n
          3|{}|{}|{}|{}|{}|{}|{}|{}|3\n
          4|{}|{}|{}|{}|{}|{}|{}|{}|4\n
          5|{}|{}|{}|{}|{}|{}|{}|{}|5\n
          6|{}|{}|{}|{}|{}|{}|{}|{}|6\n
          7|{}|{}|{}|{}|{}|{}|{}|{}|7\n
          8|{}|{}|{}|{}|{}|{}|{}|{}|8\n
            a b c d e f g h  \n",
          piece_string(&state.board, Square::A1),
          piece_string(&state.board, Square::B1),
          piece_string(&state.board, Square::C1),
          piece_string(&state.board, Square::D1),
          piece_string(&state.board, Square::E1),
          piece_string(&state.board, Square::F1),
          piece_string(&state.board, Square::G1),
          piece_string(&state.board, Square::H1),
          piece_string(&state.board, Square::A2),
          piece_string(&state.board, Square::B2),
          piece_string(&state.board, Square::C2),
          piece_string(&state.board, Square::D2),
          piece_string(&state.board, Square::E2),
          piece_string(&state.board, Square::F2),
          piece_string(&state.board, Square::G2),
          piece_string(&state.board, Square::H2),
          piece_string(&state.board, Square::A3),
          piece_string(&state.board, Square::B3),
          piece_string(&state.board, Square::C3),
          piece_string(&state.board, Square::D3),
          piece_string(&state.board, Square::E3),
          piece_string(&state.board, Square::F3),
          piece_string(&state.board, Square::G3),
          piece_string(&state.board, Square::H3),
          piece_string(&state.board, Square::A4),
          piece_string(&state.board, Square::B4),
          piece_string(&state.board, Square::C4),
          piece_string(&state.board, Square::D4),
          piece_string(&state.board, Square::E4),
          piece_string(&state.board, Square::F4),
          piece_string(&state.board, Square::G4),
          piece_string(&state.board, Square::H4),
          piece_string(&state.board, Square::A5),
          piece_string(&state.board, Square::B5),
          piece_string(&state.board, Square::C5),
          piece_string(&state.board, Square::D5),
          piece_string(&state.board, Square::E5),
          piece_string(&state.board, Square::F5),
          piece_string(&state.board, Square::G5),
          piece_string(&state.board, Square::H5),
          piece_string(&state.board, Square::A6),
          piece_string(&state.board, Square::B6),
          piece_string(&state.board, Square::C6),
          piece_string(&state.board, Square::D6),
          piece_string(&state.board, Square::E6),
          piece_string(&state.board, Square::F6),
          piece_string(&state.board, Square::G6),
          piece_string(&state.board, Square::H6),
          piece_string(&state.board, Square::A7),
          piece_string(&state.board, Square::B7),
          piece_string(&state.board, Square::C7),
          piece_string(&state.board, Square::D7),
          piece_string(&state.board, Square::E7),
          piece_string(&state.board, Square::F7),
          piece_string(&state.board, Square::G7),
          piece_string(&state.board, Square::H7),
          piece_string(&state.board, Square::A8),
          piece_string(&state.board, Square::B8),
          piece_string(&state.board, Square::C8),
          piece_string(&state.board, Square::D8),
          piece_string(&state.board, Square::E8),
          piece_string(&state.board, Square::F8),
          piece_string(&state.board, Square::G8),
          piece_string(&state.board, Square::H8),

        );
    }
}
