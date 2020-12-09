use crate::core;
use crate::core::*;

pub trait Heuristic<G: Game> {
    fn evaluate(&self, game: &G, state: &G::State, player: Player) -> f64;
}

pub struct MinMaxStrategy<G: core::Game> {
    pub heuristic: Box<dyn Heuristic<G>>,
    pub search_depth: u8,
}

struct TreeNode<G: core::Game> {
    pub state: G::State,
    pub action: G::Action,
    pub depth: u8,
}

struct NodeScore<G: core::Game> {
    pub action: G::Action,
    pub score: f64,
    pub depth: u8,
    pub player: core::Player,
}

fn coalesce_scores<G: core::Game>(
    scores_: &mut Vec<NodeScore<G>>,
    min_depth: u8,
    max_player: core::Player,
) {
    while !scores_.is_empty() && scores_[scores_.len() - 1].depth > min_depth {
        let mut best = scores_.pop().unwrap();

        while !scores_.is_empty() && scores_[scores_.len() - 1].depth == best.depth {
            let score = scores_.pop().unwrap();

            if score.player == max_player && score.score > best.score {
                best = score;
            } else if score.player != max_player && score.score < best.score {
                best = score;
            }
        }

        scores_.push(NodeScore {
            action: best.action,
            score: best.score,
            depth: best.depth - 1,
            player: core::other_player(best.player),
        });
    }
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

        let mut node_stack = Vec::<TreeNode<G>>::new();
        let mut score_stack = Vec::<NodeScore<G>>::new();

        // Seed the node stack
        for action in game.actions(state) {
            node_stack.push(TreeNode::<G> {
                state: game.play(&action, state),
                action: action,
                depth: 1,
            });
        }

        while node_stack.len() > 0 {
            let node = node_stack.pop().unwrap();

            coalesce_scores(&mut score_stack, node.depth, me);

            if game.status(&node.state) != GameStatus::InProgress || node.depth == self.search_depth
            {
                score_stack.push(NodeScore {
                    action: node.action,
                    score: self.heuristic.evaluate(game, &node.state, me),
                    depth: node.depth,
                    player: core::other_player(game.player(&node.state)),
                });

                continue;
            }

            for action in game.actions(&node.state) {
                node_stack.push(TreeNode::<G> {
                    state: game.play(&action, &node.state),
                    action: node.action.clone(),
                    depth: node.depth + 1,
                });
            }
        }

        coalesce_scores(&mut score_stack, 0, me);
        assert_eq!(score_stack.len(), 1);

        // Return the action with the best score
        return score_stack[0].action.clone();
    }
}
