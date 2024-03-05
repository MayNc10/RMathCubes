use removing_cubes::snake::Snake;

const SIZE: usize = 3;
fn main() {
    let first_snake = Snake::new(SIZE);
    let mut snakes = vec![first_snake];
    let mut counter = 0;
    loop {
        println!("Step {counter}");
        println!("Snake 0: {:?}", snakes[0]);
        println!("Snake 0 score: {:?}\n", snakes[0].score());
        counter += 1;
        let mut new_snakes = vec![];
        for snake in snakes {
            if snake.is_finished() {
                println!("Snake Finished!");
                println!("Snake State: {:?}", snake);
                break;
            }
            new_snakes.append(&mut snake.take_step());
        }
        snakes = new_snakes
    }
}
