use std::io;

mod patterns;

use patterns::barfi;
use patterns::triangle;
use patterns::cross_box;

fn main() {
    let mut user_input = String::new();

    // Ask user to select pattern
    loop {
        println!("Which pattern would you like to print?");
        println!("1. Barfi (rotated square/diamond)");
        println!("2. Triangle");
        println!("3. Cross Box");
        println!("Enter your choice (1-3):");

        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the user input");

        let choice = user_input.trim();
        if ["1", "2", "3"].contains(&choice) {
            // Get size input
            let size = get_size_input();
            
            match choice {
                "1" => barfi::print_barfi(size),
                "2" => triangle::print_triangle(size),
                "3" => cross_box::print_cross_box(size),
                _ => unreachable!(),
            }
            break;
        } else {
            println!("Invalid choice. Please enter 1, 2, or 3.");
        }
    }
}

fn get_size_input() -> u32 {
    let mut user_input = String::new();
    let mut size: u32;

    loop {
        println!("What should be the size? (1-5)");

        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the user input");

        size = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number from 1 to 5.");
                continue;
            }
        };

        if (1..=5).contains(&size) {
            break;
        }

        println!("The size should be from 1 to 5. Please try again");
    }

    size
}
