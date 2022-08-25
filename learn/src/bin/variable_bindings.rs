// 4. Variable Bindings

// Values can be bound to variables, using the let binding.

mod variable_bindings {
    pub fn mod_main() {
        let an_integer = 1u32;
        let a_boolean = true;
        let unit = ();

        let copied_integer = an_integer;

        let _unused_variable = 3u32;

    }
}

// 4.1. Mutability
// Variable bindings are immutabale by default, but this can be overriden using the mut modifer.

mod mutability {
    pub fn mod_main() {
        let immutable_binding = 1;
        let mut mutable_binding = 1;
        mutable_binding += 1;

    }
}

// 4.2. Scope and Shadowing
mod scope_and_shadowing {

    pub fn mod_main() {

    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);

    // Also, variable shadowing is allowed.

    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";
        println!("outside inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding)


    }
}

// 4.3. Declare first
// It's possible to declare variable binding first, and initialize them later.
// However, this form is seldom used, as it may lead to the use of uninitialized variables.
mod declare_first {
    pub fn mod_main() {
        let a_binding;
        {
            let x = 2;
            a_binding = x * x;

        }

    }
}

// 4.4. Freezing
mod freezing {
    pub fn mod_main() {
        let mut _mutable_integer = 7i32;

        {
            // Shadowing by immutable `_mutable_integer`
            let _mutable_integer = _mutable_integer;
    
            // Error! `_mutable_integer` is frozen in this scope
            _mutable_integer = 50;
            // FIXME ^ Comment out this line
    
            // `_mutable_integer` goes out of scope
        }
    
        // Ok! `_mutable_integer` is not frozen in this scope
        _mutable_integer = 3; 
    }
}

fn main() {
    variable_bindings::mod_main();
    mutability::mod_main();
    scope_and_shadowing::mod_main();
    declare_first::mod_main();
    freezing::mod_main();
}
