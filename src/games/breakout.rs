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
        let mut bricks = Vec::new();
        for y in 2..10 {
            for x in 5..dimension.x - 5 {
                if x % 2 != 0 {
                    aliens.push(Vec2::xy(x, y));
                }
            }
        }

        GameState {
            dimension,
            bouncer: Vec2::xy(dimension.x / 2, dimension.y - 2),
            last_shot_frame: 0,
            bricks: bricks,
            bricks_shots: Vec::new(),
            ball_movement: (1, false),
            last_ball_movement: 0,
            last_bricks_shots: 0,
            misses: 0,
            score: 0,
        }
    }

    pub fn random_ball_direction() -> Vec2 {
        let mut rng = rand::thread_rng();
        let neg_x: bool = rng.gen();
        let neg_y: bool = rng.gen();
        Vec2::xy(if neg_x { -1 } else { 1 }, if neg_y { -1 } else { 1 })
    }

    pub fn update(&mut self, frame: usize) {

    }

  
}


fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();
    let win_size = app.window().size();
    let mut state = GameState::new((win_size * 4) / 5);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::W => state.left_player.direction = -1,
                Key::S => state.left_player.direction = 1,
                Key::O => state.right_player.direction = -1,
                Key::L => state.right_player.direction = 1,
                _ => (),
            }
        }
    });
}