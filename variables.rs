/*
    By default, all variables are immutable (can't be changed, constant),
    to change that, after using a variable declaration keyword (eg. let),
    add "mut" keyword which will make it mutable
*/

/*
    (x)8   = 8bit   | i8,
    (x)16  = 16bit  | i16,
    (x)32  = 32bit  | i32,
    (x)64  = 64bit  | i64,
    (x)128 = 128bit | i128

    Primitive Data Types
    i    = signed integer, does support (-) numbers,
    u    = unsigned integer, doesn't support (-) numbers,
    f    = floating point, decimal number (eg. 1.0),
    bool = boolean, true/false,
    char = values written in single quotes, single Unicode Scalar value
*/

fn main() {
    let immutable = 45; // Data type: i32
    println!("{0}", immutable);

    let mut mutable = 45;
    println!("Before {0}", mutable);
    mutable = 80;
    println!("After {0}", mutable);

    let datatype: i64 = 75; // Changing the data type from i32 -> i64
    println!(datatype);
}