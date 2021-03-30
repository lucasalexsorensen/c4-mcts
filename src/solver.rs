use std::{collections::{HashMap, HashSet}, time::{Duration, SystemTime}};
use rand::{prelude::SliceRandom};
use crate::board::Board;

const EXPLORATION_PARAMETER: f32 = std::f32::consts::SQRT_2;

#[derive(Debug, Clone, Copy)]
struct Stats {
    wins: f32,
    count: u32
}

impl Default for Stats {
    fn default() -> Self {
        return Stats { wins: 0f32, count: 0u32 };
    }
}

pub struct Solver {
    visited_set: HashSet<Board>,
    stats: HashMap<Board, Stats>
}

impl Solver {
    pub fn default () -> Self {
        return Solver { visited_set: HashSet::new(), stats: HashMap::new() };
    }

    pub fn search(&mut self, root_node: Board) -> u8 {
        // reset visited set /counter
        self.visited_set = HashSet::new();
        self.stats = HashMap::new();
        self.visited_set.insert(root_node);
        self.stats.insert(root_node, Stats::default());

        let max_time = SystemTime::now() + Duration::from_millis(2500);
        let mut counter = 0;
        loop {
            counter += 1;

            // SELECTION
            let frontier_path = self.select(root_node);
            let frontier_node = *frontier_path.last().unwrap();

            // EXPANSION
            self.expand(frontier_node);

            // SIMULATION
            let result = self.rollout(frontier_node);

            // BACKPROPAGATION
            self.backpropagate(result, frontier_path);

            if counter % 250 == 0 {
                if SystemTime::now() >= max_time { break }
            }
        }

        let mut best_move = 0;
        let mut best_score = 0f32;
        root_node.get_legal_moves().iter().for_each(|mv| {
            let stats = self.stats.get(&root_node.into_move(*mv)).unwrap();
            let score = stats.wins as f32 / stats.count as f32;
            if score > best_score {
                best_score = score;
                best_move = *mv;
            }
        });

        return best_move;
    }

    fn select(&self, start_node: Board) -> Vec<Board> {
        let mut path = Vec::new();
        let mut node = start_node;
        loop {
            path.push(node);
            let legal_moves = node.get_legal_moves();

            if legal_moves.len() < 1 || node.is_player_win() { return path; }

            // determine unvisited children
            let unvisited_nodes = legal_moves.iter().filter(|mv| {
                return !self.visited_set.contains(&node.into_move(**mv));
            }).collect::<Vec<_>>();

            if unvisited_nodes.len() > 0 { // children exist! skip UCB calculation as this becomes trivial
                let next_move = unvisited_nodes.choose(&mut rand::thread_rng()).unwrap();
                path.push(node.into_move(**next_move));
                return path;
            } else { // no children exist!
                let total = self.stats.get(&node).unwrap().count;

                let mut best_move = *legal_moves.last().unwrap();
                let mut best_ucb = 0f32;

                for mv in legal_moves {
                    let stats = self.stats.get(&node.into_move(mv)).unwrap();
                    let ucb = Self::calc_ucb(stats.wins, stats.count as f32, total as f32);
                    if ucb > best_ucb {
                        best_ucb = ucb;
                        best_move = mv;
                    }
                }
                node = node.into_move(best_move);
            }
        }
    }

    fn expand(&mut self, node: Board) {
        if !self.visited_set.contains(&node) {
            self.visited_set.insert(node);
            self.stats.insert(node, Stats::default());
        }
    }

    fn rollout(&self, start_node: Board) -> f32 {
        let mut node = start_node;
        while !node.is_draw() {
            let legal_moves = node.get_legal_moves();
            if legal_moves.len() < 1 { return 0.5; }

            node = node.into_move(*legal_moves.choose(&mut rand::thread_rng()).unwrap());
            if Board::are_four_connected(node.get_opponent_position()) {
                return if node.moves_count % 2 == 1 { 1f32 } else { 0f32 };
            }
        }
        return 0.5;
    }

    fn backpropagate(&mut self, result: f32, path: Vec<Board>) {
        for node in path {
            match self.stats.get_mut(&node) {
                Some(ref mut stat) => {
                    stat.count += 1;
                    stat.wins += result;
                },
                None => ()
            }
        }
    }

    fn calc_ucb(wins: f32, node_played: f32, total_played: f32) -> f32 {
        return (wins / node_played) + EXPLORATION_PARAMETER * (total_played.ln() / node_played).sqrt();
    }
}
