/*
Copyright 2023 Keane Moraes

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/

use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};

use std::io;
use rand::Rng;
use std::cmp::Ordering;


struct ArcadeState {
    pub dimension: Vec2,
    pub games: Vec2,
    pub current_game: usize,
    pub last_game: usize,
    pub name: String,
    pub high_scores: Vec<(String, usize)>,
}


fn secret_num() {
    println!("GUESSING GAME");

    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        println!("You guessed: {}", guess);
    
        let guess: i32 = match guess.trim().parse() {
                                    Ok(num) => num,
                                    Err(_) => continue,
                                };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    

}

impl ArcadeState {

    // FIGUREOUT: How do we get the dimensions of the terminal?
    // How do we implement it as the design that I had. 
    // Maybe start with just a list.
    pub fn new(dimension: Vec2) -> ArcadeState {

        ArcadeState { 
            dimension: dimension,
            games: Vec2::xy(0, 0),
            current_game: 0,
            last_game: 0,
            name: "Space Invaders".to_string(),
            high_scores: Vec::new(), 
        }
    }

    // TODO: Implement this
    pub fn update(&mut self, step: u64) {
        if step % 10 == 0 {
            self.current_game = (self.current_game + 1);
        }
    }
}

fn main() {
    let mut app = App::default();
    let mut state = ArcadeState::new(Vec2::xy(50, 22));
    let mut fps_counter = FPSCounter::default();

   
}