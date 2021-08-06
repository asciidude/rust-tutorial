/*
    With tuple structs, you must specify *only* the data types of the members seperated by a comma.
    Accessing the members is simple and like the use of normal tuples.
*/

struct Color(u8, u8, u8);

fn main() {
    let red = Color(255, 0, 0);

    println!("The RGB value of red is {0}, {1}, {2}", red.0, red.1, red.2);
}