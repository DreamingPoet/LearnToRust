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
    pub fn mod_main() {
        let number = 13;

        // match range is all value of i32
        match number {
            // match a single value
            1 => println!("One!"),
            // match several values
            3 | 5 | 7 | 9 | 11 => println!("this is prime"),
            // match a exclusive range
            // 13..19 => println!("A teen"),
            // match a inclusive range
            13..=19 => println!("A teen"),
            _ => println!("the rest values of i32, which I don't care!"),
        }

        let boolean = true;
        let binary = match boolean {
            true => 0,
            false => 1,
        };

        println!("{} -> {}", boolean, binary);
    }
}
// 8.5.1. Destructuring
// A match block can destructure items in a variety of ways.
// Destructuring Tuples
// Destructuring Arrays and Slices
// Destructuring Enums
// Destructuring Pointers
// Destructuring Structures

mod destructuring {
    pub fn mod_main() {}
}

// 8.5.1.1. tuples
mod tuples {
    pub fn mod_main() {
        let triple = (1, 2, 3);
        match triple {
            (0, x, y) => println!("First is 0, x is {}, y is {}", x, y),
            (1, ..) => println!("First is 1 and the rest doesn't matter!"),
            _ => println!("It doesn't matter what they are"),
        }
    }
}
// 8.5.1.2. arrays/slices
mod arrays_slices {
    pub fn mod_main() {
        let array = [1, 2, 3];
        match array {
            // single values can be ignored with _
            [first, _, third] => println!("First is {}, third is {}", first, third),
            [0, second, third] => println!("Second is {}, third is {}", second, third),
            // you can also bind some and ignore the rest
            [-1, second, ..] => println!(""),
            // or store them in another array/slice (the type depends on that of the value that is being matched against)
            [3, second, tail @ ..] => println!("tail[0] is {}, tail[1] is {}", tail[0], tail[1]),
        }

        // slice
        let array = &array[..];
        match array {
            // single values can be ignored with _
            [first, _, third] => println!("First is {}, third is {}", first, third),
            [0, second, third] => println!("Second is {}, third is {}", second, third),
            // you can also bind some and ignore the rest
            [-1, second, ..] => println!(""),
            // or store them in another array/slice (the type depends on that of the value that is being matched against)
            [3, second, tail @ ..] => println!("tail[0] is {}, tail[1] is {}", tail[0], tail[1]),
            [] => println!(" "),
            _ => println!(" "),
        }
    }
}
// 8.5.1.3. enums
mod enums {
    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    pub fn mod_main() {
        let color = Color::RGB(122, 17, 40);
        // An `enum` can be destructured using a `match`.
        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
            Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) => println!(
                "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k
            ),
            // Don't need another arm because all variants have been examined
        }
    }
}
// 8.5.1.4. pointers/ref
// For pointers, a distinction needs to be made between destructuring and dereferencing
// as they are different concepts which are used differently from languages like C/C++.
// Dereferencing uses *
// Destructuring uses &, ref, and ref mut
mod pointers_ref {
    pub fn mod_main() {

        // Assign a reference of type i32
        let reference = &4;
        match reference {
            // If 'reference' is pattern matched against '&val', it results
            // in a comparison like:
            // '&i32' (type of 'reference')
            // '&val' (pattern for match)
            // We see that if the matching '&'s are droped, then the 'i32'
            // should be assigned to 'val'.
            // 'val' represent the value which 'reference' referenced to
            &val => println!("Got a value via destructuring: {:?}", val), 
        }

        // To avoid the '&', you dereference before matching.
        match *reference {
            val => println!("Got a value via dereferencing: {:?}", val),
        }

        // another way to assign a reference
        let _not_a_reference = 3;
        let ref _is_a_referecne = 3;

        // defining values without reference, references can be retrived via 'ref' and 'ref mut'
        let value = 5;
        let mut mut_value = 6;
        
        match value {
            ref r => println!("Got a reference to a value: {:?}", r),
        }

        match mut_value {
            ref mut rm => {
                *rm += 10;
                println!("Got a mut reference to a mut value: {:?}", rm);
            },
        }

        let value = 5;
        
        let ref rm_val = value;
        // cannot borrow 'value' as mutable
        // let ref mut rm_val = value;
        


        let mut mut_value = 6;
        let ref r = mut_value;
        
        let _type_check: i32 = *r;
        // error, r is a immutable reference, so the data it refers to cannot be written
        // *r += 10;

        // rust derefers automatically
        let b = r + 10;
        let b = *r + 10;
        // error, r is immtable
        // r = &10;

        
        let ref mut rm = mut_value;
        let _type_check: i32 = *rm;

        // cannot add {interger} to &mut i32
        // let b = rm + 10;
        let b = *rm + 10;

        // when dereferencing we got a variable which refers to a data
        // we can regard '*rm' as a new variable
        *rm += 10;
        println!("now mut_value = {}", mut_value);

        
    }
}
// 8.5.1.5. structs
mod structs {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    
    pub fn mod_main() {

        let foo = Foo{x:(1,2), y:3};
        match foo {
            Foo {x: (1, b), y: a} => println!("{}, {}", b, a),
            Foo {y, .. } => println!("we donnot care about x, y = {}", y),
            _ => println!("the rest"),
            
        }

    }
}
// 8.5.2. Guards
// A match guards can be added to filter
mod guards {
    pub fn mod_main() {
        let pair = (2, -2);
        match pair {
            (x, y) if x == y => println!("They are twins"),
            _ => println!("No correlation..."),
        }

        let number: u8 = 4;

        // Note that the compiler does not check arbitrary expressions for whether
        // all possible conditions have been checked.
        // Therefore, you must use the _ pattern at the end.
        match number {
            i if i == 0 => println!("Zero"),
            i if i > 0 => println!("Greater than zero"),
            _ => println!("Fell through"), // This should not be possible to reach
        }


    }
}
// 8.5.3. Binding
// match provides the @ sigil for binding values to names
mod binding {
    fn age() -> u32 {
        15
    }

    fn some_number() -> Option<u32>{
        Some(42)

    }
    pub fn mod_main() {

        match age() {
            0 => println!("I haven't celebrated my first birthday yet"),
            n @ 1..=12 => print!("I'm a child of age {}", n),
            n  => print!("I'm an old person of age {:?}", n),
        }

        match some_number() {
            
            Some(n@ 42..=50) => println!("The number is {}, in range 42 to 50", n),
            Some(n) => println!("Any other number."),
            // match anything else (None)
            _ => (),

        }

    }
}
// 8.6. if let
// 'if let' is clear for only one condition matched and in addition allows various failure options to be specified.
mod if_let {

    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    pub fn mod_main() {
        let number = Some(7);
        let boolean = false;
        
        if let Some(i) = number {
            println!("Matched {}", i);

        }else if boolean{
            println!("Not matched");
        }else{

        }
        // if let can be used to match any enum value
        let a = Foo::Bar;
        let b = Foo::Qux(6);

        if let Foo::Bar = a {
            println!("a is Bar");
        }

        if let Foo::Qux(value) = b {
            println!("value is {}", value);
        }

        if let Foo::Qux(n@ 6) = b {
            println!("b's value is {}", n );
        }

    }
}
// 8.7. while let
mod while_let {
    pub fn mod_main() {
        let optional = Some(7);

        loop {
            match optional {
                _ => {break;}             
            }
        }

        while let Some(7) = optional {
            break;
        } 


    }
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
