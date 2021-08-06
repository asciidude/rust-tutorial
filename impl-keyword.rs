/*
    The "impl" (otherwise known as implementation) keyword is a way to add methods to a struct to make it more useful,
    that way it is more like an object.
*/

struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    // Specify any functions to be defined
    // The reference to self allows access to the struct "Rect" members
    fn multiply(&self) -> u32 {
        return self.width * self.height;
    }

    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}

fn main() {
    let rect = Rect { width: 10, height: 5 };
    println!("Width * Height: {0} x {1} = {2}", rect.width, rect.height, rect.multiply());
    println!("Is square?: {0}", rect.is_square());
}