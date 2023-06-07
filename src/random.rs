// use std::io;
// use rand::Rng;

// fn main() {
//     println!("GUESSING GAME");

//     let mut guess = String::new();

//     println!("Please input your guess.");
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
    
//     println!("You guessed: {guess}", guess=guess);

//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("The secret number is: {secret_number}", secret_number=secret_number);

//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("spaces: {spaces}", spaces=spaces);
    
// }
