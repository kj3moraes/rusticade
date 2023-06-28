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

    pub fn move_x(&mut self) {
        if self.position.x > 0 && self.position.x < 10 {
            self.position.x += self.direction;
        }
        self.position.x += self.direction;
    }
}

/**
 * BallState defines the state of the ball.
 * - its current position
 * - its current direction
 */
struct BallState {
    pub position: Vec2,
    pub direction: Vec2,
}

impl BallState {
    pub fn new(position: Vec2) -> BallState {
        BallState {
            position,
            direction: Vec2::xy(1, -1),
        }
    }

    pub fn move_ball(&mut self) {
        self.position.x += self.direction.x;
        self.position.y += self.direction.y;
    }

    pub fn bounce_x(&mut self) {
        self.direction.x *= -1;
    }

    pub fn bounce_y(&mut self) {
        self.direction.y *= -1;
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
    pub ball: BallState,
    pub last_ball_movement: usize,
    pub last_bricks_shots: usize,
    pub score: usize,
}


impl GameState {

    pub fn new(dimension: Vec2) -> GameState {

        // Create the bricks relative to the size of the window
        let mut bricks = Vec::new();
        for y in (1..=16).step_by(2) {
            for x in (3.. dimension.x - 6).step_by(3) {
                if x % 2 != 0 {
                    bricks.push(BrickState::new(Vec2::xy(x, y)));
                }
            }
        }

        GameState {
            dimension,
            bouncer: PlayerState { 
                position: Vec2::xy(dimension.x / 2, dimension.y - 2),
                direction: 0,
                misses: 0,
            },
            last_shot_frame: 0,
            bricks: bricks,
            ball: BallState::new(Vec2::xy(dimension.x / 2, dimension.y / 2)),
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
            self.bouncer.direction = direction;
        }
    }

    pub fn update(&mut self) {

        // Move the bouncer
        self.bouncer.move_x();
        self.bouncer.direction = 0;
    
        // Move the ball
        // 1. Ensure that the ball is within bounds
        if self.ball.position.x <= 1 || self.ball.position.x > self.dimension.x - 1 {
            self.ball.bounce_x();
        } else if (self.ball.position.y <= 1) {
            self.ball.bounce_y();
        }
        self.ball.move_ball();


        // 2. Check if the ball hits the bouncer
        if (self.ball.position.y < self.bouncer.position.y - 1) 
            && (self.ball.position.y >= self.bouncer.position.y - 2) 
            && (self.ball.position.x >= self.bouncer.position.x - 2 
                || self.ball.position.x <= self.bouncer.position.x + 2){
            self.ball.bounce_y();
        }

        // 3. Check if the ball hits a brick
        for brick in self.bricks.iter_mut() {
            if brick.alive && self.ball.position == brick.position {
                brick.kill();
                self.ball.bounce_y();
                self.score += 5;
            }
        }
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
        

        // Draw the ball
        if state.ball.position.y > state.dimension.y + 10{
            let msg = &format!("{}  -  score: {}", "dead", state.score);
            pencil.set_origin(win_size / 2 - Vec2::x(msg.len() / 2));
            pencil.draw_text(msg, Vec2::zero());
            return;
        }
        pencil.set_foreground(Color::Blue);
        pencil.draw_rect(&RectCharset::simple_lines(), 
                        state.ball.position, 
                        Vec2::xy(2, 2));

        // Draw the bricks
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
                            Vec2::xy(6, 2));
        }

       
    });
}