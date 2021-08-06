/*
    There are two types of strings, the primitive type and the main string data type.
    This will only cover the main string data type.

    https://doc.rust-lang.org/std/string/struct.String.html
*/

fn main() {
    // There's quite a few methods to use on strings, all easily viewable by code completion or docs.
    let mut string = String::from("Hello world!");
    println!("Length: {0}", string.len());
    println!("Empty?: {0}", string.is_empty());
    println!("Contains 'hello'?: {0}", string.to_lowercase().contains("hello"));
    string.push_str(" How are you?");
    println!("String appended to: {0}", string);

    // Splitting whitespace in strings, it will return an iterator
    for token in string.split_whitespace() {
        println!("{0}", token);
    }
}