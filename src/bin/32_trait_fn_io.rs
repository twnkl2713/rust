#![allow(unused)]

// Trait input and output

// Trait that defines a common behavior for animals
trait Animal {
    fn speak(&self) -> String;
}

// Two simple structs that will implement the trait
struct Cat;
struct Dog;

// Implement Animal for Cat
impl Animal for Cat {
    fn speak(&self) -> String {
        "meow".to_string()
    }
}

// Implement Animal for Dog
impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".to_string()
    }
}

// Function using static dispatch - compiler knows the exact type
fn greet(animal: &impl Animal) {
    println!("static: {}", animal.speak());
}

// Function using dynamic dispatch - type resolved at runtime
fn greet_dyn(animal: &dyn Animal) {
    println!("dynamic: {}", animal.speak());
}

// Returns a concrete type that implements Animal
fn return_concrete_type() -> impl Animal {
    Dog
}

// Returns a boxed trait object - allows returning different types
fn rand_animal(rand: u32) -> Box<dyn Animal> {
    if rand <= 10 {
        Box::new(Dog)
    }
    else {
        Box::new(Cat)
    }
}
fn main() {
    let cat = Cat;
    let dog = Dog;

    // Static dispatch - known types
    greet(&cat); // static: meow
    greet(&dog); // static: woof

    // Returned concrete type - still statically dispatched
    let animal = return_concrete_type();
    println!("animal.speak: {}", animal.speak()); // woof

    // Dynamic dispatch using trait object
    let animal_str = "dog";
    let animal: &dyn Animal = match animal_str {
        "dog" => &Dog,
        _ => &Cat,
    };

    greet_dyn(animal); // dynamic: woof

    let animal = rand_animal(11);
    println!("rand animal: {}", animal.speak()); // meow
}