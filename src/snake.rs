use crate::game::*;
use crate::coordinate::Coordinate;

#[derive(Clone, Debug)]
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
            for j in -1..=1 {
                for k in -1..=1 {
                    let new_coord = self.path.last().unwrap().add(i, j, k, size);
                    if new_coord.is_some() {
                        possible_moves.push(new_coord.unwrap());
                    }
                }
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
        let best_score = best.score();
        let possible_moves = possible_moves.iter()
            .filter(|a| {
                let new_a = self.new_with_move(**a);
                new_a.score() == best_score
            })
            .map(|a| self.new_with_move(*a))
            .collect::<Vec<_>>();

        possible_moves
    }
    pub fn score(&self) -> usize {
        let (rows, cols, aisles) = self.game.positions_visited();
        rows.len() + cols.len() + aisles.len()
    }
    pub fn is_finished(&self) -> bool {
        self.game.is_solved()
    }
}