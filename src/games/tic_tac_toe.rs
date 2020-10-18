use std::fmt;
use std::io;

use crate::core;
use crate::playground;
use crate::strategy;

pub struct TicTacToe {}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TicTacToeCell {
    X,
    O,
    Empty,
}

impl fmt::Display for TicTacToeCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TicTacToeCell::X => write!(f, "X"),
            TicTacToeCell::O => write!(f, "O"),
            TicTacToeCell::Empty => write!(f, " "),
        }
    }
}

#[derive(Clone)]
pub struct TicTacToeState {
    pub board: [TicTacToeCell; 9],
    pub player: core::Player,
}

#[derive(PartialEq, Eq)]
pub struct TicTacToeAction {
    pub cell: u8,
}

impl TicTacToe {
    fn cell_to_str(&self, state: &TicTacToeState, idx: usize) -> String {
        match state.board[idx] {
            TicTacToeCell::Empty => return format!("{}", idx),
            _ => return format!("{}", state.board[idx]),
        }
    }
}

impl core::Game for TicTacToe {
    type State = TicTacToeState;
    type Action = TicTacToeAction;

    fn name(&self) -> String {
        return "Tic-Tac-Toe".to_string();
    }

    fn init(&self) -> Self::State {
        let state = Self::State {
            board: [TicTacToeCell::Empty; 9],
            player: core::Player::Player1,
        };

        return state;
    }

    fn player(&self, state: &Self::State) -> core::Player {
        return state.player.clone();
    }

    fn actions(&self, state: &Self::State) -> Vec<Self::Action> {
        let mut actions: Vec<Self::Action> = Vec::new();

        for idx in 0..9 {
            if state.board[idx] == TicTacToeCell::Empty {
                actions.push(TicTacToeAction { cell: idx as u8 });
            }
        }

        return actions;
    }

    fn play(&self, action: &Self::Action, state: &Self::State) -> Self::State {
        let val = if state.player == core::Player::Player1 {
            TicTacToeCell::X
        } else {
            TicTacToeCell::O
        };
        let mut new_state = state.clone();
        new_state.board[action.cell as usize] = val;
        new_state.player = if state.player == core::Player::Player1 {
            core::Player::Player2
        } else {
            core::Player::Player1
        };

        return new_state;
    }

    fn status(&self, state: &Self::State) -> core::GameStatus {
        // Check for wins for either player
        let wins = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for win in wins.iter() {
            let v0 = state.board[win[0]];

            if state.board[win[1]] == v0 && state.board[win[2]] == v0 {
                match v0 {
                    TicTacToeCell::X => return core::GameStatus::Player1Win,
                    TicTacToeCell::O => return core::GameStatus::Player2Win,
                    TicTacToeCell::Empty => (),
                }
            }
        }

        let mut draw = true;

        // If there are no empty on the board, then it's a draw
        for val in state.board.iter() {
            match val {
                TicTacToeCell::Empty => {
                    draw = false;
                    break;
                }
                _ => (),
            }
        }

        return if draw {
            core::GameStatus::Draw
        } else {
            core::GameStatus::InProgress
        };
    }
}

pub struct TicTacToeParser {}

impl core::ActionParser for TicTacToeParser {
    type Game = TicTacToe;

    fn read_action(&self) -> <TicTacToe as core::Game>::Action {
        println!("Enter cell [0, 9]:");

        let mut cell_str = String::new();
        io::stdin()
            .read_line(&mut cell_str)
            .expect("Failed to read line");

        let cell_idx: u8 = cell_str.trim().parse().expect("Expected number");

        return TicTacToeAction { cell: cell_idx };
    }
}

impl playground::PlaygroundUtils for TicTacToe {
    fn strategies(&self) -> Vec<Box<dyn core::Strategy<Self>>> {
        return vec![
            Box::new(strategy::HumanStrategy {
                parser: TicTacToeParser {},
            }),
            Box::new(strategy::RandomStrategy {}),
        ];
    }

    fn serialize_state(&self, state: &TicTacToeState) -> String {
        return format!(
            "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}",
            self.cell_to_str(state, 0),
            self.cell_to_str(state, 1),
            self.cell_to_str(state, 2),
            self.cell_to_str(state, 3),
            self.cell_to_str(state, 4),
            self.cell_to_str(state, 5),
            self.cell_to_str(state, 6),
            self.cell_to_str(state, 7),
            self.cell_to_str(state, 8)
        );
    }
}
