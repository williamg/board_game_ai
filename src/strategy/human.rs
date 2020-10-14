use crate::core;
use crate::core::*;

pub struct HumanStrategy<P : core::ActionParser> {
    pub parser: P
}

impl<P> core::Strategy<P::Game> for HumanStrategy<P>
where
    P : core::ActionParser,
    P::Game : core::Game,
    <P::Game as core::Game>::Action : Eq
{
    fn name(&self) -> String {
        return "Human".to_string();
    }

    fn select_action(
        &self,
        game: &P::Game,
        state: &<P::Game as core::Game>::State) -> <P::Game as core::Game>::Action {
        loop {
            let action = self.parser.read_action();

            // See if the action is valid
            for available_action in game.actions(state).iter() {
                if &action == available_action {
                    return action;
                }
            }
        }
    }
}
