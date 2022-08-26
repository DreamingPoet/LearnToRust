// 19. Std library types
// The std library provides many custom types which expands drastically on the primitives.
// growable String S like: "hello world"
// growable vector: [1, 2, 3]
// optional types: Option<i32>
// error handling types: Result<i32, ()>
// heap allocated pointers: Box<i32>

// 19.1. Box, stack and heap
mod box_stack_and_heap {
    pub fn mod_main() {}
}
// 19.2. Vectors
mod vectors {
    pub fn mod_main() {}
}

// 19.3. Strings
// There are two types of strings in Rust: String and &str.
// A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence.
// String is heap allocated, growable and not null terminated (not end with \0).

// &str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String,
// just like &[T] is a view into Vec[T].

mod strings {
    use std::mem;
    pub fn mod_main() {
        let char1: char = 'a';
        let str = "ab";
        println!("size of char is {}", mem::size_of::<char>());
        println!("size of &str is {}", mem::size_of::<&str>());

        // char is stored with Unicode scalar value (code points) in memory
        let len = mem::size_of_val(&char1);
        println!("len of char 'a' is {}", len);

        // str is stored with UTF-8 value in memory
        let len = mem::size_of_val(str);
        println!("len of str 'ab' is {}", len);

        // inspect String and &str in memory
        let string = String::from("hello world");
        let string_slice = &string[..3];

        println!("addr of string:String on stack is {:p}", &string);
        println!("addr of data in String on heap is {:p}", string.as_ptr());
        println!("addr of string_slice:&str on stack is {:p}", &string_slice);
        println!("addr of data in &str on heap is {:p}", string_slice);

        // iterate over words
        let pangram = "the quick brown fox jumps over the lazy dog";
        for i in pangram.split_whitespace().rev() {
            println!("> {}", i);
        }

        // copy chars into a vector, sort and remove duplicates
        let mut chars: Vec<char> = pangram.chars().collect();
        chars.sort();
        chars.dedup();

        // create a new string
        let mut new_string = String::new();
        for c in chars {
            new_string.push(c);
            new_string.push(',');
        }
        println!("new_string now is \"{}\"", new_string);

        // the trimmed string is a slice to the original string, hence no new allocation is performed
        let chars_to_trim: &[char] = &[' ', ','];
        let trimmed_string = new_string.trim_matches(chars_to_trim);
        println!("Used chataters: \"{}\"", trimmed_string);

        // heap allocated string
        let alice = String::from("I like dogs");
        let bob = alice.replace("dog", "cat");

        println!("Alice says {}", alice);
        println!("Bob says {}", bob);

        // literials and escapes (escape charater, escape sequence, \r \n ...)

        let byte_escape = "I'm writing \x52\x75\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

        // ...or Unicode code points.
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

        println!(
            "Unicode character {} (U+211D) is called {}",
            unicode_codepoint, character_name
        );

        let wang_in_chinese = "\u{738b}";
        println!("{}", wang_in_chinese);

        // long string
        let long_string = "String literals
        can span multiple lines.
        The linebreak and indentation here ->\
        <- can be escaped too!";
        println!("{}", long_string);

        // raw string
        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("{}", raw_str);
    
        // If you need quotes in a raw string, add a pair of #s
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);
    
        // If you need "# in your string, just use more #s in the delimiter.
        // You can use up to 65535 #s.
        let longer_delimiter = r####"A string with "#" in it. And even "##" also "###"!"####;
        println!("{}", longer_delimiter);

        

        
        
    }
}
// 19.4. Option
mod option {
    pub fn mod_main() {}
}
// 19.5. Result
mod result {
    pub fn mod_main() {}
}
// 19.5.1. ?
mod question_mark_operator {
    pub fn mod_main() {}
}
// 19.6. panic!
mod panic {
    pub fn mod_main() {}
}
// 19.7. HashMap
mod hashmap {
    pub fn mod_main() {}
}
// 19.7.1. Alternate/custom key types
mod alternate_or_custom_key_types {
    pub fn mod_main() {}
}
// 19.7.2. HashSet
mod hashset {
    pub fn mod_main() {}
}
// 19.8. Rc
mod reference_counting {
    pub fn mod_main() {}
}
// 19.9. Arc
mod atomic_reference_counted {
    pub fn mod_main() {}
}

fn main() {
    box_stack_and_heap::mod_main();
    vectors::mod_main();
    strings::mod_main();
    option::mod_main();
    result::mod_main();
    question_mark_operator::mod_main();
    panic::mod_main();
    hashmap::mod_main();
    alternate_or_custom_key_types::mod_main();
    hashset::mod_main();
    reference_counting::mod_main();
    atomic_reference_counted::mod_main();
}
