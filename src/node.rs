use crate::game::Game;
use crate::game::Move;
use crate::game::Player;

use rand::Rng;
use rand::prelude::IteratorRandom;

#[derive(Debug)]
pub struct ArenaTree {
    pub nodes: Vec<Node>,
}

#[derive(Debug,Copy,Clone)]
pub struct Node {
    pub parent_id: Option<usize>,
    pub first_child_id: Option<usize>,
    pub last_child_id: Option<usize>,
    pub visits: i32,
    pub wins: i32,
    pub move_from_parent: Option<Move>,
    pub game_state: Game,
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

impl ArenaTree {
    pub fn new_node(&mut self, game: Game, parent_id: Option<usize>, move_from_parent: Option<Move>) -> usize {
        let node_id = self.nodes.len();

        self.nodes.push(Node {
            parent_id,
            move_from_parent,
            game_state: game,
            wins: 0,
            visits: 0,
            first_child_id: None,
            last_child_id: None,
        });

        node_id
    }

    pub fn is_leaf_node(&self, node_id :usize) -> bool{
        let node = &self.nodes[node_id];

        let children_start = node.first_child_id.unwrap();
        let children_stop = node.last_child_id.unwrap();

        (children_start..children_stop).any(|i| self.nodes[i].visits == 0)
    }

    pub fn expand_step(&mut self, node_id: usize) -> usize {
        // check node is not terminal

        let game_state = self.nodes[node_id].game_state;
        if game_state.game_over {
            return node_id
        }

        // if node has no children, add them all to array:

        if self.nodes[node_id].first_child_id.is_none() || self.nodes[node_id].last_child_id.is_none() {
            self.nodes[node_id].first_child_id = Some(self.nodes.len());

            for legal_move in game_state.get_legal_moves() {
                let next_state: Game = game_state.make_move(&legal_move);

                self.new_node(next_state, Some(node_id), Some(legal_move));
            }

            self.nodes[node_id].last_child_id = Some(self.nodes.len());

        }

        let first = self.nodes[node_id].first_child_id.unwrap();
        let last = self.nodes[node_id].last_child_id.unwrap();

        (first..last)
            .filter(|i| self.nodes[*i].visits == 0)
            .choose(&mut rand::thread_rng())
            .unwrap()

    }


    pub fn select_step(&self, node_id: usize) -> usize {
        let node = &self.nodes[node_id];

        // if the node has no children, return it for expansion
        if node.first_child_id.is_none() || node.last_child_id.is_none() {
            return node_id;
        };

        let children_start = node.first_child_id.unwrap();
        let children_stop = node.last_child_id.unwrap();

        // if the node is a leaf, return it for expansion
        if self.is_leaf_node(node_id) {
            return node_id;
        } else {
            let mut scores : Vec<f32> = Vec::new();

            for i in children_start..children_stop {
                let lhs = self.nodes[i].wins as f32 / self.nodes[i].visits as f32;
                let rhs = (self.nodes[node_id].visits as f32).log(2.71) / (self.nodes[i].visits as f32);
                scores.push(lhs + 1.41*(rhs.powf(0.5)));
            }

            let maxi = scores.iter().enumerate()
                .fold(
                    (0, 0.0),
                    |max, (ind, val)| if val > &max.1 { (ind, *val) } else { max },
                )
                .0;

            self.select_step((children_start..children_stop).nth(maxi).unwrap())
        }
    }

    pub fn playout(&self, node_id: usize) -> Option<Player> {
        if self.nodes[node_id].game_state.game_over{
            return self.nodes[node_id].game_state.winner
         };

        self.nodes[node_id].game_state.greedy_playout()
    }

    pub fn backpropagate(&mut self, node_id : usize, player : Option<Player>){
        self.nodes[node_id].visits += 1;

        if let Some(winner) = player {
            if !variant_eq(&winner, &self.nodes[node_id].game_state.player) {
                self.nodes[node_id].wins += 1;
            };
        };

        if let Some(id) = self.nodes[node_id].parent_id {
            self.backpropagate(id, player);
        };

    }

    pub fn reccomend(&self) -> usize{
        let node = &self.nodes[0];

        let first_child = node.first_child_id.unwrap();
        let last_child = node.last_child_id.unwrap();

        let mut max_vists : i32 = 0;
        let mut reccomend : usize = 0;
        for child in first_child..last_child{
            let visits = self.nodes[child].visits;
            // let wins = self.nodes[child].wins;
            // println!("Player: {:?}, Action: {}, Visits: {},{},{:?}", node.game_state.player,child, visits, wins, self.nodes[child].game_state.winner);
            if visits >= max_vists {
                max_vists = visits;
                reccomend = child;
            };
        };
        reccomend
    }

    pub fn merge_trees(&self, other: &Self) -> Self {
        let mut result = ArenaTree{nodes : Vec::new()};

        result.nodes.push(self.nodes[0].merge(&other.nodes[0]));

        let first_child = self.nodes[0].first_child_id.unwrap();
        let last_child = self.nodes[0].last_child_id.unwrap();

        for child in first_child..last_child{
            result.nodes.push(self.nodes[child].merge(&other.nodes[child]));
        };

        return result
    }
}

impl Node {
    pub fn random_child(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.first_child_id.unwrap()..self.last_child_id.unwrap())
    }

    pub fn merge(&self, other: &Self) -> Self {
        let mut result = self.clone();

        result.visits += other.visits;
        result.wins += other.wins;

        return result
    }
}

