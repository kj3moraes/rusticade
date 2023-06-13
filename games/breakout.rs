use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};

use rand::{self, prelude::*};


struct GameState {
    pub dimension: Vec2,
    pub bouncer: Vec2,
    pub last_shot_frame: usize,
    pub bricks: Vec<Vec2>,
    pub bricks_shots: Vec<Vec2>,
    pub ball_movement: (i32, bool),
    pub last_ball_movement: usize,
    pub last_bricks_shots: usize,
    pub misses: usize,
    pub score: usize,
}



impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        let mut aliens = Vec::new();
        for y in 2..7 {
            for x in 5..dimension.x - 5 {
                if x % 2 != 0 {
                    aliens.push(Vec2::xy(x, y));
                }
            }
        }
        // GameState {
        //     dimension,
        //     spaceship: Vec2::xy(dimension.x / 2, dimension.y - 2),
        //     spaceship_shots: Vec::new(),
        //     last_shot_frame: 0,
        //     aliens,
        //     aliens_shots: Vec::new(),
        //     aliens_movement: (1, false),
        //     last_aliens_movement: 0,
        //     last_aliens_shots: 0,
        //     lives: 3,
        //     score: 0,
        // }
    }

  
}