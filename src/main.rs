use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};

use rand::{self, prelude::*};

struct PlayerState {
    pub position: Vec2,
    pub direction: i32,
    pub misses: usize,
}

impl PlayerState {
    pub fn new(position: Vec2) -> PlayerState {
        PlayerState {
            position,
            direction: 0,
            misses: 0,
        }
    }

    pub fn move_x(&mut self, direction: i32) {
        self.direction = direction;
        if self.position.x > 0 && self.position.x < 10 {
            self.position.x += direction;
        }
        self.position.x += direction;
    }
}


struct GameState {
    pub dimension: Vec2,
    pub bouncer: PlayerState,
    pub last_shot_frame: usize,
    pub bricks: Vec<Vec2>,
    pub bricks_shots: Vec<Vec2>,
    pub ball_movement: (i32, bool),
    pub last_ball_movement: usize,
    pub last_bricks_shots: usize,
    pub score: usize,
}


impl GameState {

    pub fn new(dimension: Vec2) -> GameState {
        let mut bricks = Vec::new();
        for y in 2..10 {
            for x in 5..dimension.x - 5 {
                if x % 2 != 0 {
                    bricks.push(Vec2::xy(x, y));
                }
            }
        }

        GameState {
            dimension,
            bouncer: PlayerState { 
                position: Vec2::xy(1, dimension.y),
                direction: 0,
                misses: 0,
            },
            last_shot_frame: 0,
            bricks: bricks,
            bricks_shots: Vec::new(),
            ball_movement: (1, false),
            last_ball_movement: 0,
            last_bricks_shots: 0,
            score: 0,
        }
    }

    pub fn bouncer_move_x(&mut self, direction: i32) {
        if (self.bouncer.position.x < 0 && direction < 0) 
            || (self.bouncer.position.x > self.dimension.x && direction > 0) {
            self.bouncer.direction = 0;
        } else {
            self.bouncer.move_x(direction);
        }
    }

    pub fn update(&mut self) {
        self.score += 12;
    }

  
}


fn main() {
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

        let mut pencil = Pencil::new(window.canvas_mut());
        for key_down in app_state.keyboard().get_keys_down() {
            let relative_speed = win_size.x / 50;
            let b = state.bouncer.position.x.to_string();
            pencil.draw_text(&b, Vec2 { x: 0, y: 0 });
            match key_down {
                // TODO: The speed has to be relative to the window size.
                Key::A | Key::J => state.bouncer_move_x(-relative_speed),
                Key::D | Key::L =>  state.bouncer_move_x(relative_speed),
                _ => (),
            }
        }

        state.update();
        pencil.set_origin((win_size - state.dimension) / 2);
        pencil.set_foreground(Color::Red);
        pencil.draw_rect(&RectCharset::double_lines(), 
                        state.bouncer.position, 
                        Vec2::xy(state.dimension.x / 10, 2));

        for bricks in &state.bricks {
            pencil.set_foreground(Color::Blue);
            pencil.draw_rect(&RectCharset::simple_round_lines(), 
                            *bricks, 
                            Vec2::xy(state.dimension.x / 10, 2));
        }
        
    });
}