
// 10. Modules
// Rust provides a powerful module system that can be used to hierarchically split
// code in logical units(modules), and manage visiability(public/private) between them.


// 10.1. Visibility
mod visibility {
    pub fn mod_main() {
        
    }
}
// 10.2. Struct visibility
mod struct_visibility {
    pub fn mod_main() {
        
    }
}
// 10.3. The use declaration
mod the_use_declaration {
    pub fn mod_main() {
        
    }
}
// 10.4. super and self
mod super_and_self {
    pub fn mod_main() {
        
    }
}
// 10.5. File hierarchy
mod file_hierarchy {
    pub fn mod_main() {
        
    }
}


fn main() {

    visibility::mod_main();
    struct_visibility::mod_main();
    the_use_declaration::mod_main();
    super_and_self::mod_main();
    file_hierarchy::mod_main();

}