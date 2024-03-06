use removing_cubes::snake::Snake;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use rayon::prelude::*;

pub fn dedup<T>(v: &mut Vec<T>)
where T: std::hash::Hash + Eq
{
    let set: HashSet<_> = v.drain(..).collect();
    v.extend(set.into_iter())
}

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

    loop {
        if snakes.len() == 0 {
            break;
        }
        println!("Step {counter}, best = {shortest_snake}, num_snakes: {}", snakes.len());
        counter += 1;
        let mut new_snakes = vec![];
        //let mut snake_set = HashSet::new();
        let new_snakes_iter = snakes.into_par_iter()
            .map( |mut snake |{
                let mut new_snakes = vec![];
                while !snake.is_finished() {
                    let mut spawned_snakes = snake.take_step();
                    if spawned_snakes.len() == 0 {
                        println!("Error: no more snakes!");
                        println!("Spawning snake: {:?}", snake);
                        panic!();
                    }
                    // Reassign first snake as one we're updating
                    let mut end = spawned_snakes.split_off(1);
                    snake = spawned_snakes.pop().unwrap();
                    new_snakes.append(&mut end);
                    dedup(&mut new_snakes);
                    Snake::cull_snakes(&mut new_snakes, shortest_snake);
                }
                (snake, new_snakes)
            })
            .collect::<Vec<_>>();
        for (snake, mut new_snakes_inner) in new_snakes_iter {
            new_snakes.append(&mut new_snakes_inner);
            if snake.num_cubes() < shortest_snake {
                shortest_snake = snake.num_cubes();
                println!("Shortest snake: {}", shortest_snake);
                println!("Snake: {:?}", snake);
                println!();
            }
            else {
                //println!("Snake was greater than optimal, score: {}", snake.num_cubes());
            }
        }
        snakes = new_snakes;
        dedup(&mut snakes);
        Snake::cull_snakes(&mut snakes, shortest_snake);
    }
}
