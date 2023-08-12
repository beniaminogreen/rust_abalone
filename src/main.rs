use rand::seq::SliceRandom;
use rayon::prelude::*;
use reduce::Reduce;

pub mod game;
use crate::game::Game;
use crate::game::Player;
use crate::game::Move;

pub mod node;
use crate::node::ArenaTree;

fn prompt_user_for_move(game: &Game) -> Game{
    let legal_moves = game.get_legal_moves();

    for (i, legal_move) in legal_moves.iter().enumerate() {
        println!("Move {i}: \n {}", game.make_move(&legal_move));
    }

    let mut line = String::new();
    println!("Select Move from above");
    std::io::stdin().read_line(&mut line).unwrap();

    let i: usize = line.trim().parse().expect("invalid input");
    println!("selected {:?}", i);

    game.make_move(&legal_moves[i])
}

fn mcts_search(root: Game, n_think : i32) -> ArenaTree {
    let mut arena = ArenaTree{nodes : Vec::new()};

    arena.new_node(root, None, None);

    let mut selected: usize;
    let mut expanded: usize;
    let mut result: Option<Player>;

    for _ in 0..n_think{
        selected = arena.select_step(0);
        expanded = arena.expand_step(selected);
        result = arena.playout(expanded);
        arena.backpropagate(expanded, result);
    }

   arena
}

fn main() {
    println!("n_think,winner,turn_n ");

    for _ in 0..30 {
        let think_nums = vec![100,200,300,400,500,1000,1500,2500,5000,10000];

        for n_think in think_nums {
            let mut game = Game::new_basic();
            while !game.game_over {
                // println!("starting turn");
                let arena_vec : Vec<ArenaTree> = (0..7).
                    into_par_iter().
                    map(|_| mcts_search(game, n_think)).
                    collect();
                let merged_tree : ArenaTree = Reduce::reduce(arena_vec.into_iter(), | a,b | (&a).merge_trees(&b)).unwrap();

                game = merged_tree.nodes[merged_tree.reccomend()].game_state;
                game.validate_state();


                let legal_moves = game.get_legal_moves();
                game = game.make_move(legal_moves.choose(&mut rand::thread_rng()).unwrap());
            }
            println!("{}, {:?}, {}",n_think, game.winner.unwrap(), game.move_number);
        }
    }
}
