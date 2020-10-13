#[derive(PartialEq, Eq, Clone)]
pub enum Player {
    Player1,
    Player2
}

#[derive(PartialEq, Eq, Clone)]
pub enum GameStatus {
    Player1Win,
    Player2Win,
    Draw,
    InProgress
}

pub trait Game {
    type State;
    type Action;

    // Basic game functions
    fn init(&self) -> Self::State;
    fn player(&self, state: &Self::State) -> Player;
    fn actions(&self, state: &Self::State) -> Vec<Self::Action>;
    fn play(&self, action: &Self::Action, state: &Self::State) -> Self::State;
    fn status(&self, state: &Self::State) -> GameStatus;
}

pub trait ConsoleUI
where
    Self::Game: Game
{
    type Game;

    fn read_action(&self) -> <Self::Game as Game>::Action;
    fn serialize_state(&self, state: &<Self::Game as Game>::State) -> String;
}
