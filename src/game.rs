use std::collections::HashSet;
use crate::coordinate::Coordinate;

#[derive(Clone, Debug)]
pub struct Game {
    grid: Vec<Vec<Vec<bool>>>,
}

impl Game {
    pub fn new(size: usize) -> Game {
        Game { grid: vec![vec![vec![false; size]; size]; size] }
    }
    pub fn new_with_initial(size: usize) -> Game {
        let mut grid = vec![vec![vec![false; size]; size]; size];
        grid[0][0][0] = true;
        Game { grid }
    }
    pub fn visit(&mut self, coord: Coordinate) {
        self.grid[coord.x][coord.y][coord.z] = true;
    }
    pub fn positions_visited(&self) -> (HashSet<usize>, HashSet<usize>, HashSet<usize>) {
        let size = self.get_size();
        // see if we've visited the correct rows, cols, and aisles

        let mut rows = HashSet::new();
        let mut cols = HashSet::new();
        let mut aisles = HashSet::new();

        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    if self.grid[x][y][z] {
                        rows.insert(x);
                        cols.insert(y);
                        aisles.insert(z);
                    }
                }
            }
        }
        (rows, cols, aisles)
    }
    pub fn is_solved(&self) -> bool {
        let size = self.get_size();
        let complete = (0..size).collect::<Vec<usize>>();

        let (rows, cols, aisles) = self.positions_visited();

        for item in &complete {
            if !rows.contains(item) || !cols.contains(item) || !aisles.contains(item) {
                return false;
            }
        }
        true

    }
    pub fn verify_orthogonal(&self) -> bool {
        true
    }
    pub fn get_size(&self) -> usize {
        self.grid.len()
    }
}