use rand::Rng;
use crate::app::config::AppConfig;
use crate::audio::SoundEffect;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct GameState {
    pub score: u32,
    pub game_over: bool,
}

pub struct Game {
    pub snake: Vec<Position>,
    pub food: Position,
    pub direction: (i32, i32),
    pub score: u32,
    pub game_over: bool,
    pub last_update: Instant,
    pub update_interval: Duration,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new() -> Self {
        let width = 20;
        let height = 15;
        let mut rng = rand::thread_rng();
        
        Self {
            snake: vec![Position { x: width / 2, y: height / 2 }],
            food: Position {
                x: rng.gen_range(1..width-1),
                y: rng.gen_range(1..height-1),
            },
            direction: (1, 0),
            score: 0,
            game_over: false,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(150),
            width,
            height,
        }
    }
    
    pub fn new_with_config(config: &AppConfig) -> Self {
        let width = config.board_width;
        let height = config.board_height;
        let mut rng = rand::thread_rng();

        Self {
            snake: vec![Position { x: width / 2, y: height / 2 }],
            food: Position {
                x: rng.gen_range(1..width.saturating_sub(1)),
                y: rng.gen_range(1..height.saturating_sub(1)),
            },
            direction: (1, 0),
            score: 0,
            game_over: false,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(config.game_speed),
            width,
            height,
        }
    }

    pub fn update(&mut self) -> Option<SoundEffect> {
        if self.game_over {
            return None;
        }
        
        let mut sound_to_play = None;
        if self.last_update.elapsed() >= self.update_interval {
            // Calculate new head position
            let head = self.snake[0];
            let new_head_x = head.x as i32 + self.direction.0;
            let new_head_y = head.y as i32 + self.direction.1;
            
            // Check collision with walls
            if new_head_x < 0 || new_head_x >= self.width as i32 || new_head_y < 0 || new_head_y >= self.height as i32 {
                self.game_over = true;
                return Some(SoundEffect::GameOver);
            }

            let new_head = Position {
                x: new_head_x as usize,
                y: new_head_y as usize,
            };
            
            // Check collision with self
            if self.snake.contains(&new_head) {
                self.game_over = true;
                return Some(SoundEffect::GameOver);
            }
            
            // Move snake
            self.snake.insert(0, new_head);
            
            // Check if food eaten
            if new_head == self.food {
                self.score += 10;
                self.place_food();
                sound_to_play = Some(SoundEffect::Eat);
            } else {
                self.snake.pop();
            }
            
            self.last_update = Instant::now();
        }
        sound_to_play
    }
    
    pub fn change_direction(&mut self, direction: (i32, i32)) {
        // Prevent 180-degree turns
        if (self.direction.0 + direction.0 != 0) || (self.direction.1 + direction.1 != 0) {
            self.direction = direction;
        }
    }
    
    fn place_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let new_food = Position {
                x: rng.gen_range(1..self.width.saturating_sub(1)),
                y: rng.gen_range(1..self.height.saturating_sub(1)),
            };
            if !self.snake.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }
}