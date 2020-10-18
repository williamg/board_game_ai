use crate::core;
use crate::games;

extern crate uci;

pub struct UCIStrategy {
    engine: uci::Engine,
}

impl UCIStrategy {
    pub fn new() -> UCIStrategy {
        let engine = uci::Engine::new("stockfish").unwrap();

        return UCIStrategy { engine: engine };
    }
}

impl core::Strategy<games::Chess> for UCIStrategy {
    fn name(&self) -> String {
        return "UCI".to_string();
    }

    fn select_action(&self, _game: &games::Chess, state: &games::ChessState) -> games::ChessAction {
        let fen = format!("{}", state.board);
        self.engine
            .set_position(&fen)
            .expect("Failed to set position");

        let bestmove = self.engine.bestmove().expect("Failed to get move");
        return games::action_from_string(&bestmove).unwrap();
    }
}
