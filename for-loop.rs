fn main() {
    // Print numbers from 1-10
    // NOTE #0: All for loops require an iterator
    // NOTE #1: x..y = range
    let numbers = 1..11;
    for i in numbers {
        println!("{0}", i);
    }

    // Print all elements in a vector with the indexes
    // NOTE #0: Not using .iter() with vectors means the ownership of the values will be moved to the for loop
    let vector = vec!["hello", "beautiful", "world"];
    for (i, v) in vector.iter().enumerate() { // iter = iterator
        println!("{0} | {1}", v, i);
    }
}