// Shadowing, in basic terms, is re-assigning a variable, but not really

fn main() {
    // x = 10 outside of the code blocks
    let x: i8 = 1;
    println!("{0}", x);

    {
        // x = 2 inside of this code block
        // This is called "shadowing", it now only exists inside of this codeblock
        let x: i8 = 2;
        println!("{0}", x);
    }

    {
        // You can also change the data type of "x"
        let x: bool = true;
        println!("{0}", x);
    }
}