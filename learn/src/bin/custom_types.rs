
// 3. Custom Types
//
mod custom_types {
    pub fn mod_main() {
        // struct: define a stucture
        // enum: define an enumeration
        // Constants can be created via the const and static keywords.
    }
}
// 3.1. Structures

// a. tuple structs, named tuples
// b. classic C structs
// c. unit structs, field-less, for generics

mod structures {
    struct Unit;

    struct TestPair(i32, f32);

    struct Point {
        x: f32,
        y: f32,
    }

    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    struct Person {
        name: String,
        age: u8,
    }

    pub fn mod_main() {
        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };
        let peter_age = peter.age;
    }
}

// 3.2. Enums
//
mod enums {
    // Enumration
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    type WebEvt = WebEvent;


    fn inspect(event: WebEvent) {
        match event {
            WebEvt::PageLoad => todo!(),
            WebEvt::PageUnload => todo!(),
            WebEvt::KeyPress(_) => todo!(),
            WebEvt::Paste(_) => todo!(),
            WebEvt::Click { x, y } => todo!(),
        }
    }

    
    pub fn mod_main() {
        
        
        
    }
}


// 3.2.1. use
//
mod use_ {
    
    enum Status {
        Rich,
        Poor,
    }
    enum Work {
        Civilian,
        Soldier,
    }
    
    enum Gender {
        Male,
        Female,
    }
    

    pub fn mod_main() {
        
        use Status::{Poor, Rich};
        use crate::use_::Work::{Civilian, Soldier};
        use self::Gender::*;
        
        let status = Poor;
        let work = Soldier;
        let gender = Male;
        
        match gender {
            Male => println!("gender is male"),
            Female => println!("gender is female"),
        }
        
    }
}
// 3.2.2. C-like
//
mod c_like {
    enum Number {
        Zero,
        One,
        Two,
    }
    
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    
    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }


    pub fn mod_main() {
        
        
        let a = Color::Red;
        print_type_of(&a);
        
        println!("Color::Red = {}", a as i32);
    }
}


// 3.2.3. Testcase: linked-list
//
mod linked_list {

    use self::List::*;
    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            Nil
        }

        fn prepend(self, elem:u32) -> List {
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0
            }
        }

        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                },
                Nil => {
                    format!("Nil")
                },
            }

        }
    }

    pub fn mod_main() {
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);
        
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());

    }
}

// 3.2.4. Testcase: linked-list struct
//
mod linked_list_struct {

    // use self::List::*;
    struct List {
        elem: u32,
        next: Option<Box<List>>,
    }

    impl List {
        fn new() -> List {
            let elem = 0;
            let next = None;
            List{elem, next}
        }

        fn prepend(self, elem:u32) -> List {
            let next = Some(Box::new(self));
            List{elem, next}
        }

        fn len(&self) -> u32 {
            match self.next {
                Some(ref tail) => 1 + tail.len(),
                None => 0
            }
        }

        fn stringify(&self) -> String {
            match self.next {
                Some(ref tail) => {
                    format!("{}, {}", self.elem, tail.stringify())
                },
                None => {
                    format!("None")
                },
            }

        }
    }

    pub fn mod_main() {
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);


        println!("structure linked list has length: {}", list.len());
        println!("{}", list.stringify());

    }
}





// 3.3. constants
//
mod constants {
    pub fn mod_main() {}
}

fn main() {
    custom_types::mod_main();
    structures::mod_main();
    enums::mod_main();
    use_::mod_main();
    c_like::mod_main();
    linked_list::mod_main();
    linked_list_struct::mod_main();
    constants::mod_main();
}
