/// Prints a diamond pattern (rotated square) of the given size.
/// The size determines the "radius" of the diamond (distance from center to edge).
pub fn print_barfi(size: u32) {
    println!("This is a barfi of size {size}");

    // Draw a rotated square (diamond) with Manhattan-distance math.
    let size = size as i32;
    let diameter = 2 * size + 1;
    let center = size;

    for row in 0..diameter {
        for col in 0..diameter {
            let dist = (row - center).abs() + (col - center).abs();
            if dist <= size {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

