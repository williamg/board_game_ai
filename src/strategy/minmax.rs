use crate::core;
use crate::core::*;

pub trait Heuristic<G: Game> {
    fn evaluate(&self, game: &G, state: &G::State, player: Player) -> f64;
}

pub struct MinMaxStrategy<G: core::Game> {
    pub heuristic: Box<dyn Heuristic<G>>,
    pub search_depth: u8,
    pub alpha_beta: bool,
}

#[derive(Clone)]
struct AlphaBeta {
    alpha: f64,
    beta: f64,
}

fn evaluate<G: core::Game>(
    game: &G,
    state: &G::State,
    depth: u8,
    heuristic: &Box<dyn Heuristic<G>>,
    alpha_beta_in: Option<AlphaBeta>,
    max_player: Player,
) -> f64 {
    let player = game.player(&state);
    let mut alpha_beta = alpha_beta_in.clone();

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
                alpha_beta.clone(),
                max_player,
            ));

            if let Some(ab) = &mut alpha_beta {
                ab.alpha = ab.alpha.max(value);

                if ab.alpha >= ab.beta {
                    break;
                }
            }
        }
    } else {
        value = f64::INFINITY;
        for action in actions {
            value = value.min(evaluate(
                game,
                &game.play(&action, state),
                depth - 1,
                &heuristic,
                alpha_beta.clone(),
                max_player,
            ));

            if let Some(ab) = &mut alpha_beta {
                ab.beta = ab.beta.min(value);

                if ab.beta <= ab.alpha {
                    break;
                }
            }
        }
    }

    return value;
}

impl<G> core::Strategy<G> for MinMaxStrategy<G>
where
    G: core::Game,
{
    fn name(&self) -> String {
        if self.alpha_beta {
            return "MinMaxAB".to_string();
        }

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
                    if self.alpha_beta {
                        Some(AlphaBeta {
                            alpha: -f64::INFINITY,
                            beta: f64::INFINITY,
                        })
                    } else {
                        None
                    },
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
