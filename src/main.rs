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
        let mut snake_set = HashSet::new();
        let new_snakes_iter = snakes.into_par_iter()
            .map( |snake |{
                snake.take_step()
            })
            .collect::<Vec<_>>();

        for new_snake in new_snakes_iter {
            //println!("New Snake: {:?}", new_snake);
            for snake in new_snake {
                if snake.is_finished() {
                    if snake.len() < shortest_snake {
                        shortest_snake = snake.len();
                        println!("Shortest len: {}", shortest_snake);
                        let mut file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(&file_name)
                            .unwrap();
                        if let Err(e) = writeln!(file, "snake length: {}\n, snake: {:?}", snake.len(), snake) {
                            eprintln!("Couldn't write to file: {}", e);
                            panic!();
                        }
                        file.flush().unwrap();
                    }
                }

                let snake = snake.clean();
                let unique = snake_set.insert(snake.clone());
                if unique && snake.len()  < shortest_snake && !snake.is_finished() {
                    new_snakes.push(snake);
                }
            }
        }
        snakes = new_snakes;
    }
}
