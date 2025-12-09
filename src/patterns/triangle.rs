/// Prints a triangle pattern of the given size.
/// The size determines the height of the triangle.
pub fn print_triangle(size: u32) {
    println!("This is a triangle of size {size}");

    let size = size as i32;
    
    for row in 0..size {
        // Calculate spaces before stars (for centering)
        let spaces = size - row - 1;
        let stars = 2 * row + 1;
        
        // Print leading spaces
        for _ in 0..spaces {
            print!("  ");
        }
        
        // Print stars
        for _ in 0..stars {
            print!("* ");
        }
        
        println!();
    }
}

