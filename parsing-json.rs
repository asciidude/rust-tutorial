/*
    This will show you how to parse JSON, aka you take a serialized JSON string and convert
    it to a useable data structure.

    First, include serde, serde_json, and serde_derive packages in your Cargo.toml file
*/

extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use serde_json::Value as JsonValue; // reference as JsonValue

fn main() {
    // #""# = multiline strings
    let json_str = r#"{
        "name": "asciidude",
        "age": 15,
        "is_male": true
    }"#;

    let res = serde_json::from_str(json_str);
    
    // First way
    if res.is_ok() {
        let p: JsonValue = res.unwrap();
        // In most cases you would NOT want to unwrap a JSON string without using an Option
        println!("Name: {0}", p["name"].as_str().unwrap());
        println!("Age: {0}", p["age"]);
        println!("Male?: {0}", p["is_male"]);
    } else {
        println!("Unable to parse JSON");
    }

    // Second way
}