pub mod game;
use crate::game::Game;
use crate::game::Player;

pub mod node;
use crate::node::ArenaTree;

use rayon::prelude::*;
use reduce::Reduce;

fn mcts_search(root: Game) -> ArenaTree {
    let mut arena = ArenaTree{nodes : Vec::new()};

    arena.new_node(root, None, None);

    let mut selected: usize;
    let mut expanded: usize;
    let mut result: Option<Player>;

    for _ in 1..10_000{
        selected = arena.select_step(0);
        expanded = arena.expand_step(selected);
        result = arena.random_playout(expanded);
        arena.backpropagate(expanded, result);
    }

   arena
}

fn main() {
    let mut game = Game::new_belgian_daisy();
    // game.board[0][0] = Space::Empty;

    // game.board[2][0] = Space::Occupied(Player::White);
    // game.board[4][0] = Space::Occupied(Player::Black);
    // game.board[0][4] = Space::Empty;

    // let game_1 = mcts_search(game);
    // let game_2 = mcts_search(game);

    while !game.game_over {
    println!("{}", game);
    let arena_vec : Vec<ArenaTree> = (0..8).
        into_par_iter().
        map(|_| mcts_search(game)).
        collect();
    let merged_tree : ArenaTree = Reduce::reduce(arena_vec.into_iter(), | a,b | (&a).merge_trees(&b)).unwrap();

    game = merged_tree.nodes[merged_tree.reccomend()].game_state;
    }
    println!("{}", game);
}
