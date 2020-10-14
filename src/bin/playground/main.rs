use std::io;

use board_game_ai::playground;

use board_game_ai::games;

fn games() -> Vec<Box<dyn playground::PlaygroundGame>> {
    return vec!({ Box::new(games::TicTacToe {}) });
}

fn select_game() -> Box<dyn playground::PlaygroundGame> {
    let mut all_games = games();
    let mut selected_idx = -1;

    while selected_idx < 0 || selected_idx >= all_games.len() as i32 {
        println!("Select game:");

        let mut idx = 1;

        for game in all_games.iter() {
            println!("{}) {}", idx, game.name());
            idx += 1;
        }

        let mut game_str = String::new();
        io::stdin()
                .read_line(&mut game_str)
                .expect("Failed to read line");

        selected_idx = game_str.trim().parse::<i32>().expect("Expected number") - 1;
    }

    return all_games.remove(selected_idx as usize);
}

fn main() {
    let g = select_game();
    g.start ();
    /*

    loop {
        let gameConfig = read_config();
        play_game(gameConfig);
    }*/
}

