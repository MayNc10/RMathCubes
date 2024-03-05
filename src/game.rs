use std::collections::HashSet;
use crate::coordinate::Coordinate;

#[derive(Clone, Copy, Debug, Hash)]
pub struct Pair<T> 
where T: Eq + Sized 
{
    a: T,
    b: T,
}

impl<T> Pair<T> 
where T: Eq + Sized
{
    pub fn new(a: T, b: T) -> Pair<T> {
        Pair {a, b}
    }
}

impl<T> PartialEq for Pair<T> 
where T: Eq + Sized
{
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}
impl<T> Eq for Pair<T> 
where T: Eq + Sized {}

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
    pub fn positions_visited(&self) -> (HashSet<Pair<usize>>, HashSet<Pair<usize>>, HashSet<Pair<usize>>) {
        let size = self.get_size();
        // see if we've visited the correct rows, cols, and aisles

        let mut xys = HashSet::new();
        let mut yzs = HashSet::new();
        let mut xzs = HashSet::new();

        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    if self.grid[x][y][z] {
                        xys.insert(Pair::new(x, y));
                        yzs.insert(Pair::new(y, z));
                        xzs.insert(Pair::new(x, z));
                    }
                }
            }
        }
        (xys, yzs, xzs)
    }
    pub fn is_solved(&self) -> bool {
        let size = self.get_size();
        let complete = (0..size).collect::<Vec<usize>>();

        let (rows, cols, aisles) = self.positions_visited();

        for item1 in &complete {
            for item2 in &complete {
                let item = Pair::new(*item1, *item2);
                if !rows.contains(&item) || !cols.contains(&item) || !aisles.contains(&item) {
                    return false;
                }
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