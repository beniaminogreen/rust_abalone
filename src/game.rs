use std::fmt;
use rand::seq::SliceRandom;

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

static DIRS: [(i8, i8); 6] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (0, -1),
    (-1, -1),
    (-1, 0),
];

fn in_bounds(x: i8, y: i8) -> bool {
    x >= 0 && x <= 8 && y >= 0 && y <= 8
}

#[derive(Clone, Debug, Copy)]
pub enum Space {
    Occupied(Player),
    Empty,
    OutOfBounds,
}

#[derive(Clone, Debug, Copy)]
pub enum Player {
    Black,
    White,
}

#[derive(Clone, Debug, Copy)]
pub struct Game {
    pub board: [[Space; 9]; 9],
    pub player: Player,
    pub winner: Option<Player>,
    pub game_over: bool,
    pub move_number: i16,
    pub white_pieces: i16,
    pub black_pieces: i16
}

impl Game {
    pub fn new_game() -> Self {
        let mut game = Self {
            board: [[Space::Empty; 9]; 9],
            player: Player::White,
            winner: None,
            move_number: 0,
            game_over: false,
            white_pieces  : 0,
            black_pieces : 0,
        };

        for x in 0..9 {
            for y in 0..9 {
                if y < 4 && x > 4 + y {
                    game.board[x][y] = Space::OutOfBounds;
                } else if y > 4 && x < y - 4 {
                    game.board[x][y] = Space::OutOfBounds;
                }
            }
        }

        game
    }

    pub fn new_basic() -> Self {
        let mut game = Self::new_game();

        let pieces = vec![
            (0, 3),
            (0, 4),
            (1, 4),
            (1, 5),
            (2, 4),
            (2, 5),
            (2, 6),
            (3, 5),
            (3, 6),
            (3, 7),
            (4, 6),
            (4, 7),
            (4, 8),
            (5, 8),
        ];

        game.white_pieces = 14;
        game.black_pieces = 14;

        for (x, y) in pieces {
            game.board[y][x] = Space::Occupied(Player::White);
            game.board[x][y] = Space::Occupied(Player::Black);
        }
        game
    }

    pub fn new_german_daisy() -> Self {
        let mut game = Self::new_game();

        let white_pieces = vec![
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 4),
            (6, 5),
            (6, 4),
            (7, 6),
            (7, 5),
            (7, 4),
            (8, 6),
            (8, 5),
        ];

        let black_pieces = vec![
            (2, 1),
            (2, 0),
            (3, 2),
            (3, 1),
            (3, 0),
            (4, 2),
            (4, 1),
            (4, 6),
            (4, 7),
            (5, 6),
            (5, 7),
            (5, 8),
            (6, 7),
            (6, 8),
        ];

        game.white_pieces = 14;
        game.black_pieces = 14;

        for (x, y) in white_pieces {
            game.board[y][x] = Space::Occupied(Player::White);
        }

        for (x, y) in black_pieces {
            game.board[y][x] = Space::Occupied(Player::Black);
        }

        game
    }

    pub fn new_belgian_daisy() -> Self {
        let mut game = Self::new_game();

        let white_pieces = vec![
            (0, 3),
            (0, 4),
            (1, 3),
            (1, 4),
            (1, 5),
            (2, 4),
            (2, 5),
            (6, 4),
            (6, 3),
            (7, 5),
            (7, 4),
            (7, 3),
            (8, 5),
            (8, 4),
        ];

        let black_pieces = vec![
            (4, 1),
            (3, 0),
            (3, 1),
            (4, 2),
            (4, 1),
            (4, 0),
            (5, 2),
            (5, 1),
            (3, 6),
            (3, 7),
            (4, 6),
            (4, 7),
            (4, 8),
            (5, 7),
            (5, 8),
        ];

        game.white_pieces = 14;
        game.black_pieces = 14;

        for (x, y) in white_pieces {
            game.board[y][x] = Space::Occupied(Player::White);
        }

        for (x, y) in black_pieces {
            game.board[y][x] = Space::Occupied(Player::Black);
        }

        game
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for x in 0..9 {
            for y in 0..9 {
                // if a space occupied by a player
                if let Space::Occupied(player) = self.board[x][y] {
                    if variant_eq(&player, &self.player) {
                        // Search along all directions
                        for dir_index in 0..6 {
                            moves.append(&mut Self::search_along_dir(&self, x, y, dir_index));
                        }
                    };
                };
            }
        }
        moves
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn search_along_dir(&self, x: usize, y: usize, dir_index: usize) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let mut sumito = 1;
        let mut opp_sumito = 0;

        let x_off = DIRS[dir_index].0;
        let y_off = DIRS[dir_index].1;

        let mut new_x = x as i8 + x_off;
        let mut new_y = y as i8 + y_off;

        if !in_bounds(new_x, new_y) {
            return moves;
        };

        match self.board[new_x as usize][new_y as usize] {
            Space::Empty => {
                moves.push(Move::Inline {
                    start: (x, y),
                    dir: (x_off, y_off),
                });
                return moves;
            }
            Space::OutOfBounds => return moves,
            _ => (),
        };

        loop {
            // if next square not in-bounds, check if you are pushing an opponent's piece off
            if !in_bounds(new_x, new_y) {
                if sumito > opp_sumito && opp_sumito != 0 {
                    moves.push(Move::Inline {
                        start: (x, y),
                        dir: (x_off, y_off),
                    });
                }
                return moves;
            };
            match self.board[new_x as usize][new_y as usize] {
                // if next square is occupied, check if it is by you, or not.
                // if you occupy it, check for broadsides and look ahead to next square
                // if not, check if you can push opponent
                Space::Occupied(player) => {
                    if variant_eq(&player, &self.player) {
                        if opp_sumito != 0 {
                            break;
                        }
                        sumito += 1;
                        if sumito > 3 {
                            break;
                        }

                        'perp_dir: for new_dir_offset in 1..3 {
                            let perp_dir_idx: usize = (dir_index + new_dir_offset) % 6;

                            let x_perp_off: i8 = DIRS[perp_dir_idx].0;
                            let y_perp_off: i8 = DIRS[perp_dir_idx].1;

                            if !in_bounds(new_x + x_perp_off, new_y + y_perp_off) {
                                continue 'perp_dir;
                            };
                            if !variant_eq(&self.board[(new_x + x_perp_off) as usize][(new_y + y_perp_off) as usize], &Space::Empty) {
                                continue 'perp_dir;
                            };

                            if !in_bounds(x as i8 + x_perp_off, y as i8 + y_perp_off) {
                                continue 'perp_dir;
                            };
                            if !variant_eq(&self.board[(x as i8 + x_perp_off) as usize][(y as i8 + y_perp_off) as usize], &Space::Empty) {
                                continue 'perp_dir;
                            };

                            if sumito == 3 {
                                let mid_x: i8 = x as i8 + x_off + x_perp_off;
                                let mid_y: i8 = y as i8 + y_off + y_perp_off;

                                if !in_bounds(mid_x, mid_y) {
                                    continue 'perp_dir;
                                };
                                if !variant_eq(&self.board[mid_x as usize][mid_y as usize], &Space::Empty) {
                                    continue 'perp_dir;
                                };
                                moves.push(Move::Broadside {
                                    start: (x, y),
                                    mid: Some((
                                        (x as i8 + x_off) as usize,
                                        (y as i8 + y_off) as usize,
                                    )),
                                    stop: (new_x as usize, new_y as usize),
                                    dir: (x_perp_off, y_perp_off),
                                })
                            } else {
                                moves.push(Move::Broadside {
                                    start: (x, y),
                                    mid: None,
                                    stop: (new_x as usize, new_y as usize),
                                    dir: (x_perp_off, y_perp_off),
                                })
                            }
                        }
                    } else {
                        opp_sumito += 1;
                        if opp_sumito == sumito {
                            break;
                        }
                    }
                }
                Space::Empty => {
                    if sumito > opp_sumito {
                        moves.push(Move::Inline {
                            start: (x, y),
                            dir: (x_off, y_off),
                        });
                        break;
                    }
                }
                Space::OutOfBounds => {
                    if sumito > opp_sumito && opp_sumito != 0 {
                        moves.push(Move::Inline {
                            start: (x, y),
                            dir: (x_off, y_off),
                        });
                    }
                    break;
                }
            };
            new_x += x_off;
            new_y += y_off;
        }
        return moves;
    }

    pub fn make_move(&self, next_move: &Move) -> Self {
        let mut new_state = self.clone();

        new_state.move_number += 1;

        new_state.player = match self.player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        match next_move {
            Move::Inline { start, dir } => {
                let mut x = start.0 as i8;
                let mut y = start.1 as i8;
                let mut new_x = x + dir.0;
                let mut new_y = y + dir.1;

                new_state.board[x as usize][y as usize] = Space::Empty;
                new_state.board[new_x as usize][new_y as usize] = self.board[start.0][start.1];

                x = new_x;
                y = new_y;
                new_x = x + dir.0;
                new_y = y + dir.1;

                while matches!(self.board[x as usize][y as usize], Space::Occupied(_)) {

                    //if I would be moving to an out of bounds, don't.
                    // Instead, knock piece off
                    if !in_bounds(new_x, new_y)  || matches!(self.board[new_x as usize][new_y as usize], Space::OutOfBounds) {
                        match self.board[x as usize][y as usize]{
                            Space::Occupied(moved_off) => {
                                match moved_off {
                                    Player::White => {new_state.white_pieces -= 1},
                                    Player::Black => {new_state.black_pieces -= 1},
                                };
                            },
                            _=> panic!("{:?}", self.board[x as usize][y as usize]),
                        };
                        break
                    };


                    new_state.board[new_x as usize][new_y as usize] = self.board[x as usize][y as usize];
                    x = new_x;
                    y = new_y;
                    new_x = x + dir.0;
                    new_y = y + dir.1;
                }
            }
            Move::Broadside {
                start,
                mid,
                stop,
                dir,
            } => {
                new_state.board[start.0][start.1] = Space::Empty;
                new_state.board[stop.0][stop.1] = Space::Empty;

                new_state.board[(start.0 as i8 + dir.0) as usize]
                    [(start.1 as i8 + dir.1) as usize] = self.board[start.0][start.1];
                new_state.board[(stop.0 as i8 + dir.0) as usize][(stop.1 as i8 + dir.1) as usize] =
                    self.board[stop.0][stop.1];

                if let Some(mid) = mid {
                    new_state.board[mid.0][mid.1] = Space::Empty;
                    new_state.board[(mid.0 as i8 + dir.0) as usize]
                        [(mid.1 as i8 + dir.1) as usize] = self.board[mid.0][mid.1];
                };
            }
        };

        if new_state.white_pieces == 8 {
            new_state.winner = Some(Player::Black);
            new_state.game_over = true;
        };


        if new_state.black_pieces == 8 {
            new_state.winner = Some(Player::White);
            new_state.game_over = true;
        };

        if new_state.move_number > 1000 {
            new_state.winner = None;
            new_state.game_over = true;
        }


        new_state
    }

    pub fn validate_state(&self) {
        let mut white_seen: i16 = 0;
        let mut black_seen: i16 = 0;

        for y in 0..9 {
            for x in 0..9 {
                match self.board[x][y] {
                    Space::Occupied(Player::Black) => black_seen += 1,
                    Space::Occupied(Player::White) => white_seen +=1,
                    _ => (),
                };
            }
        }

        if black_seen != self.black_pieces {
            panic!("counting error")
        }

        if white_seen != self.white_pieces {
            panic!("counting error")
        }

    }

    pub fn random_playout(&self) -> Option<Player> {
        let next_move : Move = * Game::get_legal_moves(self)
            .choose(&mut rand::thread_rng())
            .unwrap();

        let next_state : Game = Game::make_move(self, &next_move);

        if next_state.game_over {
            return next_state.winner
        } else {
            return next_state.random_playout()
        }
    }

    pub fn greedy_playout(&self) -> Option<Player> {
        let next_moves : Vec<Move> = Game::get_legal_moves(self);

        let current_black = self.black_pieces;
        let current_white = self.white_pieces;

        let next_state: Game;
        for potenital_move in &next_moves{
            let potential_state : Game = Game::make_move(self, potenital_move);

            let next_black = potential_state.black_pieces;
            let next_white = potential_state.white_pieces;

            if next_black < current_black || next_white < current_white {
                next_state = potential_state;


                if next_state.game_over {
                    return next_state.winner
                } else {
                    return next_state.greedy_playout()
                };
            };
        };


        let next_move: Move = * next_moves
            .choose(&mut rand::thread_rng())
            .unwrap();

        next_state = Game::make_move(self, &next_move);

        if next_state.game_over {
            return next_state.winner
        } else {
            return next_state.greedy_playout()
        };
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for y in 0..9 {
            for x in 0..9 {
                match self.board[x][y] {
                    Space::Empty => output.push('_'),
                    Space::Occupied(Player::Black) => output.push('B'),
                    Space::Occupied(Player::White) => output.push('W'),
                    Space::OutOfBounds => output.push(' '),
                };
            }
            output.push('\n');
        }

        output.push_str(&format!("Turn Number {}\n", self.move_number));
        match self.player {
            Player::White => output.push_str("White To Move\n"),
            Player::Black => output.push_str("Black To Move\n"),
        }

        if self.white_pieces == 8 {
            output.push_str("Black Wins!\n");
        } else if self.black_pieces == 8 {
            output.push_str("White Wins!\n");
        } else {
            output.push_str(&format!("White Score: {}\n", 14-self.black_pieces));
            output.push_str(&format!("Black Score: {}\n", 14-self.white_pieces));
        }

        write!(f, "{}", output)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Broadside {
        start: (usize, usize),
        mid: Option<(usize, usize)>,
        stop: (usize, usize),
        dir: (i8, i8),
    },
    Inline {
        start: (usize, usize),
        dir: (i8, i8),
    },
}
