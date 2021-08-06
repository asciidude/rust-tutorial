/*
    These are just five more string methods to use in Rust, which will become useful later on.
*/

fn main() {
    let string = String::from("Hello world!\n\rWhat's up?\n\rStar,the,GitHub!");

    // Replace: replaces word(s) with given replace string
    {
        println!("{0}", string.replace("world", "compiler"));
    }

    // Lines: splits string to iterator for each line
    {
        for i in string.lines() {
            println!("[{0}]", i);
        }
    }

    // Split: splits string based off delimeter
    {
        // Split returns an iterator, you can call .collect() to return a vector of string slices
        let tokens: Vec<&str> = string.split(",").collect();
        println!("Index 2: {0}", tokens[2]);
    }

    // Trim: simply trims the string of whitespace
    {
        println!("{0}", string.trim());
    }

    // Characters: accessing iterator of characters in string
    {
        // Using match and .nth() to find index
        match string.chars().nth(4) {
            Some(c) => println!("Index 4: {0}", c),
            None => println!("No character found!")
        }
    }
}