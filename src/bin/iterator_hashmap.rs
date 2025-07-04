use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Twinkle", 50);
    scores.insert("Saumya", 40);
    scores.insert("Soham", 80);

    // Example 1:
    println!("Iterating over key-value pairs: ");
    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }

    // Example 2:
    println!("\nIterating over mutable key-value pairs: ");
    for (key, value) in scores.iter_mut() {
        *value = *value + 10;
        println!("{}: {}", key, value);
    }
}