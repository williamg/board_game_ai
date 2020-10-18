use std::io;
use std::time;

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

        let mut num_moves = 0;
        let mut player1_time = time::Duration::new(0, 0);
        let mut player2_time = time::Duration::new(0, 0);

        while self.status (&state) == core::GameStatus::InProgress {
            println!("{}", self.serialize_state(&state));

            let start = time::Instant::now();

            let action = match self.player(&state) {
                core::Player::Player1 => {
                    println!("Player 1's turn...");
                    let action = p1_strat.select_action(self, &state);
                    player1_time = player1_time + (start.elapsed());
                    action
                },
                core::Player::Player2 => {
                    println!("Player 2's turn...");
                    let action = p2_strat.select_action(self, &state);
                    player2_time = player1_time + (start.elapsed());
                    num_moves = num_moves + 1;
                    action
                }
            };

            state = self.play(&action, &state);
        }

        let result = core::MatchResult {
            status: self.status(&state),
            num_moves: num_moves,
            player1_time: player1_time,
            player2_time: player2_time
        };

        println!("{}", result);

        return result;
    }
}
