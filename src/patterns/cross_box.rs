/// Prints a cross box (X with border) pattern of the given size.
/// The size determines the dimensions of the cross box.
pub fn print_cross_box(size: u32) {
    println!("This is a cross box of size {size}");

    let size = size as i32;
    let diameter = 2 * size + 1;
    
    for row in 0..diameter {
        for col in 0..diameter {
            // Print stars on the two diagonals to form an X
            // Main diagonal: row == col
            // Anti-diagonal: row + col == diameter - 1
            // Also print border edges
            if row == col || row + col == diameter - 1 || row == 0 || col == 0 || row == diameter - 1 || col == diameter - 1 {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

