use std::io;
use std::time;

use crate::core;

pub trait PlaygroundUtils
where
    Self: core::Game,
{
    fn strategies(&self) -> Vec<Box<dyn core::Strategy<Self>>>;
    fn serialize_state(&self, state: &<Self as core::Game>::State) -> String;
}

pub trait PlaygroundGame {
    fn name(&self) -> String;
    fn start(&self) -> core::MatchResult;
}

fn select_strategy<T: core::Game + PlaygroundUtils>(game: &T) -> Box<dyn core::Strategy<T>> {
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
where
    T: core::Game + PlaygroundUtils,
{
    fn name(&self) -> String {
        return self.name();
    }

    fn start(&self) -> core::MatchResult {
        println!("Select strategy for player 1");
        let p1_strat = select_strategy(self);
        println!("Select strategy for player 2");
        let p2_strat = select_strategy(self);

        let result = simulate(self, &(*p1_strat), &(*p2_strat), true);

        println!("{}", result);

        return result;
    }
}

pub fn simulate<G: core::Game + PlaygroundUtils>(
    game: &G,
    p1_strat: &dyn core::Strategy<G>,
    p2_strat: &dyn core::Strategy<G>,
    debug: bool,
) -> core::MatchResult {
    let mut state = game.init();

    let mut num_moves = 0;
    let mut player1_time = time::Duration::new(0, 0);
    let mut player2_time = time::Duration::new(0, 0);

    while game.status(&state) == core::GameStatus::InProgress {
        if debug {
            println!("{}", game.serialize_state(&state));
        }

        let start = time::Instant::now();

        let action = match game.player(&state) {
            core::Player::Player1 => {
                if debug {
                    println!("Player 1's turn...");
                }
                let action = p1_strat.select_action(game, &state);
                player1_time = player1_time + (start.elapsed());
                action
            }
            core::Player::Player2 => {
                if debug {
                    println!("Player 2's turn...");
                }
                let action = p2_strat.select_action(game, &state);
                player2_time = player2_time + (start.elapsed());
                num_moves = num_moves + 1;
                action
            }
        };

        state = game.play(&action, &state);
    }

    return core::MatchResult {
        status: game.status(&state),
        num_moves: num_moves,
        player1_time: player1_time,
        player2_time: player2_time,
    };
}
