/*
    We will make an HTTP-GET request in this.
    This will cover two ways, the first way being the long way (more flexability), and the second being
    the short one-liner.

    First, of course, you need to include "reqwest" in your dependencies.
*/

extern crate reqwest;

fn main() {
    // The long way
    match reqwest::get("https://www.google.com") {
        Ok(mut res) => {
            // 200: OK
            if res.status() == reqwest::StatusCode::Ok {
                match res.text() {
                    Ok(text) => println!("{0}", text),
                    Err(err) => println!("Error: {0}", err)
                }
            } else {
                println!("Response: not OK")
            }
        }
        Err(err) => println!("Error: {0}", err)
    }

    // One-liner
    let res_text = reqwest::get("https://youtube.com/c/asciidude")
        .expect("Unable to make request")
        .text().expect("Unable to read response text");

    println!("{0}", res_text);
}