use crate::core;

use rand::Rng;

use indextree::{Arena, Node};

pub struct MCTSStrategy {}

struct NodeData<G: core::Game> {
    action: Option<G::Action>,
    state: G::State,
    playouts: f64,
    wins: f64,
}

fn playout<G: core::Game>(node: &mut Node<NodeData<G>>, game: &G) -> core::GameStatus {
    let data = node.get_mut();
    let mut state = data.state.clone();

    while game.status(&state) == core::GameStatus::InProgress {
        let actions = game.actions(&state);
        let action_idx = rand::thread_rng().gen_range(0, actions.len());
        state = game.play(&actions[action_idx], &state);
    }

    return game.status(&state);
}

impl<G> core::Strategy<G> for MCTSStrategy
where
    G: core::Game,
{
    fn name(&self) -> String {
        return "MCTS".to_string();
    }

    fn select_action(
        &self,
        game: &G,
        state: &<G as core::Game>::State,
    ) -> <G as core::Game>::Action {
        let mut tree = Arena::new();
        let root = tree.new_node(NodeData::<G> {
            action: None,
            state: (*state).clone(),
            playouts: 0.0,
            wins: 0.0,
        });

        for _ in 0..100 {
            let mut current = root;

            // Select a leaf for expansion
            loop {
                // Reached a leaf/terminal state
                let cur_state = tree.get(current).unwrap().get().state.clone();

                if game.status(&cur_state) != core::GameStatus::InProgress {
                    break;
                }

                // This node has un-expanded moves
                let actions = game.actions(&cur_state);

                if current.children(&tree).count() < actions.len() {
                    let expanded_actions = current
                        .children(&tree)
                        .map(|id| tree.get(id).unwrap().get().action.clone().unwrap())
                        .collect::<Vec<G::Action>>();
                    // Find an action that hasn't been expanded
                    let first_unused = actions
                        .iter()
                        .filter(|x| !expanded_actions.contains(x))
                        .nth(0)
                        .unwrap();

                    let new = tree.new_node(NodeData::<G> {
                        action: Some(first_unused.clone()),
                        state: game.play(&first_unused, &cur_state),
                        playouts: 0.0,
                        wins: 0.0,
                    });

                    current.append(new, &mut tree);
                    current = new;
                    break;
                }

                // Select a node
                current = current
                    .children(&tree)
                    .map(|c| {
                        let node = tree.get(c).unwrap();
                        let data = node.get();
                        let parent = tree.get(node.parent().unwrap()).unwrap();
                        let parent_data = parent.get();
                        let score = (data.wins / (data.playouts + 1.0))
                            + (2.0 * (parent_data.playouts + 1.0).ln() / (data.playouts + 1.0))
                                .sqrt();
                        return (c, score);
                    })
                    .max_by(|a, b| {
                        let (_, a_score) = a;
                        let (_, b_score) = b;
                        return a_score.partial_cmp(b_score).unwrap();
                    })
                    .map(|a| {
                        let (n, _) = a;
                        return n;
                    })
                    .unwrap();
            }

            let playout_id = current;
            let playout_node = tree.get(playout_id).unwrap().get();

            // Playout the node
            let this_status = game.status(&playout_node.state);
            let result = if this_status == core::GameStatus::InProgress {
                playout(tree.get_mut(playout_id).unwrap(), game)
            } else {
                this_status
            };

            // Propagate results up the tree
            let mut cur_node = Some(playout_id);

            while let Some(node_id) = cur_node {
                let node = tree.get_mut(node_id).unwrap();
                let data = node.get_mut();
                data.playouts += 1.0;

                match (&result, game.player(&data.state)) {
                    (core::GameStatus::Draw, _) => data.wins += 0.5,
                    (core::GameStatus::Player1Win, core::Player::Player2) => data.wins += 1.0,
                    (core::GameStatus::Player2Win, core::Player::Player1) => data.wins += 1.0,
                    _ => (),
                }

                cur_node = node.parent();
            }
        }

        // Choose the action with the best win rate for us
        let best_node = root
            .children(&tree)
            .max_by(|a, b| {
                let na = tree.get(*a).unwrap().get();
                let nb = tree.get(*b).unwrap().get();
                /*
                return (na.wins / na.playouts)
                    .partial_cmp(&(nb.wins / nb.playouts))
                    .unwrap();*/
                return na.playouts.partial_cmp(&nb.playouts).unwrap();
            })
            .unwrap();

        return tree
            .get(best_node)
            .unwrap()
            .get()
            .action
            .clone()
            .unwrap()
            .clone();
    }
}
