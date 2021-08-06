fn main() {
    let mut n: i32 = 20;

    // Loop until "n" is greater than or equal to 30
    // NOTE #0: Must break out of loops
    // NOTE #1: You can skip iterations with "continue" keyword
    loop {
        if n < 30 {
            println!("n < 30");
            n = 30;
        } else if n == 30 {
            println!("n = 30");
            break;
        } else {
            println!("n > 30");
            break;
        }
    }
}