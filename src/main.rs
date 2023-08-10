use crate::{ascii::run_ascii, canvas::run_canvas};
use std::io;

mod ascii;
mod canvas;

pub fn main() -> Result<(), String> {
    let mut choice: u8;
    'selecting: loop {
        let mut user_input = String::new();
        print!("Select your flavor of donut!\n1. ASCII\n2. Canvas\n0. Quit\n ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line"); // We get `Stdin` here.

        choice = match user_input.trim().parse() {
            Ok(choice) => choice,
            Err(_) => u8::MAX,
        };

        print!("\x1B[2J\x1B[1;1H");
        match choice {
            1 => {
                println!("You've chose ASCII donut");
                break 'selecting;
            }
            2 => {
                println!("You've chose Canvas donut");
                break 'selecting;
            }
            0 => return Ok(()),
            _ => {
                println!("Error choice, number needs to be between 1, 2 and 3");
                continue;
            }
        };
    }

    match choice {
        1 => run_ascii(),
        2 => run_canvas()?,
        _ => (),
    }

    Ok(())
}
