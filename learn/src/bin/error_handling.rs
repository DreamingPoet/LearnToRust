// 18. Error handling
// 18.1. panic
// 18.2. Option & unwrap
// 18.2.1. Unpacking options with ?
// 18.2.2. Combinators: map
// 18.2.3. Combinators: and_then
// 18.3. Result
// 18.3.1. map for Result
// 18.3.2. aliases for Result
// 18.3.3. Early returns
// 18.3.4. Introducing ?
// 18.4. Multiple error types
// 18.4.1. Pulling Results out of Options
// 18.4.2. Defining an error type
// 18.4.3. Boxing errors
// 18.4.4. Other uses of ?
// 18.4.5. Wrapping errors
// 18.5. Iterating over Results

// qstmark = ?

mod error_handling {
    // An explicit `panic` is mainly useful for tests and dealing with
    // unrecoverable errors.
    // When there is a chance that things do go wrong and the caller has to
    // deal with the problem, use `Result`.
    // You can `unwrap` and `expect` them as well(please don't do that unless it's a
    // test or quick prototype)
    pub fn mod_main() {}
}
mod panic {
    fn drink(beverage: &str) {
        // if beverage == "lemonade" { panic!("AAAAaaaaa!!!");}
        println!("Some refeshing {} is all I need.", beverage);
    }
    pub fn mod_main() {
        drink("water");
        drink("lemonade")
    }
}


mod option_unwrap {
    // An enum called option<T> in the std library is used when absence is a possibility.
    // It manifests itself as one of two options:
    // Some(T): An element of type T was found
    // None: No element was found

    // These cases can either be explicitly handled via match or implicitly with unwrap.
    // Implicit handling will either return the inner element or panic.

    // Note that it's possible to manually customize panic with `expect`, but unwrap
    // otherwise leaves us with less meaningfull output than explicit handling.
    // In the following example, explicit handling yieds a more controlled result while retaining
    // the option to panic if desired.

    // The adult has seen it all, and can handle any drink well.
    // All drinks are handled explicitly using `mathc`.
    fn give_adult(drink: Option<&str>) {
        // Specify a course of action for each case.
        match drink {
            Some("lemonade") => println!("Yuck! Too sugary."),
            Some(inner) => println!("{}? How nice.", inner),
            None => println!("No drink? Oh well."),
        }
    }

    // Others will panic when drinking sugary drinks.
    // All drinks are handled implicitly using unwrap.
    fn drink(drink: Option<&str>) {
        // `unwrap` returns a `panic` when it receives a `None`.
        let inside = drink.unwrap();
        if inside == "lemonade" { panic!("AAAaaaa!!!!");}

        println!("I love {}s", inside);
    }

    pub fn mod_main() {
        let water = Some("water");
        let lemonade = Some("lemonade");
        let void = None;

        give_adult(water);
        give_adult(lemonade);
        give_adult(void);

        let coffee = Some("coffee");
        let nothing:Option<&str> = None;

        drink(coffee);
        // will panic
        //drink(nothing);
    }
}
mod unpacking_options_with_qstmark {
    use std::fmt::format;


    // You can unpack `Option`s by match statements, but it's offen easier
    // to use the ? operator.
    // If x is an Option, then evaluating x? will return the underlying value if x is Some(T),
    // otherwise it will terminate whatever function is being executed and retuen None.

    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        // If current_age is None, this returns None.
        // If current_age is Some, the inner u8 gets assigned to next_age.
        let next_age = current_age?;
        Some(format!("Next year I will be {}", next_age))
    }

    // You can chain many `?`s together to make you code much more readable.
    struct Person {
        job: Option<Job>,
    }

    impl Person {
       // Gets the area code of the phone number of person's job, if it exists.
       fn work_phone_area_code(&self) -> Option<u8> {
            self.job?.phone_number?.area_code
       } 
    }

    #[derive(Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }

    #[derive(Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }


    
    pub fn mod_main() {
        println!("{}", next_birthday(Some(3)).unwrap());

        let person = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(123), // None
                    number: 18611,
                }),
            }),
        };


        let person2 = Person {
            job: None,
        };

        // will panic
        // println!("person2's work phone number area code is {}", person2.work_phone_area_code().unwrap());
     
        assert_eq!(person.work_phone_area_code(), Some(123));

    }
}
mod combinators_map {
    pub fn mod_main() {
      // TODO 
    }
}

mod combinators_and_then {
    pub fn mod_main() {
        // TODO 
    }
}

mod result {
    
    pub fn mod_main() {

    }
}

mod map_for_result {
    pub fn mod_main() {

    }
}

mod aliases_for_result {
    pub fn mod_main() {

    }
}

mod early_returns {
    pub fn mod_main() {

    }
}
mod introducing_qstmark {
    pub fn mod_main() {

    }
}
mod multiple_errortypes {
    pub fn mod_main() {
        
    }
}
mod pulling_results_out_of_options {
    pub fn mod_main() {

    }
}
mod defining_an_errortype {
    pub fn mod_main() {

    }
}
mod boxing_errors {
    pub fn mod_main() {

    }
}
mod other_uses_of_qstmark {
    pub fn mod_main() {

    }
}
mod wrapping_errors {
    pub fn mod_main() {

    }
}
mod iterating_over_results {
    pub fn mod_main() {

    }
}


fn main() {
    error_handling::mod_main();
    panic::mod_main();
    option_unwrap::mod_main();
    unpacking_options_with_qstmark::mod_main();
    combinators_map::mod_main();
    combinators_and_then::mod_main();
    result::mod_main();
    map_for_result::mod_main();
    aliases_for_result::mod_main();
    early_returns::mod_main();
    introducing_qstmark::mod_main();
    multiple_errortypes::mod_main();
    pulling_results_out_of_options::mod_main();
    defining_an_errortype::mod_main();
    boxing_errors::mod_main();
    other_uses_of_qstmark::mod_main();
    wrapping_errors::mod_main();
    iterating_over_results::mod_main();
}