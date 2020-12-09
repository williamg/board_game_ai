use serde_json::Value;

use crate::core::{Game, GameStatus, Strategy};
use crate::playground::{simulate, PlaygroundUtils};

use std::fmt;

pub struct PlayerStats {
    avg_move_time: u128,
    avg_win_move_count: u64,
    num_wins: u64,
}

impl PlayerStats {
    pub fn new() -> PlayerStats {
        return PlayerStats {
            avg_move_time: 0,
            avg_win_move_count: 0,
            num_wins: 0,
        };
    }
}

pub struct BenchmarkResult {
    match_count: u64,
    p1_stats: PlayerStats,
    p2_stats: PlayerStats,
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Match count:\t{}\nPlayer 1:\n\tAvg Move Time:\t{}\n\tAvg Win Moves:\t{}\n\tNum Wins:\t{}\nPlayer 2:\n\tAvg Move Time:\t{}\n\tAvg Win Moves:\t{}\n\tNum Wins:\t{}\n",
            self.match_count,
            self.p1_stats.avg_move_time,
            self.p1_stats.avg_win_move_count,
            self.p1_stats.num_wins,
            self.p2_stats.avg_move_time,
            self.p2_stats.avg_win_move_count,
            self.p2_stats.num_wins)
    }
}

fn select_strategy<T: Game + PlaygroundUtils>(
    game: &T,
    name: &str,
) -> Option<Box<dyn Strategy<T>>> {
    let mut all_strats = game.strategies();

    for i in 0..all_strats.len() {
        if all_strats[i].name() == name {
            return Some(all_strats.remove(i));
        }
    }

    return None;
}

pub trait BenchmarkGame {
    fn run(&self, test: &Test) -> Result<BenchmarkResult, &'static str>;
}

pub struct Test {
    pub label: String,
    pub game: Box<dyn BenchmarkGame>,
    pub p1_strat_conf: Value,
    pub p2_strat_conf: Value,
    pub iterations: u64,
}

impl<T> BenchmarkGame for T
where
    T: Game + PlaygroundUtils,
{
    fn run(&self, test: &Test) -> Result<BenchmarkResult, &'static str> {
        let p1_strat = match &test.p1_strat_conf["name"] {
            Value::String(s) => select_strategy(self, &s).expect("No matching strategy"),
            _ => return Err("name must be a string"),
        };
        let p2_strat = match &test.p2_strat_conf["name"] {
            Value::String(s) => select_strategy(self, &s).expect("No matching strategy"),
            _ => return Err("name must be a string"),
        };

        p1_strat.configure(&test.p1_strat_conf);
        p2_strat.configure(&test.p2_strat_conf);

        let mut p1_stats = PlayerStats::new();
        let mut p2_stats = PlayerStats::new();

        for iter in 0..test.iterations {
            let result = simulate(self, &(*p1_strat), &(*p2_strat), false);
            let p1_avg_move_time = result.player1_time.as_millis() / result.num_moves as u128;
            let p2_avg_move_time = result.player2_time.as_millis() / result.num_moves as u128;

            let i = iter as u128;
            p1_stats.avg_move_time = ((p1_stats.avg_move_time * i) + p1_avg_move_time) / (i + 1);
            p2_stats.avg_move_time = ((p2_stats.avg_move_time * i) + p2_avg_move_time) / (i + 1);

            match result.status {
                GameStatus::Player1Win => {
                    let new_win_count = p1_stats.num_wins + 1;
                    p1_stats.avg_win_move_count = ((p1_stats.avg_win_move_count
                        * p1_stats.num_wins)
                        + result.num_moves as u64)
                        / new_win_count;
                    p1_stats.num_wins = new_win_count;
                }
                GameStatus::Player2Win => {
                    let new_win_count = p2_stats.num_wins + 1;
                    p2_stats.avg_win_move_count = ((p2_stats.avg_win_move_count
                        * p2_stats.num_wins)
                        + result.num_moves as u64)
                        / new_win_count;
                    p2_stats.num_wins = new_win_count;
                }
                _ => {}
            }
        }

        return Ok(BenchmarkResult {
            match_count: test.iterations,
            p1_stats: p1_stats,
            p2_stats: p2_stats,
        });
    }
}
