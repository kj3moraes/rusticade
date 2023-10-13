use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use array2d::Array2D;


const MAX_MISSES : usize = 3;

fn check_hit(item1: &Vec2, item2: &Vec2, dimension: &Vec2) -> bool {
    if (item1.y < item2.y - 1 && item1.y >= item2.y - 2) && (item1.x >= item2.x - item1.x / 10 && item2.x <= item1.x + dimension.x / 10) {
        true
    } else {
        false
    }
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
            direction: Vec2::xy( 1 , -1),
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
        self.direction = Vec2::xy( 1 , -1); 
    }
}

/*
    BrickState defines the state of a brick. 
    - its current position
    - whether it is alive or not
*/

#[derive(Clone)]
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
    pub bricks: Array2D<BrickState>,
    pub ball: BallState,
    pub score: usize,
}


impl GameState {

    pub fn new(dimension: Vec2) -> GameState {

        // Create the bricks relative to the size of the window
        let mut bricks = Array2D::filled_with(BrickState::new(Vec2::xy(0, 0)),
                                                                    10,
                                                                    8);
        for y in (1..=16).step_by(2) {
            for x in (1..=30).step_by(3) {
                bricks[(x / 3, y / 2)] = BrickState::new(Vec2::xy(x, y));
            }
        }

        GameState {
            dimension,
            bouncer: PlayerState::new(Vec2::xy(dimension.x / 2, dimension.y - 2)),
            bricks: bricks,
            ball: BallState::new(Vec2::xy(dimension.x / 2, dimension.y / 2)),
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
        if check_hit(&self.ball.position, &self.bouncer.position, &self.dimension) {
            self.ball.bounce_y();
        }

        // 3. Check if the ball hits a brick
        for mut row in self.bricks.as_rows() {
            row.retain(|brick| {
                if check_hit(&self.ball.position, &brick.position, &self.dimension) {
                    self.ball.bounce_y();
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
    let gameplay_dimensions = Vec2::xy(win_size.x * 3/4, win_size.y);
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
        pencil.draw_char('O', state.ball.position);

        // Draw the bricks
        for row in state.bricks.as_rows() {
            for brick in row {
                pencil.draw_rect(&RectCharset::simple_lines(),
                                brick.position,
                                Vec2::xy(2, 2));
            }
        }
        println!("{{state.bricks.as_rows()}}");
    });
}