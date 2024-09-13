use std::collections::VecDeque;
use std::fmt;

pub const GRID_WIDTH: usize = 20;
pub const GRID_HEIGHT: usize = 20;

#[derive(Debug)]
pub enum GameError {
    Collision,
    InvalidMove,
    OutOfBounds,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::Collision => write!(f, "Collision occurred!"),
            GameError::InvalidMove => write!(f, "Invalid move!"),
            GameError::OutOfBounds => write!(f, "Out of bounds!"),
        }
    }
}

impl std::error::Error for GameError {}

pub struct GameState {
    snake: VecDeque<(usize, usize)>,
    direction: (isize, isize),
    food: (usize, usize),
    collision_grid: Vec<Vec<bool>>,
}

impl GameState {
    pub fn new() -> Self {
        let initial_snake = VecDeque::from([(5, 5), (5, 4), (5, 3)]);
        let mut collision_grid = vec![vec![false; GRID_HEIGHT]; GRID_WIDTH];

        for &(x, y) in &initial_snake {
            collision_grid[x][y] = true;
        }

        Self {
            snake: initial_snake,
            direction: (1, 0), // Initial direction
            food: (10, 10), // Place food at an initial position
            collision_grid,
        }
    }

    pub fn update(&mut self) -> Result<(), GameError> {
        let head = self.snake.front().unwrap();
        let new_head = (
            ((head.0 as isize + self.direction.0) as usize) % GRID_WIDTH,
            ((head.1 as isize + self.direction.1) as usize) % GRID_HEIGHT,
        );

        // Collision detection using the grid
        if self.collision_grid[new_head.0][new_head.1] {
            return Err(GameError::Collision);
        }

        // Update the snake's body
        self.snake.push_front(new_head);
        self.collision_grid[new_head.0][new_head.1] = true;

        // If not growing, remove the tail
        if new_head != self.food {
            let tail = self.snake.pop_back().unwrap();
            self.collision_grid[tail.0][tail.1] = false;
        } else {
            self.spawn_food();
        }

        Ok(())
    }

    fn spawn_food(&mut self) {
        // Simple logic to spawn food, can be improved
        self.food = (rand::random::<usize>() % GRID_WIDTH, rand::random::<usize>() % GRID_HEIGHT);
    }
}
