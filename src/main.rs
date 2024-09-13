mod game;
mod renderer;
mod game;
mod render;
mod game_test;

use crate::game::{GameState, GameError};
use std::time::{Duration, Instant};
use std::thread;

fn main() {
    let mut game_state = GameState::new();

    // Main game loop
    loop {
        match game_state.update() {
            Ok(_) => {
                renderer::render(&game_state);
                thread::sleep(Duration::from_millis(100)); // Adjust frame rate
            }
            Err(GameError::Collision) => {
                println!("Game Over! Collision occurred!");
                break;
            }
            Err(e) => {
                eprintln!("An error occurred: {}", e);
                break;
            }
        }
    }
}
