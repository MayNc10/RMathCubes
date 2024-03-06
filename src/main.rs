use removing_cubes::snake::Snake;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use rayon::prelude::*;

const SIZE: usize = 4;
fn main() {
    let first_snake = Snake::new(SIZE);
    let mut snakes = vec![first_snake];
    let mut counter = 0;
    let size_f32 = SIZE as f32;
    let mut shortest_snake = (1.5 * size_f32 * size_f32 + 0.5 * size_f32 - 1.0).floor() as usize;
    let mut file_name = format!("data/{0}x{0}x{0}", SIZE);
    let mut file = File::create(&file_name).unwrap();
    file.flush().unwrap();

    'solve: loop {
        if snakes.len() == 0 {
            break;
        }
        println!("Step {counter}, best = {shortest_snake}, num_snakes: {}", snakes.len());
        counter += 1;
        let mut new_snakes = vec![];
        //let mut snake_set = HashSet::new();
        let new_snakes_iter = snakes.into_iter()
            .map( |snake |{
                let mut new_snakes = vec![snake];
                while !new_snakes[0].is_finished() {
                    println!("Num new snakes: {}", new_snakes.len() - 1);
                    let mut spawned_snakes = new_snakes[0].take_step();
                    if spawned_snakes.len() == 0 {
                        println!("Error: no more snakes!");
                        println!("Spawning snake: {:?}", new_snakes[0]);
                        panic!();
                    }
                    // Reassign first snake as one we're updating
                    let mut end = spawned_snakes.split_off(1);
                    new_snakes[0] = spawned_snakes.pop().unwrap();
                    new_snakes.append(&mut end);
                }
                new_snakes
            })
            .collect::<Vec<_>>();
        for mut snake in new_snakes_iter {
            let mut new_snakes_inner = snake.split_off(1);
            let snake = snake.pop().unwrap();
            new_snakes.append(&mut new_snakes_inner);
            if snake.len() < shortest_snake {
                shortest_snake = snake.len();
                println!("Shortest snake: {}", shortest_snake);
                println!("Snake: {:?}", snake);
                println!();
            }
        }
        snakes = new_snakes;
    }
}
