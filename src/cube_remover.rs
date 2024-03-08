use crate::{coordinate::Coordinate, game::Game};

pub fn efficiency(game: &Game) -> usize {
    let mut efficiency = 0;
    for x in 0..game.side_len() {
        for y in 0..game.side_len() {
            for z in 0..game.side_len() {
                let coord = Coordinate {x, y, z};
                efficiency += efficiency_cube(game, coord);
            }
        }
    }
    efficiency
}

pub fn efficiency_cube(game: &Game, coord: Coordinate) -> usize {
    let mut new_game = game.clone();
    new_game.remove_cube(coord);
    // Somehow this glitches??
    count_filled(game).checked_sub(count_filled(&new_game)).unwrap_or(0)
}

pub fn count_filled(game: &Game) -> usize {
    let filled = game.positions_visited();
    filled.0.len() + filled.1.len() + filled.2.len()
}

pub fn remove_cube(game: &mut Game) -> bool {
    let mut best = (Coordinate {x: 0, y: 0, z: 0}, 0);
    for x in 0..game.side_len() {
        for y in 0..game.side_len() {
            for z in 0..game.side_len() {
                if game.grid()[x][y][z] {
                    let coord = Coordinate {x, y, z};
                    let mut new_game = game.clone();
                    new_game.remove_cube(coord);
                    let efficiency = efficiency(&new_game);
                    if efficiency > best.1 && new_game.is_solved() && new_game.verify_orthogonal() {
                        best.1 = efficiency;
                        best.0 = coord;
                    }
                }
            }
        }
    }
    if best.1 > 0 {
        game.remove_cube(best.0);
    }
    best.1 > 0
}