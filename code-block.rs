/*
    A codeblock is a piece of code found inside two curly braces,
    it is isolated *but* has access to data outside of the block of code
*/

fn main() {
    let x = 10;

    {
        let y = 5;
        println!("x: {0}, y: {1}", x, y);
    }
}