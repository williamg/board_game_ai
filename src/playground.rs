use std::io;
use crate::core;

pub trait PlaygroundUtils where Self : core::Game {
    fn strategies(&self) -> Vec<Box<dyn core::Strategy<Self>>>;
    fn serialize_state(&self, state: &<Self as core::Game>::State) -> String;
}

pub trait PlaygroundGame {
    fn name(&self) -> String;
    fn start(&self) -> core::MatchResult;
}

fn select_strategy<T : core::Game + PlaygroundUtils> (game: &T) -> Box<dyn core::Strategy<T>> {
    let mut all_strats = game.strategies();
    let mut strat_idx = all_strats.len();

    while strat_idx >= all_strats.len() {
        let mut idx = 1;
        for strat in all_strats.iter() {
            println!("{}) {}", idx, strat.name());
            idx += 1;
        }

        let mut strat_str = String::new();
        io::stdin()
            .read_line(&mut strat_str)
            .expect("Failed to read line");

        strat_idx = strat_str.trim().parse::<usize>().expect("Expected number") - 1;
    }

    return all_strats.remove(strat_idx);
}

impl<T> PlaygroundGame for T
    where T: core::Game + PlaygroundUtils
{
    fn name(&self) -> String {
        return self.name();
    }

    fn start(&self) -> core::MatchResult {
        println!("Select strategy for player 1");
        let p1_strat = select_strategy(self);
        println!("Select strategy for player 2");
        let p2_strat = select_strategy(self);

        let mut state = self.init();

        while self.status (&state) == core::GameStatus::InProgress {
            println!("{}", self.serialize_state(&state));

            let action = match self.player(&state) {
                core::Player::Player1 => {
                    println!("Player 1's turn...");
                    p1_strat.select_action(self, &state)
                },
                core::Player::Player2 => {
                    println!("Player 2's turn...");
                    p2_strat.select_action(self, &state)
                }
            };

            state = self.play(&action, &state);
        }

        match self.status (&state) {
            core::GameStatus::Player1Win => println!("Player 1 wins!"),
            core::GameStatus::Player2Win => println!("Player 2 wins!"),
            core::GameStatus::Draw => println!("Draw"),
            core::GameStatus::InProgress => ()
        }

        println!("{}", self.serialize_state(&state));

        return core::MatchResult {
            status: self.status(&state),
            num_moves: 0
        };
    }
}
