mod game;
use game::*;

mod tic_tac_toe;

fn main() {
    loop {
        play_game();
    }
}

fn play_game() {
    println!("=============================");
    let g = tic_tac_toe::TicTacToe {};
    let mut state = g.init();

    while g.status (&state) == game::GameStatus::InProgress {
        println!("{}", g.serialize_state(&state));

        match g.player(&state) {
            game::Player::Player1 => println!("Enter move for player 1"),
            game::Player::Player2 => println!("Enter move for player 2")
        };

        let mut valid_action = false;

        while ! valid_action {
            let action = g.read_action();

            // Validate action
            for available_action in g.actions(&state).iter() {
                if &action == available_action {
                    valid_action = true;
                    break;
                }
            }

            if ! valid_action {
                println!("Invalid action!");
            } else {
                state = g.play(&action, &state);
            }
        }
    }

    match g.status (&state) {
        game::GameStatus::Player1Win => println!("Player 1 wins!"),
        game::GameStatus::Player2Win => println!("Player 2 wins!"),
        game::GameStatus::Draw => println!("Draw"),
        game::GameStatus::InProgress => ()
    }

    println!("{}", g.serialize_state(&state));
}
