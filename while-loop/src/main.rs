fn main() {
    let mut x = 1;

    while x <= 50 {
        // If x is a multiple of 5, print x = x
        if x % 5 == 0 {
            println!("x = {0}", x);
        }

        x += 1;
    }
}
