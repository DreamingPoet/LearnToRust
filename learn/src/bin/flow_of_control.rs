// 8. Flow of Control

// 8.1. if/else
mod if_else {
    pub fn mod_main() {
        let mut n = 5;
        if n < 0 {
            print!("{} is negative", n);
        } else if n > 0 {
            print!("{} is positive", n);
        } else {
            print!("{} is zero", n);
        }

        let a = if n == 5 { 66 } else { 77 };
    }
}
// 8.2. loop
mod loop_ {
    pub fn mod_main() {
        let mut count = 0u32;
        loop {
            count += 1;
            if count == 3 {
                continue;
            }
            if count == 5 {
                break;
            }
        }
    }
}
// 8.2.1. Nesting and labels
mod nesting_and_labels {
    pub fn mod_main() {
        'outer: loop {
            'inner: loop {
                // This would break only the inner loop
                //break;
                break 'outer;
            }
        }
    }
}
// 8.2.2. Returning from loops
mod returning_from_loops {
    pub fn mod_main() {
        let mut counter: u32 = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        assert_eq!(result, 20);
    }
}
// 8.3. while
mod while_ {
    pub fn mod_main() {
        // A counter variable
        let mut n = 1;

        // Loop while `n` is less than 101
        while n < 31 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }

            // Increment counter
            n += 1;
        }
    }
}
// 8.4. for and range
// One of the easiest ways to create an iterator is to use the range notation a..b .
// The a..b yields values from a (inclusive) to b (exclusive) in step of one.
// The a..=b is inclusive on both ends.
mod for_and_range {
    pub fn mod_main() {

        for i in 1..11 {
            println!("counter is {}", i);
        }

        for i in 1..=10 {
            // same as the for loop before
        }


        let names = vec!["Bob", "Frank", "Ferris"];
        for name in names {
            println!("{}", name);
        }

        let names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter() {
            println!("{}", &name);
        }
    }
}
// 8.5. match
mod match_ {
    pub fn mod_main() {}
}
// 8.5.1. Destructuring
mod destructuring {
    pub fn mod_main() {}
}
// 8.5.1.1. tuples
mod tuples {
    pub fn mod_main() {}
}
// 8.5.1.2. arrays/slices
mod arrays_slices {
    pub fn mod_main() {}
}
// 8.5.1.3. enums
mod enums {
    pub fn mod_main() {}
}
// 8.5.1.4. pointers/ref
mod pointers_ref {
    pub fn mod_main() {}
}
// 8.5.1.5. structs
mod structs {
    pub fn mod_main() {}
}
// 8.5.2. Guards
mod guards {
    pub fn mod_main() {}
}
// 8.5.3. Binding
mod binding {
    pub fn mod_main() {}
}
// 8.6. if let
mod if_let {
    pub fn mod_main() {}
}
// 8.7. while let
mod while_let {
    pub fn mod_main() {}
}

fn main() {
    if_else::mod_main();
    loop_::mod_main();
    nesting_and_labels::mod_main();
    returning_from_loops::mod_main();
    while_::mod_main();
    for_and_range::mod_main();
    match_::mod_main();
    destructuring::mod_main();
    tuples::mod_main();
    arrays_slices::mod_main();
    enums::mod_main();
    pointers_ref::mod_main();
    structs::mod_main();
    guards::mod_main();
    binding::mod_main();
    if_let::mod_main();
    while_let::mod_main();
}
