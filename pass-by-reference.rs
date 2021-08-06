struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let blue = Color { red: 0, green: 0, blue: 255 };
    print_color(&blue); // To use a reference as a paremeter, we must pass a reference for the variable as well
    // If we don't pass by reference, we can't call print_color() anymore! That's because the value of blue has moved to a different scope.
}

// &Color = reference to color, the compiler will accept it as a parameter
fn print_color(c: &Color) {
    println!("{0}, {1}, {2}", c.red, c.green, c.blue);
}