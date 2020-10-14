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
    fn name(&self) -> String;
    fn init(&self) -> Self::State;
    fn player(&self, state: &Self::State) -> Player;
    fn actions(&self, state: &Self::State) -> Vec<Self::Action>;
    fn play(&self, action: &Self::Action, state: &Self::State) -> Self::State;
    fn status(&self, state: &Self::State) -> GameStatus;
}

pub trait Strategy<G: Game> {
    fn name(&self) -> String;
    fn select_action(
        &self,
        game: &G,
        state: &G::State) -> G::Action;
}

pub struct MatchResult {
    pub status: GameStatus,
    pub num_moves: u32
}

pub trait ActionParser {
    type Game : Game;
    fn read_action(&self) -> <Self::Game as Game>::Action;
}
