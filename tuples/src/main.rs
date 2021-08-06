// Tuples: lots of variables in one collection

fn main() {
    let tuple1 = (20.5, "RustUp!", true, (1, 4, 7) /*<- nested tuple*/);
    println!("{0}", tuple1.1); // Return index by tuple.index, tuples start at index 0
    println!("{0}", (tuple1.3).2); // Access nested tuples, returns 7

    let tuple2 = (17, 38, "aye"); // B)
    let (a, b, c) = tuple2; // Seperate indexes into variables
    println!("a = {0}, b = {1}, c = {2}", a, b, c);
}