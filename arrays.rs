/*
    Arrays, much like tuples, are a collection of data.
    However, unlike tuples, to access values you must do array[index],
    the index starts at 0.

    Much like vectors, you must use the .iter() function to avoid the values
    being moved.
*/

fn main() {
    // You can specify the data type of the values by doing [data type; length]
    let num_array: [i32; 5] = [0, 1, 2, 3, 4];
    for i in num_array.iter() {
        println!("{0}", i);
    }

    // Another way to loop through the array and print the values is through a range of 0-array's length
    // This can be considered useful in many cases.
    for i in 0..num_array.len() {
        println!("{0}", num_array[i]);
    }

    // You can also initialize an array with default values
    // So first, you put the value for all the values in the array
    // Once you do that, you put a semi-colon, then how many times you want the element to be copied
    let default_array = [2; 400];
    for i in default_array.iter() {
        println!("{0}", i); // Prints "2" 400 times
    }
}