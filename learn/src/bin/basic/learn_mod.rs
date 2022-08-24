use std::fmt;

pub fn learn_main() {
    // 1 
    println!("hello world!");
    
    println!("{} days", 31);

    println!("{0}'s name is {0}, his father is {1}!", "Alan", "Bob");
    
    println!("{name}'s name is {name}, his father is {father}!", name = "Alan", father = "Bob");

    println!("Base 10 repr:         {}", 69420);
    println!("Base 2  repr:         {:b}", 69420);
    println!("Base 8  repr:         {:o}", 69420);
    println!("Base 16 repr:         {:x}", 69420);
    println!("Base 16 repr:         {:X}", 69420);

    println!("number = {number:>5}", number = 1);
    println!("number = {number:0>5}", number = 1);
    println!("number = {number:0>width$}", number = 1, width = 3);
    println!("number = {number:*>width$}", number = 1, width = 3);

    let number = 2;
    let width = 5;
    println!("number = {number:0>width$}");


    // Debug 输出使用 {:?} 来进行打印，所有的标准库中的类型，都可以使用 debug 来输出。
    // Debug 主要是面向程序的输出，一般来说，使用 derive 来自动实现 Debug，使用 {:#?} 来进行美化打印。
    // 如果要实现 debug 输出，需要实现 std::fmt::Debug 这个 trait。

    #[derive(Debug)]
    struct MyStruct {
        data:i32,
    }

    let my_struct = MyStruct {
        data:66,
    };

    impl fmt::Display for MyStruct{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MyStruct::data = {}", self.data)
        } 
    }
    println!("print MyStruct {}", my_struct);
    println!("print MyStruct {:?}", my_struct);
    println!("print MyStruct {:#?}", my_struct);
    
    let pi = 3.141592;
    println!("Pi is roughly {pi:.2}");


    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);
    println!("{:?}", Deep(Structure(7)));

    let _default_float = 3.0; // f64
    let _default_ineger = 7; // i32
    let _suffix_integer = 5i32; // i32
    let _suffix_float = 5f64; // f64
    let _nums = 1_000_000; // same as 1000000
    let _nums_deciemal = 0.000_01; // same as 0.00001

    // tuple

    fn reverse(pair: (i32, bool)) -> (bool, i32){
        let (int, boolean) = pair;
        (boolean, int)
    }
    let pair = (6, false);
    println!(" reverse tuple {:?} is {:?}", pair , reverse(pair));

    // Arrays And Slices

    use std::mem;

    fn analyze_silce(slice:&[i32]) {
        println!("first element of this slice : {}", slice[0]);
        println!("the length of this silce: {}", slice.len());
    }

    let xs:[i32; 5] = [1,2,3,4,5];
    let ys:[i32; 500] = [0; 500];

    println!("first element of the array: {} ", xs[0]);
    println!("second element of the array: {}", xs[1]);
    
    println!("number of elements in array: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    
    analyze_silce(&xs);
    analyze_silce(&ys[1 .. 4]);

    let empty_array: [u32;0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0 .. xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{} : {}", i , xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // structure enumeration

    // 1. tuple structs, named tuples
    // 2. classic C structs
    // 3. unit structs, field-less, for generics

    struct Unit;

    struct TestPair(i32, f32);

    struct Point{
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

    let name = String::from("Peter");
    let age = 27;
    let peter = Person{name, age};

    // Enumration
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click{x: i64, y: i64},
    }

    let peter_age = peter.age;
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


    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    fn print_type_of<T>(_: &T) {

        println!("{}", std::any::type_name::<T>())
        
    }

    let a  =  Color::Red;
    print_type_of(&a);

    println!("Color::Red = {}", a as i32);
    
    




    



}