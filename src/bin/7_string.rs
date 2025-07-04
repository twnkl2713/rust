#![allow(unused)]

fn main(){
    // String -> mutate or data needs to be owned
    let msg = String::from("Hello Rust👋");
    let len = msg.len();
    println!("msg: {msg}");
    println!("len: {len}");
    // &str -> read only (immutable)
    let msg = String::from("Hello rust");
    let s = &msg[0..5];
    let len = s.len();
    println!("slice: {s}");
    println!("len: {len}");
    
    let hello = "Hello Rust";
    
    // Multi line string literal
    let s = r#"
        { "a": 1,
          "b": { "c": 2 },
          "d": 3
        }
    "#;
    println!("{s}");

    // Deref coercian
    let msg = String::from("Hello Twinkle");
    let s = &msg[0..5];

    // Add &str to msg
    let mut msg = "Yo".to_string();
    msg += "!";
    println!("{msg}");

    let lang = "Rust";
    let emoji = "🦀";
    let msg = format!("Hello {lang} {emoji}");
    println!("{}", msg);
}

// we want a 'view' into the original string and not copy it over - string slices