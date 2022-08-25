//6. Conversion
// Primitive types can be converted to each other through casting.
// Rust addresses conversion between custom types by use of traits.
// The generic conversions will use the From and Into traits.
mod conversion {
    pub fn mod_main() {}
}
//6.1. From and Into
mod from_and_into {
    // The From trait allows for a type to define how to create itself from another type.
    use::std::convert::From;
    
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number {value:item}
        }

    }

    pub fn mod_main() {
        let num = Number::from(30);
        println!("My number is {:?}", num);

        // Using the Into trait will typically require specificaion of the type to convert into as the compiler is unable to detemine this most of the time.
        let int = 5;
        let num: Number = int.into();
        println!("My number is {:?}", num);

    }
}
//6.2. TryFrom and TryInto
// TryFrom / TryInto traits are used for fallible conversions, and as such, return Result S.

mod tryfrom_and_tryinto {
    
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);
    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    pub fn mod_main() {
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        // try_into
        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));

    }
}
//6.3. To and from Strings
// Rather than implement the ToString trait directly, you should implement the fmt::Display trait which automagically
// provides ToString and also allow printing the type.
mod to_and_from_strings {
    use std::fmt;

    struct Circle {
        radius: i32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }
    pub fn mod_main() {
        let circle = Circle { radius: 6};
        println!("{}", circle.to_string());
    }
}

fn main() {
    conversion::mod_main();
    from_and_into::mod_main();
    tryfrom_and_tryinto::mod_main();
    to_and_from_strings::mod_main();
}
