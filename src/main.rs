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
                    bricks.push(Vec2::xy(x, y));
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

    pub fn bouncer_move_x(&mut self, displacement: i32) {
        if displacement < 0 && self.bouncer.x != 0
        || displacement > 0 && self.bouncer.x != self.dimension.x 
        {
            self.bouncer.x += displacement;
        }
    }

    pub fn random_ball_direction() -> Vec2 {
        let mut rng = rand::thread_rng();
        let neg_x: bool = rng.gen();
        let neg_y: bool = rng.gen();
        Vec2::xy(if neg_x { -1 } else { 1 }, if neg_y { -1 } else { 1 })
    }

    pub fn update(&mut self) {
        self.score += 12;
        // pencil.draw_text("left", Vec2::xy(0, 0));
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

        
        // let win_size = window.size();
        let mut pencil = Pencil::new(window.canvas_mut());
        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::A | Key::H => state.bouncer_move_x(-1),
                Key::D | Key::L =>  state.bouncer_move_x(1),
                Key::Space => state.bouncer_move_x(2),
                _ => (),
            }
        }
        pencil.draw_text(
            &format!("lives: {}  -  score: {}", state.misses, state.score),
            Vec2::xy(15, 0),
        );
        state.update();

        pencil.set_origin((win_size - state.dimension) / 2);
        pencil.set_foreground(Color::Cyan);
        pencil.draw_char('^', state.bouncer);
    });
}