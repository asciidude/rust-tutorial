/*
    To declare constants, get out of any and all functions and
    start with the "const" keyword, constants *cannot* be changed
*/

const MAX_NUM: u8 = 5;

fn main() {
    for i in 1..MAX_NUM + 1 {
        println!("{0}", i);
    }
}