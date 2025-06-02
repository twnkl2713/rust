#![allow(unused)]

// Simulates a function returning Result<u32, String>
fn f1() -> Result<u32, String> {
    println!("f1 called");
    Ok(1) // Successful result
}

// Another func returning Result<u32, String>
fn f2() -> Result<u32, String> {
    println!("f2 called");
    Ok(2) // Successful result
}

// Example handling f1 and f2 using match to handle Result
fn f1_f2_match() -> Result<u32, String> {
    // call f1
    let res1 = f1();
    let out1 = match res1 {
        Ok(val) => val,
        Err(_) => return Err("Error from f1".to_string()),
    };
    
    // call f2
    let res2 = f2();
    let out2 = match res2 {
        Ok(val) => val,
        Err(_) => return Err("Error from f2".to_string()),
    };

    // return the sum if both succeed
    Ok(out1 + out2)
}

// Example using ? operator to simplify error handling
fn f1_f2_question() -> Result<u32, String> {
    let out1 = f1()?; // if Err, return Err immediately
    let out2 = f2()?; // if Err, return Err immediately
    Ok(out1 + out2)
}

fn main() {
    // using the match-based approach
    let result_match = f1_f2_match();
    println!("Result from f1_f2_match: {:?}", result_match);

    // using the ?-operator approach
    let result_question = f1_f2_question();
    println!("Result from f1_f2_question: {:?}", result_question);
}