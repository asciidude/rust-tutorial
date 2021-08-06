/*
    By default, all functions inside of the module are private. On the contrary, you can call
    private functions from a public function.
*/

mod main {
    fn say_bye() {
        println!("Bye now!");
    }

    pub fn say_hello() {
        println!("Hello world!");
        say_bye();
    }

    // We can also nest modules
    pub mod water {
        pub fn liters() {
            println!("2.5 liters of water");
        }
    }
}

fn main() {
    main::say_hello();
    main::water::liters();
}