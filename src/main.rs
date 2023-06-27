use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};

use rand::{self, prelude::*};

/*
    PlayerState defines the state of the player's bouncer. 
    - its current position
    - which direction it is moving
    - how many shots it has missed
*/
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

/*
    BrickState defines the state of a brick. 
    - its current position
    - whether it is alive or not
*/
struct BrickState {
    pub position: Vec2,
    pub alive: bool,
}

impl BrickState {
    pub fn new(position: Vec2) -> BrickState {
        BrickState {
            position,
            alive: true,
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }
}

struct GameState {
    pub dimension: Vec2,
    pub bouncer: PlayerState,
    pub last_shot_frame: usize,
    pub bricks: Vec<BrickState>,
    pub ball_movement: (i32, bool),
    pub last_ball_movement: usize,
    pub last_bricks_shots: usize,
    pub score: usize,
}


impl GameState {

    pub fn new(dimension: Vec2) -> GameState {

        // Create the bricks relative to the size of the window
        let mut bricks = Vec::new();
        for y in (1..=16).step_by(2) {
            for x in (3..60).step_by(4) {
                if x % 2 != 0 {
                    bricks.push(BrickState::new(Vec2::xy(x, y)));
                }
            }
        }

        GameState {
            dimension,
            bouncer: PlayerState { 
                position: Vec2::xy(1, dimension.y - 2),
                direction: 0,
                misses: 0,
            },
            last_shot_frame: 0,
            bricks: bricks,
            ball_movement: (1, false),
            last_ball_movement: 0,
            last_bricks_shots: 0,
            score: 0,
        }
    }

    pub fn bouncer_move_x(&mut self, direction: i32) {
        if (self.bouncer.position.x - 3 < 0 && direction < 0) 
            || (self.bouncer.position.x + 3 > self.dimension.x && direction > 0) {
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
    
    let gameplay_dimensions = Vec2::xy(win_size.x * 3/4, win_size.y);
    let mut state = GameState::new(gameplay_dimensions);

    app.run(|app_state: &mut State, window: &mut Window| {
        
        // Quit the game if the user presses the ESC key or Q.
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        // Define the pencil
        let mut pencil = Pencil::new(window.canvas_mut());
        
        // Register the movement of the bouncer
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

        // Draw the bouncer
        pencil.set_origin((win_size - state.dimension) / 2);
        pencil.set_foreground(Color::Red);
        pencil.draw_rect(&RectCharset::double_lines(), 
                        state.bouncer.position, 
                        Vec2::xy(state.dimension.x / 10, 2));

        for (i, bricks) in state.bricks.iter().enumerate() {

            // Based on the row, change the colour of the bricks
            if i / 10 < 1 || i / 10 < 2 {
                pencil.set_foreground(Color::Red);
            } else if i / 10 < 3 || i / 10 < 4{
                pencil.set_foreground(Color::Xterm(130));
            } else if i / 10 < 5 {
                pencil.set_foreground(Color::Yellow);
            } else if i / 10 < 6 {
                pencil.set_foreground(Color::Green);
            } else {
                pencil.set_foreground(Color::LightGrey);
            }
            // pencil.set_foreground(Color::LightGrey);
            pencil.draw_rect(&RectCharset::simple_lines(), 
                            (*bricks).position, 
                            Vec2::xy(8, 2));
        }
        
    });
}