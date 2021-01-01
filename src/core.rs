use std::fmt;
use std::time;

use serde_json::Value;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Player {
    Player1,
    Player2,
}

pub fn other_player(player: Player) -> Player {
    match player {
        Player::Player1 => return Player::Player2,
        Player::Player2 => return Player::Player1,
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum GameStatus {
    Player1Win,
    Player2Win,
    Draw,
    InProgress,
}

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GameStatus::Player1Win => write!(f, "Player 1 Win"),
            GameStatus::Player2Win => write!(f, "Player 2 Win"),
            GameStatus::Draw => write!(f, "Draw"),
            GameStatus::InProgress => write!(f, "In Progress"),
        }
    }
}

pub trait Game {
    type State: Clone;
    type Action: Clone + Eq;

    // Basic game functions
    fn name(&self) -> String;
    fn init(&self) -> Self::State;
    fn player(&self, state: &Self::State) -> Player;
    fn actions(&self, state: &Self::State) -> Vec<Self::Action>;
    fn play(&self, action: &Self::Action, state: &Self::State) -> Self::State;
    fn status(&self, state: &Self::State) -> GameStatus;
}

pub trait Strategy<G: Game> {
    fn name(&self) -> String;
    fn select_action(&self, game: &G, state: &G::State) -> G::Action;
    fn configure(&self, _conf: &Value) {}
}

pub struct MatchResult {
    pub status: GameStatus,
    pub num_moves: u32,
    pub player1_time: time::Duration,
    pub player2_time: time::Duration,
}

impl fmt::Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Match result:\t{}\n# Moves:\t{}\nPlayer 1 Time:\t{}\nPlayer 2 Time:\t{}",
            self.status,
            self.num_moves,
            self.player1_time.as_millis(),
            self.player2_time.as_millis()
        )
    }
}

pub trait ActionParser {
    type Game: Game;
    fn read_action(&self) -> <Self::Game as Game>::Action;
}
