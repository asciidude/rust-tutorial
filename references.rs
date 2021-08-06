/* 
    References are just another way to refer to a variable,
    in this variable "y" will be a reference to "x"
*/

fn main() {
    let mut x: i8 = 5;
    let y = &mut x; // Reference x with &x, make reference mutable with "mut"

    println!("y: {0}", y); // Should return same value
    
    // Cool! Now lets try changing the value of "x" through a reference
    // With mutable references, you must put * in front of the variable
    *y = 1;
    println!("y: {0}", y);

    /*
        Reference rule:
        You are not allowed to have both references (aka "borrows") at the same time:
            * one or more references (&T) to a resource
            * exactly one mutable reference (&mut T)
    */
}