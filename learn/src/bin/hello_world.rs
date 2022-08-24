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

        // Use arguments from context.
        // let number = 1i32;
        // let number: i32 = 1;
        let number = 3;
        let width = 5usize;
        println!("number = {number:>width$}");

        // Control the number of decimal places shown.
        let pi = 3.1415926;
        println!("Pi is roughly {:*<12}", pi);
        println!("Pi is roughly {:.2}", pi);
        println!("Pi is roughly {:0>9.2}", pi);
    }
}

// 1.2.1 Debug
// Trait fmt::Debug use the {:?} marker.
// Trait fmt::Display use the {} marker.
// The std library implement fmt::Display trait for all types in std library.
// Implementing the fmt::Dispaly trait automatically implements the ToString trait
// which allow us to convert the type to String.

mod debug {

    // This structure cannot be printed either with 'fmt::Display' or
    // with 'fmt::Debug'.
    struct UnPrintable(u32);

    // The 'derive' attribute automatically creates the implementaion
    // required to make this 'struct' printable with 'fmt::Debug'.
    #[derive(Debug)]
    struct Printable(u32);

    // Put a 'Structure' inside of another 'Structure'.
    #[derive(Debug)]
    struct Deep(Printable);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    pub fn mod_main() {
        // Printing with '{:?}' is similler to with '{}'.
        println!("{:?} months in a year. {} months in a year.", 12, 12);

        println!("Now {:?} is printable.", Printable(7));

        println!("Now {:?} is printable.", Deep(Printable(9)));

        // Pretty printting with '{:#?}'
        println!("Pretty print:\n{:#?}", Deep(Printable(9)));

        let name = String::from("Peter");
        let age = 26;
        let peter = Person { name, age };
        println!("Pretty print:\n{:#?}", peter);
    }
}

// 1.2.2 Display
// manually implement 'fmt::Display'
mod display {
    use std::fmt;

    struct Structure(i32);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Structure({})", self.0)
        }
    }

    pub fn mod_main() {
        println!("Print custom type {}", Structure(66));

        let real = 3.3;
        let imag = 7.2;

        println!("{}", Complex { real, imag });
        println!("{:?}", Complex { real, imag });
    }
}

// 1.2.2.1 Print List
// Use ? operator to see if a function or a macro errors.
// If it errors, return the error. Otherwise continue.
// func()?;

mod print_list {
    use std::fmt;
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ",")?;
                }
                write!(f, "{}", v)?;
            }

            write!(f, "]")
        }
    }

    pub fn mod_main() {
        let v = List(vec![1, 2, 3]);
        println!("{}", v);

        let v = vec![1, 2, 3];
        // not allowed: println!("{}", v);
    }
}

// 1.2.3 Formatting
mod formatting {

    use std::fmt;
    #[derive(Debug)]
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ",")?;
                }
                write!(f, "{}", v)?;
            }

            write!(f, "]")
        }
    }

    pub fn mod_main() {
        let v = List(vec![1, 2, 3]);
        let s = format!("{:?}", v);
        println!("{}", s);
    }
}

fn main() {
    hello_world::mod_main();
    comments::mod_main();
    formatted_print::mod_main();
    debug::mod_main();
    display::mod_main();
    print_list::mod_main();
    formatting::mod_main();
}
