// 2 Primitives
// Rust provides access to a wide variety of primitives (primitive types).

// Scalar Types
// signed integers: i8, i16, i32, i64, i128, isize(pointer size)
// unsigned integers: u8, u16, u32, u64, u128, usize(pointer size)
// floating point: f32, f64
// char Unicode scalar values like 'a', 'α', '∞' (4 bytes each)
// bool either true or false (boolean)
// unit type (), whose only possible values is an tuple: ()

// Compound Types
// arrays like [1, 2, 3]
// tuples like (1, ture)

mod primitives {
    use std::mem;

    pub fn mod_main() {
        // Variables can be type annotated.
        let logical: bool = true;

        let a_float: f64 = 1.0; // Regular annotation
        let an_integer = 5i32; // Suffix annotation

        // Or a default will be used.
        let default_float = 3.0; // `f64`
        let default_integer = 7; // `i32`

        // A type can also be inferred from context
        let mut inferred_type = 12; // Type i64 is inferred from another line
        inferred_type = 4294967296i64;

        // A mutable variable's value can be changed.
        let mut mutable = 12; // Mutable `i32`
        mutable = 21;

        // Error! The type of a variable can't be changed.
        // mutable = true;

        // Variables can be overwritten with shadowing.
        let mutable = true;

        let charater: char = 'a';
        println!("{}", charater);
        println!("char occupies {} bytes", mem::size_of_val(&charater));
        
        let charater: String = String::from("a");
        println!("{}", charater);
        println!("String occupies {} bytes", mem::size_of::<String>());
        println!("String occupies {} bytes", mem::size_of_val(&charater));


    }
}

// 2.1. Literals and operators

mod literals_and_operations {
    pub fn mod_main() {
        // Integer addition
        println!("1 + 2 = {}", 1u32 + 2);

        // Integer subtraction
        println!("1 - 2 = {}", 1i32 - 2);
        // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

        // Short-circuiting boolean logic
        println!("true AND false is {}", true && false);
        println!("true OR false is {}", true || false);
        println!("NOT true is {}", !true);

        // Bitwise operations
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

        // Use underscores to improve readability!
        println!("One million is written as {}", 1_000_000u32);

        let _default_float = 3.0; // f64
        let _default_ineger = 7; // i32
        let _suffix_integer = 5i32; // i32
        let _suffix_float = 5f64; // f64
        let _nums = 1_000_000; // same as 1000000
        let _nums_deciemal = 0.000_01; // same as 0.00001
    }
}

// 2.2. Tuples

mod tuples {
    pub fn mod_main() {
        fn reverse(pair: (i32, bool)) -> (bool, i32) {
            let (int, boolean) = pair;
            (boolean, int)
        }

        let pair = (6, false);
        println!(" reverse tuple {:?} is {:?}", pair, reverse(pair));
    }
}

// 2.3. Arrays and Slices

mod arrays_and_slices {
    use std::mem;

    pub fn mod_main() {

        fn analyze_silce(slice: &[i32]) {
            println!("first element of this slice : {}", slice[0]);
            println!("the length of this silce: {}", slice.len());
        }

        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        let ys: [i32; 500] = [0; 500];

        println!("first element of the array: {} ", xs[0]);
        println!("second element of the array: {}", xs[1]);

        println!("number of elements in array: {}", xs.len());
        println!("array occupies {} bytes", mem::size_of_val(&xs));

        analyze_silce(&xs);
        analyze_silce(&ys[1..4]);

        let empty_array: [u32; 0] = [];
        assert_eq!(&empty_array, &[]);
        assert_eq!(&empty_array, &[][..]);

        for i in 0..xs.len() + 1 {
            match xs.get(i) {
                Some(xval) => println!("{} : {}", i, xval),
                None => println!("Slow down! {} is too far!", i),
            }
        }
    }
}

fn main() {
    primitives::mod_main();
    literals_and_operations::mod_main();
    tuples::mod_main();
    arrays_and_slices::mod_main();
}
