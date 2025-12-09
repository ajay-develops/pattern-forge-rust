use std::io;

fn main() {
    println!("Hello, world!");


    let mut user_input = String::new();
    let mut size_of_barfi: u32;
    

    loop {
        println!("what should be the length of barfi?");
        
        io::stdin().read_line(&mut user_input).expect("Failed to read the user input");

        size_of_barfi = user_input.trim().parse().expect("The size of barfi should be a positive input from 1 to 5"); 
        
        if size_of_barfi >= 1 && size_of_barfi <= 5 {
            break;    
        }

        println!("The size of barfi should be from 1 to 5. Please try again");

        continue;
    } 


    println!("This is a barfi of size {size_of_barfi}");

    println!("This pattern prints a 'barfi' structure");


}
