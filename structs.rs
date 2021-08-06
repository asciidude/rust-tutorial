/*
    Structs are ways to group similar data into one data type
*/

// u8: 0-255, so it is best for RGB color codes
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    // Set "bg" to type Color, inside of the curly braces is where you specify values of the members
    let mut bg = Color { red: 255, green: 70, blue: 15 };
    bg.blue = 45; // Only changeable due to the "mut" keyword!

    println!("{0}, {1}, {2}", bg.red, bg.green, bg.blue); // Access members of Color using bg.member
}