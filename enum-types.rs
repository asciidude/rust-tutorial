enum Direction {
    X,Y,Z
}

fn main() {
    // Enums, esentially, are just a way to express your code in a descriptive and simple way
    let direction: Direction = Direction::Y;
    
    // The "match" keyword works like a switch statement
    match direction {
        Direction::X => println!("Moving on X axis"),
        Direction::Y => println!("Moving on Y axis"),
        Direction::Z => println!("Moving on Z axis")
    }
}