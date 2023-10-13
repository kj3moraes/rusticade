use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use rand::*;

const MAX_MISSES : usize = 3;

// Checks if the item in the 2D collides with the surface on the same plane.
// Here surface is a vector of 2 points indicating the start and the end of the 
// surface. 
fn check_hit(item: &Vec2, surface: &Vec<Vec2>, is_vertical: bool) -> bool {
    if is_vertical {
        if item.x > surface[0].x && item.x < surface.last().unwrap().x {
            return (item.y - surface[0].y).abs() <= 1;
        }
        false
    } else {
        if item.y > surface[0].y && item.y < surface.last().unwrap().y {
            return (item.x - surface[0].x).abs() <= 1;
        } 
        false
    }   
}

pub fn random_ball_direction() -> Vec2 {
    let mut rng = rand::thread_rng();
    let neg_x: bool = rng.gen();
    Vec2::xy(if neg_x { -1 } else { 1 }, -1 )
}

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
        self.position.x += self.direction;
    }
}

/**
 * BallState defines the state of the ball.
 * - its current position
 * - its current direction
 */
#[derive(Clone, Debug)]
struct BallState {
    pub position: Vec2,
    pub direction: Vec2,
    initial_position: Vec2
}


impl BallState {
    pub fn new(position: Vec2) -> BallState {
        BallState {
            position,
            direction: random_ball_direction(),
            initial_position: position
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

    pub fn reset(&mut self) {
        self.position = self.initial_position.clone();
        self.direction = random_ball_direction(); 
    }
}

/*
    BrickState defines the state of a brick. 
    - its current position
    - whether it is alive or not
*/
#[derive(Clone, Debug)]
struct BrickState {
    pub position: Vec2,
}

impl BrickState {
    pub fn new(position: Vec2) -> BrickState {
        BrickState {
            position,
        }
    }
}

struct GameState {
    pub dimension: Vec2,
    pub bouncer: PlayerState,
    pub bricks: Vec<Vec<BrickState>>,
    pub ball: BallState,
    pub score: usize,
}


impl GameState {

    pub fn new(dimension: Vec2) -> GameState {

        // Create the bricks relative to the size of the window
        let mut bricks = vec![vec![BrickState::new(Vec2::xy(0, 0)); 10]; 8];
        let brick_width = dimension.x / 20;
        for rows in 0..8 as u32 {
            for cols in 0..10 as u32 {
                bricks[rows as usize][cols as usize] = BrickState::new(
                                    Vec2::xy(rows * 2 * brick_width as u32, 
                                                        cols * 2));
            }
        }


        GameState {
            dimension,
            bouncer: PlayerState::new(Vec2::xy(dimension.x / 2 + 10, dimension.y - 2)),
            bricks,
            ball: BallState::new(Vec2::xy(dimension.x / 2, dimension.y)),
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
        } else if self.ball.position.y <= 1 {
            self.ball.bounce_y();
        }

        self.ball.move_ball();

        // 2. Check if the ball hits the bouncer
        if check_hit(&self.ball.position,
                    &vec![self.bouncer.position, self.bouncer.position + Vec2::xy(self.dimension.x / 10, 0)],
                    true) {
            self.ball.bounce_y();
        }

        // 3. Check if the ball hits a brick
        for row in self.bricks.iter_mut() {
            row.retain(|brick| {
                if check_hit(&self.ball.position,
                            &vec![brick.position, brick.position + Vec2::xy(self.dimension.x/10,0)],
                           true) {
                    self.ball.bounce_y();
                    // eprintln!("I hit {:?}", brick);
                    self.score += 1;
                    false
                } else {
                    true
                }
            })
        }
     }

  
}


fn main() {
    let mut app = App::default();
    let win_size = app.window().size();
    let gameplay_dimensions = Vec2::xy(win_size.x, win_size.y);
    let mut state = GameState::new(gameplay_dimensions);
    
    let mut is_game_over : bool = false;
    
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
            
            match key_down {
                // TODO: The speed has to be relative to the window size.
                Key::A | Key::J | Key::Left     => state.bouncer_move_x(-relative_speed),
                Key::D | Key::L | Key::Right    =>  state.bouncer_move_x(relative_speed),
                _ => (),
            }
        }    

        state.update();   

        // Draw the score
        pencil.set_foreground(Color::Green);
        let mut display_text = "score: ".to_owned() + &state.score.to_string().to_owned();
        pencil.draw_text(&display_text, Vec2 { x: 0, y: 0 });
        
        // Draw the misses 
        pencil.set_foreground(Color::Red);
        display_text = "misses: ".to_owned() + &state.bouncer.misses.to_string().to_owned();
        pencil.draw_text(&display_text,
                         Vec2 { x: state.dimension.x, y: 0 });

        // Draw the bouncer
        pencil.set_origin((win_size - state.dimension) / 2);
        pencil.set_foreground(Color::Red);
        pencil.draw_rect(&RectCharset::double_lines(), 
                        state.bouncer.position, 
                        Vec2::xy(state.dimension.x / 10, 2));
        
        // Check that the ball is within bounds 
        // (if it is not then decrease the number of lives left)
        if state.ball.position.y > state.dimension.y + 10 {
            state.bouncer.misses += 1;

            // Have we run out of lives ?
            if state.bouncer.misses == MAX_MISSES {
                is_game_over = true;
            } else {
                // Else, reset the ball at the starting position 
                state.ball.reset();
                pencil.set_foreground(Color::Yellow);
                pencil.draw_rect(&RectCharset::simple_lines(), 
                                state.ball.position, 
                                Vec2::xy(2, 2));
                // Draw the bouncer
                pencil.set_origin((win_size - state.dimension) / 2);
                pencil.set_foreground(Color::Red);
                pencil.draw_rect(&RectCharset::double_lines(), 
                                state.bouncer.position, 
                                Vec2::xy(state.dimension.x / 10, 2));
            }
        }
        
        // If the game is over, print the score and exit.
        if is_game_over {
            let msg = &format!("{}  -  score: {}", "dead", state.score);
            pencil.set_origin(win_size / 2 - Vec2::x(msg.len() / 2));
            pencil.draw_text(msg, Vec2::zero());
            return ();
        }
        
        // Draw the ball
        pencil.set_foreground(Color::Yellow);
        pencil.draw_char('0', state.ball.position);

        // Draw the bricks
        for (row_num, row) in state.bricks.iter().enumerate() {
            for (col_num, _brick) in row.iter().enumerate() {
                match row_num {
                    0..=1 => pencil.set_foreground(Color::Red),
                    2..=3 => pencil.set_foreground(Color::Xterm(166)),
                    4..=5 => pencil.set_foreground(Color::Green),
                    6..=7 => pencil.set_foreground(Color::Yellow),
                    _ => pencil.set_foreground(Color::DarkGrey),
                };
                eprintln!("row_num={row_num}, col_num={col_num}");
                let brick_st = &state.bricks[row_num][col_num];
                pencil.draw_rect(&RectCharset::simple_lines(),
                                Vec2::xy(brick_st.position.x, brick_st.position.y),
                                Vec2::xy(state.dimension.x / 10, 2));
            }
        }
    });
}