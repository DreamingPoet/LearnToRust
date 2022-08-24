// 1 Hello World
mod hello_world {
    pub fn mod_main() {
        println!("hello world!");
    }
}

// 1.1 Comments
mod comments {

    /// Generate library docs for the following item.
    /// This is the main function of this mod.
    pub fn mod_main() {
        
        // Line comments which go to the end of the line.
        /* Block comments which go to the closing delimiter. */
        // MARK: xxx, 
        // MARK: -, 
        // TODO: xxx, 
        // FIXME: xxx
    }
}

// 1.2 Formatted print

// format! : write formatted text to Sting.
// print! : same as format! but the text is printed to the console (io::stdout).
// println! : same as print! but a new line is appended.
// eprint! : same as format! but the text is printed to the standed error(io::stderr).
// eprintln! : same as eprint! but a new line is appended.

mod formatted_print {
    pub fn mod_main() {
        // The '{}' is automaticlly replaced with any argument.
        println!("{} days", 31);

        // Posional arguments can be used.
        println!("{0}'s name is {0}, his father is {1}!", "Alan", "Bob");

        // As can named arguments.
        println!(
            "{name}'s name is {name}, his father is {father}!",
            name = "Alan",
            father = "Bob"
        );

        // Special formatting can be specified after a ':'.
        // b for binary, o for otc, x for hex
        println!("Base 10 repr:         {}", 69420);
        println!("Base 2  repr:         {:b}", 69420);
        println!("Base 8  repr:         {:o}", 69420);
        println!("Base 16 repr:         {:x}", 69420);
        println!("Base 16 repr:         {:X}", 69420);

        // Right-align text with a special width. This will output "number =      1"
        println!("number = {number:5}", number = 1);

        // Right-align text with a special width use argument. This will output "number =      1"
        println!("number = {number:width$}", number = 1, width = 5);

        println!("number = {:05}", 1);
        println!("number = {:0>5}", 1);
        println!("number = {:0<5}", 1);
        println!("number = {:*^6}", 1);
        println!("number = {:15}", 2);
        println!("number = {:1^5}", 2);

        let number = 1i32;
        let number: i32 = 1;
        let number = 3;
        let width = 5usize;

        println!("number = {number:>width$}");

        let pi = 3.1415926;
        println!("Pi is roughly {:*<12}", pi);
        println!("Pi is roughly {:.2}", pi);
        println!("Pi is roughly {:0>9.2}", pi);
    }
}

// 1.21 Debug
// Trait fmt::Debug use the {:?} marker.
// Trait fmt::Display use the {} marker.
// The std library implement fmt::Display trait for all types in std library.
// Implementing the fmt::Dispaly trait automatically implements the ToString trait
// which allow us to convert the type to String.

mod debug {

    //
    struct UnPrintable(u32);

    pub fn mod_main() {


    }
}




fn main() {

    hello_world::mod_main();
    comments::mod_main();
    formatted_print::mod_main();


}
