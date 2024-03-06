use std::fmt::Debug;

use crate::game::*;
use crate::coordinate::Coordinate;
use itertools::Itertools;
use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Snake {
    path: Vec<Coordinate>,
    game: Game,
}

impl Snake {
    pub fn new(size: usize) -> Snake {
        Snake { path: vec![Coordinate::zero()], game: Game::new_with_initial(size) }
    }
    pub fn new_with_move(&self, mv: Coordinate) -> Snake {
        let mut new_path = self.path.clone();
        new_path.push(mv);
        let mut new_game = self.game.clone();
        new_game.visit(mv);
        Snake { path: new_path, game: new_game}
    }
    pub fn take_step(&self) -> Vec<Snake> {
        let mut possible_moves = vec![];
        let size = self.game.get_size();
        for i in -1..=1 {
            let new_coord = self.path.last().unwrap().add(i, 0, 0, size);
            if new_coord.is_some() {
                possible_moves.push(new_coord.unwrap());
            }
        }
        for j in -1..=1 {
            let new_coord = self.path.last().unwrap().add(0, j, 0, size);
            if new_coord.is_some() {
                possible_moves.push(new_coord.unwrap());
            }
        }
        for k in -1..=1 {
            let new_coord = self.path.last().unwrap().add(0, 0, k, size);
            if new_coord.is_some() {
                possible_moves.push(new_coord.unwrap());
            }
        }
        // Find the best move
        possible_moves.sort_by(|a,b| {
            let new_a = self.new_with_move(*a);
            let new_b = self.new_with_move(*b);
            new_b.score().cmp(&new_a.score())
        });
        // Filter to just have the best
        let best = self.new_with_move(possible_moves[0]);
        println!("\n** best score: {}", best.score());
        let best_score = best.score();

        let mut possible_moves = possible_moves.iter()
            .filter(|a| {
                let new_a = self.new_with_move(**a);
                println!("  score: {}, best: {}", new_a.score(), best_score);
                new_a.score() == best_score
            })
            .map(|a| {println!(".");  self.new_with_move(*a)})
            .collect::<Vec<_>>();

        println!("  ~~~Move length: {}", possible_moves.len());

        possible_moves.shuffle(&mut thread_rng());
        possible_moves
    }
    pub fn score(&self) -> usize {
        let (rows, cols, aisles) = self.game.positions_visited();
        rows.len() + cols.len() + aisles.len()
    }
    pub fn is_finished(&self) -> bool {
        self.game.is_solved()
    }
    pub fn len(&self) -> usize {
        self.path.len()
    }
    pub fn clean(&self) -> Snake {
        // Remove all duplicate coordinates except for start
        let mut previous = self.path[0..self.path.len() - 1]
            .into_iter()
            .unique()
            .map(|c| *c)
            .collect::<Vec<_>>();
        previous.push(*self.path.last().unwrap());
        Snake { path: previous, game: self.game.clone() }
    }
}

impl Debug for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, coord) in self.path.iter().enumerate() {
            writeln!(f, "#{}: {:?}", idx, coord)?;
        }
        write!(f, "{:?}", self.game)
    }
}