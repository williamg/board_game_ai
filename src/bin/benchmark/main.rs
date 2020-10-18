use board_game_ai::benchmark::{Test, BenchmarkGame};
use board_game_ai::games;

use serde_json::{Value};

use std::env;
use std::fs;

fn read_game(name: &str) -> Option<Box<dyn BenchmarkGame>> {
    return match name {
        "chess" => Some(Box::new(games::Chess {})),
        "tic-tac-toe" => Some(Box::new(games::TicTacToe {})),
        _ => None
    }
}

fn load_tests (v: Value) -> Result<Vec<Test>, &'static str> {
    let test_defs: Vec<Value> = match v {
        Value::Array(v) => v,
        _ => return Err("Malformed JSON, expect root to be array")
    };

    let mut tests: Vec<Test> = Vec::new();

    for td in test_defs {
        tests.push (Test {
            label: match &td["label"] {
                Value::String(s) => s.clone(),
                _ => return Err("Label must be string")
            },
            game: match &td["game"] {
                Value::String(s) => read_game(&s).expect("Unknown game"),
                _ => return Err("Game must be a string")
            },
            p1_strat_conf: td["p1Strat"].clone(),
            p2_strat_conf: td["p2Strat"].clone(),
            iterations: match &td["iterations"] {
                Value::Number(n) => if n.is_u64() {
                    n.as_u64().unwrap()
                } else {
                    return Err("Iterations must be a positive integer")
                }
                _ => return Err("Iterations must be a number")
            }
        });
    }

    return Ok(tests);
}

fn main() {
    let args: Vec<String> = env::args ().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Failed to read file");

    let v: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");
    let tests = load_tests(v).expect("Failed to load tests");

    for test in tests {
        let result = test.game.run(&test).expect("Failed to run test");
        println!("{}", result);
    }
}

