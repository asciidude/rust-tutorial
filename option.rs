/*
    Option enums represent a value or no value.
*/

fn main() {
    let name = "Your Name";
    println!("Occupation is: {0}", match get_occupation(name) {
        Some(c) => c.to_string(),
        None => "None".to_string()
    });
}

// Return Option enum of type string
fn get_occupation(string: &str) -> Option<&str> {
    match string {
        "Your Name" => Some("Rust Developer"),
        _ => None
    }
}