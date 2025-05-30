#![allow(unused)]

// Modules - help structure code, control visibility and avoid naming conflicts
// here 3 modules - foo, my (has nested modules a and b) (has public func - print, private func - f)

mod foo {
    pub fn print() { // this func is accessible from outside "module foo"
        println!("foo");
    }
}

mod my {
    pub fn print() { // public func
        println!("my");
        println!("Hello from Dunny Fox!");
    }
    // private - can't be called by main (only code inside "my" can call it)
    fn f() {
        println!("private");
        println!("Dunny Fox sneaks here");
    }
    // Nested modules
    pub mod a {
        pub fn print() {
            println!("a");
            println!("Welcome from Dundun Chicken!"); 
        }
        // Public struct
        #[derive(Debug)]
        pub struct S {
            pub name: String, // (accessible)         
            id: u32, // private field
        }
        pub fn build(name: String) -> S { // this public func creates S with a fixed id and given name
            println!("Dundun Chicken is building S!");
            S {name, id: 1}
        }
    }

    // Private module
    // Can't be called outside of this module
    mod b { // private module - cannot be accessed outside "my"
        pub fn print() {  // public func, but since b itself is private, outsiders can't reach it
            println!("b");
            println!("Dunny Fox's secret hideout");
        }
    }

    fn g() { // g is private
        b::print(); // calling b's func innside "my" -> inner code can access private modules 
    }

    // Go one level up in the module tree
    use super::foo;

    fn call_foo_print() { 
        println!("Dunny Fox is calling foo::print"); 
        foo::print(); 
    }
}

use my::a::print as a_print;

fn main() {
    my::print(); // Dunny Fox appears
    my::a::print(); // Dundun Chicken appears
    a_print(); // alias for my::a::print (Dundun Chicken)
    let s = my::a::build("Dunny".to_string()); // build S (Dundun Chicken builds)
    println!("{:?}", s);
    println!("s.name: {}", s.name);
}