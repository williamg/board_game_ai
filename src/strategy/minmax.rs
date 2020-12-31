use crate::core;
use crate::core::*;

pub trait Heuristic<G: Game> {
    fn evaluate(&self, game: &G, state: &G::State, player: Player) -> f64;
}

pub struct MinMaxStrategy<G: core::Game> {
    pub heuristic: Box<dyn Heuristic<G>>,
    pub search_depth: u8,
}

fn evaluate<G: core::Game>(
    game: &G,
    state: &G::State,
    depth: u8,
    heuristic: &Box<dyn Heuristic<G>>,
    max_player: Player,
) -> f64 {
    let player = game.player(&state);

    if depth == 0 || game.status(&state) != GameStatus::InProgress {
        return heuristic.evaluate(game, state, max_player);
    }

    let actions = game.actions(&state);
    let mut value;

    if player == max_player {
        value = -f64::INFINITY;
        for action in actions {
            value = value.max(evaluate(
                game,
                &game.play(&action, state),
                depth - 1,
                &heuristic,
                max_player,
            ));
        }
    } else {
        value = f64::INFINITY;
        for action in actions {
            value = value.min(evaluate(
                game,
                &game.play(&action, state),
                depth - 1,
                &heuristic,
                max_player,
            ));
        }
    }

    return value;
}

impl<G> core::Strategy<G> for MinMaxStrategy<G>
where
    G: core::Game,
{
    fn name(&self) -> String {
        return "MinMax".to_string();
    }

    fn select_action(&self, game: &G, state: &G::State) -> G::Action {
        let me = game.player(state);

        // Map each action to its score
        let actions = game.actions(state);
        let scored = actions.iter().map(|a| {
            (
                a,
                evaluate(
                    game,
                    &game.play(&a, state),
                    self.search_depth,
                    &self.heuristic,
                    me,
                ),
            )
        });

        let (best_action, _) = scored
            .max_by(|a, b| {
                let (_, score_a) = a;
                let (_, score_b) = b;
                return score_a.partial_cmp(score_b).unwrap();
            })
            .unwrap();

        return best_action.clone();
    }
}
