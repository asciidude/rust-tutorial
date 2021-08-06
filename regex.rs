/*
    To use Regex, you must include it by the Cargo.toml file, you must also use the Regex struct. (regex::Regex)
*/

extern crate regex;
use regex::Regex;

fn main() {
    // To use the Regex struct you must also use a raw string, raw strings don't count for the escape sequences
    // Let's also get the matched text using a capture, wrapping text in an opening and closing brace is a capture
    let regex_struct = Regex::new(r"(\w{5})").unwrap(); // This will match a five letter word
    let text = "hello";

    println!("Match?: {0}", regex_struct.is_match(text));

    match regex_struct.captures(text) {
        Some(c) => println!("Capture(s): {0}", c.get(0).unwrap().as_str()), // Gets capture at index 0, unwraps it, and returns string
        None => println!("No captures found!")
    }
}