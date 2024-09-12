extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{EventLoop, PressEvent};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use std::collections::VecDeque;

const WINDOW_SIZE: [u32; 2] = [640, 480];
const SQUARE_SIZE: f64 = 20.0;
const GRID_WIDTH: i32 = (WINDOW_SIZE[0] as f64 / SQUARE_SIZE) as i32;
const GRID_HEIGHT: i32 = (WINDOW_SIZE[1] as f64 / SQUARE_SIZE) as i32;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: VecDeque<(i32, i32)>,
    direction: Direction,
}

impl Snake {
    fn new(x: i32, y: i32) -> Snake {
        let mut body = VecDeque::new();
        body.push_back((x, y));
        Snake {
            body,
            direction: Direction::Right,
        }
    }

    fn update(&mut self, grow: bool) -> Result<(), ()> {
        let mut new_head = *self.body.front().unwrap();
        match self.direction {
            Direction::Up => new_head.1 = (new_head.1 - 1 + GRID_HEIGHT) % GRID_HEIGHT,
            Direction::Down => new_head.1 = (new_head.1 + 1) % GRID_HEIGHT,
            Direction::Left => new_head.0 = (new_head.0 - 1 + GRID_WIDTH) % GRID_WIDTH,
            Direction::Right => new_head.0 = (new_head.0 + 1) % GRID_WIDTH,
        }

        if self.collides_with_self(&new_head) {
            return Err(()); // Indicate collision
        }

        self.body.push_front(new_head);

        if !grow {
            self.body.pop_back();
        }

        Ok(()) // Indicate successful update
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if (self.direction == Direction::Up && new_direction != Direction::Down)
            || (self.direction == Direction::Down && new_direction != Direction::Up)
            || (self.direction == Direction::Left && new_direction != Direction::Right)
            || (self.direction == Direction::Right && new_direction != Direction::Left)
        {
            self.direction = new_direction;
        }
    }

    fn head_position(&self) -> (i32, i32) {
        *self.body.front().unwrap()
    }

    fn collides_with_self(&self, head: &(i32, i32)) -> bool {
        self.body.iter().skip(1).any(|&pos| pos == *head)
    }
}

pub struct App {
    gl: GlGraphics,
    snake: Snake,
    food: (i32, i32),
    score: i32,
    rng: ThreadRng,
}

impl App {
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let snake = &self.snake;
        let food = self.food;
        let score = self.score;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for &(x, y) in &snake.body {
                let square = rectangle::square(
                    (x as f64) * SQUARE_SIZE,
                    (y as f64) * SQUARE_SIZE,
                    SQUARE_SIZE,
                );
                rectangle(GREEN, square, c.transform, gl);
            }

            let food_square = rectangle::square(
                (food.0 as f64) * SQUARE_SIZE,
                (food.1 as f64) * SQUARE_SIZE,
                SQUARE_SIZE,
            );
            rectangle(RED, food_square, c.transform, gl);

            // Render score
            let score_str = format!("Score: {}", score);
            let transform = c.transform.trans(10.0, 20.0);
            Text::new_color(WHITE, 20)
                .draw(&score_str, glyphs, &c.draw_state, transform, gl)
                .unwrap();
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.snake.head_position() == self.food {
            if let Err(_) = self.snake.update(true) {
                // Grow on success, reset on collision
                self.reset_game();
            } else {
                self.score += 1;
                self.spawn_food();
            }
        } else {
            if let Err(_) = self.snake.update(false) {
                // Don't grow, reset on collision
                self.reset_game();
            }
        }
    }

    fn spawn_food(&mut self) {
        loop {
            let new_food = (
                self.rng.gen_range(0..GRID_WIDTH),
                self.rng.gen_range(0..GRID_HEIGHT),
            );
            if !self.snake.body.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }

    fn reset_game(&mut self) {
        self.snake = Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
        self.spawn_food();
        self.score = 0;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Snake Game", WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2),
        food: (GRID_WIDTH / 4, GRID_HEIGHT / 4),
        score: 0,
        rng: thread_rng(),
    };

    let mut events = Events::new(EventSettings::new());
    events.set_ups(8);

    let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new())
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut glyphs);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => app.snake.change_direction(Direction::Up),
                Key::Down => app.snake.change_direction(Direction::Down),
                Key::Left => app.snake.change_direction(Direction::Left),
                Key::Right => app.snake.change_direction(Direction::Right),
                _ => {} // Ignore other key presses 
            }
        }
    }
}