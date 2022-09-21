// 9. Functions
mod functions {
    pub fn mod_main() {
        fn fizzbuzz(n: u32) -> u32 {
            n
        }
        // unlike C/C++, there's no restricion on the order of function definitions
        fizzbuzz_to(100);
        fizzbuzz(5);
    }

    // Functions that "don't" return a value, actually return the unit type `()`
    // When a function returns '()', the return type can be omitted from the signature
    fn fizzbuzz_to(n: u32) {
        ()
    }
}
// 9.1. Methods
// Some functions are connected to a particular type. These come in two forms:
// associated functions, and methods.
// Associated functions are functions that are defined on a type generally (like static method in C/C++),
// while methods are associated functions that called on a particular instance of a type (like object methods in C/C++).
mod methods {
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {

        // associated function
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }

        // associated function
        fn origin() -> Point {
            Point::new(0f64, 0f64)
        }
        // method &self is suger for 'self: &Self', Where Self is the type of the caller object.
        fn distance(&self, end: &Point) -> f64 {
            ((end.x - self.x) * (end.x - self.x) + (end.y - self.y) * (end.y - self.y)).sqrt()
        }
    }
    
    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    
    // Pair owns resources: two heap allocated intages
    struct Pair (Box<i32>, Box<i32>);

    impl Pair {
        // This method consumes the resources of the caller object
        // 'self' desugars to 'self: Self'
        fn destroy(self) {
            // destructure 'self'
            // let Pair(first, second ) = self;
            // println!("Destroying Pair ({}, {})", first, second);
            // self out of scope
        }
        
    }

    impl Rectangle {
        
        // method &self is suger for 'self: &Self', Where Self is the type of the caller object.
        fn area(&self) -> f64 {
            // destructuring
            let Point{x: x1, y: y1 } = self.p1;
            let Point{x: x2, y: y2 } = self.p2;
            ((x1 - x2)*(y1 - y2)).abs()
        }

        fn perimiter(&self) -> f64 {

            ((self.p1.x - self.p2.x)*(self.p1.y - self.p2.y)).abs()
        }

        // This method requires the caller objects to be mutable.
        fn translate(&mut self, x: f64, y: f64) {

            self.p1.x += x;
            self.p1.y += y;

            self.p2.x += x;
            self.p2.y += y;

        }

    }

    pub fn mod_main() {
        let origin = Point::origin();
        let point_a = Point::new(3.0, 4.0);
        println!("the distance is {}", origin.distance(&point_a));

        let rectangle = Rectangle {
            p1: origin,
            p2: point_a,
        };


        // Note that the first argument '&self' is implicitly passed, i.e. 
        // 'rectangle.perimiter()' === 'Rectangle::perimiter(&rectengle)'
        println!("Rectangle perimiter: {}", rectangle.perimiter());
        println!("Rectangle perimiter: {}", Rectangle::perimiter(&rectangle));
        println!("Rectangle area: {}", rectangle.area());


        let mut square = Rectangle {
            p1: Point::origin(),
            p2: Point::new(2.0, 2.0),
        };

        square.translate(1.0, 1.0);

        let pair = Pair(Box::new(1), Box::new(2));
        pair.destroy();
        // pair.destroy();

    }
}
// 9.2. Closures
// Closures are functions that can capture the enclosing enviroment.
// The function and the variables it captured makes a closure. 

// The syntax and capabilities of closures make them very convenient for on the fly usage.
// Calling a closure is exactly like calling a function.
// However, both input and return types can be inferred and input variable names must be specified.

// using || instead of () around input variables.
// optional body delimitation ( {} ) for a single expression (mandatory otherwise).
// The ability to capture the outer variables
mod closures {
    pub fn mod_main() {
		// Increment via closure and function
        fn fn_add_one(i: i32) -> i32{ i + 1 }

        let closure_annotated_add_one = |i: i32| -> i32 { i + 1};
        let closure_inferred_add_one = |i|i+1;
        
        let i = 1;
        println!("fn_add_one: {}", fn_add_one(i));
        println!("closure_annotated_add_one: {}", closure_annotated_add_one(i));
        println!("closure_inferred_add_one: {}", closure_inferred_add_one(i));

        // A closure taking on argument
        // return type is inferred
        let one = ||1;
        println!("closure returning one {}", one() );
        
    }
}
// 9.2.1. Capturing
// closures can capture variables:
// by reference: &T (borrow)
// by mutable reference: &mut T
// by value: T
mod capturing {
    use std::ops::RangeBounds;

    pub fn mod_main() {
        use std::mem;

        let color = String::from("green");

        // A closure to print 'color' which immediately borrows color (by reference) and
        // stores the borrow and closure in the 'print' variable.
        // It will remain borrowed until 'print' is used the last time.
		//
        // 'println!' only requires arguments by immutable reference so it dosen't
        // impose anything more restrictive.
        let print = ||println!("'color': {}", color);

        // call the closure using the borrow
        let borrow_color = &color;
        print();
        
        
        let borrow_color = &color;
        // move not allowed
        // let move_color = color;
        print();
        
        // a move is allowed after the final use of 'print'
        let move_color = color;

        let mut count = 0;
        // a closure to increment 'count' could take a '&mut count' or 'count'
        // but '&mut count' is less restrictive so it takes that.
        // Immediatelly borrows 'count'.n 
		//
        // A 'mut' is required on 'inc' because a '&mut count' is stored inside.
        // Thus, calling the closure mutates closure.
        let mut inc = || { 
            count += 1;
            println!("'count': {}", count);
         };

         // what is the disference between 'let a = &mut b;' and 'let mut a = &mut b;'
         // let mut a = &mut b; a cannot be assigned twice.
         let mut_inference = &mut count;

         let var1 = 3;
         let var_reference = &var1;

         let mut var2 = 3;
         let mut mut_var = &mut var2;
         *mut_var += 1;
         let mut var3 = 3;
    
         mut_var = &mut var3;
         *mut_var +=1;

         // let mut mut_var = &mut var;
         // *mut_var +=1;


         // A non-copy type.
         let movable = Box::new(3);

         // 'mem::drop(T)' requires 'T' so this must take by value.
         // A copy type would copy into the clousure leaving the original untouched.
         // A non-copy type must move and so 'movable' immediately moves into the closure.
         let consume = || {
            println!("movable : {:?}", movable);
            mem::drop(movable);

         };

         // `consume` sonsumes the variable so this can only be called once.
         consume();


         // Using move before vertical pipes forces closure to take ownership of captured variables:
         // `Vec` has non-copy semantics.
         let haystack = vec![1, 2, 3];
         let contains = move|needle| haystack.contains(needle);

         let a = &1;
         let b = &4;
         println!("haystack contains {} is {}", a, contains(a));
         println!("haystack contains {} is {}", b, contains(b));
         
         // error, haystack is moved cannot be used anymore
         // let c = haystack;
         // println!("There're {} elements in vec", haystack.len());

         // removing `move` from closure's signature will cause closure
         // to borrow haystack varibale immutably, hence `haystack` will still available.

        
    }
}
// 9.2.2. As input parameters
// When takiing a closure as an input parameter, the closure's complete type must be
// detemined by what the closure does with captured value.
// In order of decreasing restriction, they are:

// Fn : the closure uses the captured value by reference (&T)
// FnMut : the closure uses the captured value by mutable reference (&mut T)
// FnOnce : the closeure uses the captured value by value (T)


mod as_input_parameters {
    // On a varisable-by-variablr basis, the complile will capture variables
    // in the least restrictive manner possible.
    
    // For instance, consider a parameter annotated as FnOnce.
    // This specifies that the closure may capture by &T, &mut T or T, but the compile will
    // ultimately choose based on how the captured cariables are used in the closure.
    
    // This is because if a move is possible, then any type of borrow should also be possible.
    // Note that the reverse is not true.
    // If the parameter is annotated as Fn, then capturing variables by &mut T or T
    // are not allowed. 
    
    // A function which takes a closure as an argument and calls it.
    fn apply<F>(f: F) where
        // The closure takes no input and returns nothing.
        F: FnOnce() {
    
            f();
    
    }
    
    
    // A funtion which takes a closure and returns an `i32`.
    fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
        f(3)
    }
    pub fn mod_main() {
        use std::mem;

        let greeting = "hello";

        // A non-copy type.
        // `to_owned` creates owned date from borrowed one
        let mut farewell = "goodby".to_owned();


        // Capture 2 variables: `greeting` by reference and `farewell` by value.
        let diary = || {
            // `greeting` is by refernce: requires `Fn`.
            println!("I said {}", greeting);

            // Mutation force `farewell` to be captured by mutable reference(&mut T).
            // Now requires `FnMut`.
            farewell.push_str("!!!");
            println!("Then I streamed {}", farewell);

            // Manually calling drop forces `farewell` to be captured by value.
            // Now requires `FnOnce`.
            mem::drop(farewell);
        };

        // Call the function which applies the closure.
        apply(diary);

        // closure `double` satisfies `apply_to _3`'s trait bound
        let double = |x| 2 * x;
        println!("3 doubled: {}", apply_to_3(double));


    }
}
// 9.2.3. Type anonymity
mod type_anonymity {
    // Using a closure as a function paramter requires generics.
    // `F` must be generics.
    fn apply<F>(f: F) where
    F: FnOnce() {
        f();
    }

    // When a closure is defined, the compile implicitly creates a new anonymous structure
    // to store the captured variables inside, meanwhile implementing the functionality
    // via one of the traits: Fn, FnMut, or FnOnce for this unknown type.
    
    // `F` must implement `Fn` for a closure which takes no inputs and return nonthing
    // - exactly what is required for `print`.
    fn apply_closure<F>(f: F) where
    F: Fn() {
        f();
    }

    pub fn mod_main() {
        let x = 7 ;

        // Capture `x` into an anonymous type and implement
        // `Fn` trait for it. Store it in `print`.
        let print = || println!("{}", x);

        apply_closure(print);
    }

}
// 9.2.4. Input functions

// Since closures may be used as aguments, functions will also do.

mod input_functions {
    // Define a function which takes a generic `F` argument
    // bounded by `Fn`, and calls it.
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    // Define a function satisfying the `Fn` bound
    fn function() {
        println!("I'm a function!");
    }

    pub fn mod_main() {
        // Define a closure satisfying the `Fn` bound
        let closure = || println!("I'm a closure!");

        call_me(function);
        call_me(closure);
    }
}

// 9.2.5. As output parameters

mod as_output_parameters {
    pub fn mod_main() {}
}
// 9.2.6. Examples in std
mod examples_in_std {
    pub fn mod_main() {}
}
// 9.2.6.1. Iterator::any
mod iterator_any {
    pub fn mod_main() {}
}
// 9.2.6.2. Searching through iterators
mod searching_through_iterators {
    pub fn mod_main() {}
}
// 9.3. Higher Order Functions
mod higher_order_functions {
    pub fn mod_main() {}
}
// 9.4. Diverging functions
mod diverging_functions {
    pub fn mod_main() {}
}

fn main() {
    functions::mod_main();
    methods::mod_main();
    closures::mod_main();
    capturing::mod_main();
    as_input_parameters::mod_main();
    type_anonymity::mod_main();
    input_functions::mod_main();
    as_output_parameters::mod_main();
    examples_in_std::mod_main();
    iterator_any::mod_main();
    searching_through_iterators::mod_main();
    higher_order_functions::mod_main();
    diverging_functions::mod_main();
}
