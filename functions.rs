fn main() {
    print_to(500);
}

fn print_to(num: u64) {
    for i in 1..num + 1 {
        if is_even(i) {
            println!("{0}", i)
        }
    }
}

// Returning values, example: fn name(parameters) -> data_type_being_returned {}
fn is_even(num: u64) -> bool {
    return num % 2 == 0;
}