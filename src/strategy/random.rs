use crate::core;

use rand::Rng;

pub struct RandomStrategy {
}

impl<G> core::Strategy<G> for RandomStrategy
where
    G: core::Game
{
    fn name(&self) -> String {
        return "Random".to_string();
    }

    fn select_action(
        &self,
        game: &G,
        state: &<G as core::Game>::State) -> <G as core::Game>::Action {
        let mut all_actions = game.actions(state);
        let action_idx = rand::thread_rng().gen_range(0, all_actions.len());
        return all_actions.remove(action_idx);
    }
}
