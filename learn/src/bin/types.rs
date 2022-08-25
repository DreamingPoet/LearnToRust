//5. Types
// Rust provides several mechanisms to change or define the type of primitive and user defined types.
mod types {
    pub fn mod_main() {

    }
}

//5.1. Casting
// Rust provides no implicit type conversion(coercion) between primitive types.
// But, explicit type conversion(casting) can be performed using the as keyword.

mod casting {
    pub fn mod_main() {
        let decimal = 65.4321_f64;
        // Error! No implicit conversion
        // let integer: u8 = decimal;

        let integer = decimal as u8;

        // Explicit conversion
        let charater = integer as char;

        // A float cannot be directly converted to a char
        // let charater = decimal as char;
        
    }
}


//5.2. Literals
mod literals {
    pub fn mod_main() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` u8 in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` u32  in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` f32  in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` i32 in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` f64 in bytes: {}", std::mem::size_of_val(&f));

    }
}
//5.3. Inference
mod inference {
    pub fn mod_main() {
        let elem = 5u8;

        let mut vec = Vec::new();

        vec.push(elem);
        println!("{:?}", vec);

    }
}
//5.4. Aliasing
// The type statement can be used to give a new name to an existing type.
mod aliasing {
    type UnsignedIntegerLong = u64;
    pub fn mod_main() {

        let a: UnsignedIntegerLong = 9867987097097;
        println!("UnsignedIntegerLong is equal to u64 {}", a);
    }
}

fn main() {
    types::mod_main();
    casting::mod_main();
    literals::mod_main();
    inference::mod_main();
    aliasing::mod_main();
}
