#![allow(unused)]
#[deny(arithmetic_overflow)]
fn main() {
    // Signed Integers
    let i0: i8 = 0; // -(2**8 - 1) to 2**8 - 1 (8-bit signed integer)
    let i1: i16 = 1;
    let i2: i32 = 1;
    let i3: i64 = 1;
    let i4: i128 = 1; // 128-bit signed integer
    let i5: isize = 1; // isize depends on computer architecture (32/64 bit)

    // Unsigned Integers
    let u0: u8 = 1;  // 0 to 2**8 - 1
    let u1: u16 = 1;
    let u2: u32 = 1;
    let u3: u64 = 1;
    let u4: u128 = 1;
    let u5: usize = 1;

    // Floats
    let f0: f32 = 0.21; // 32-bit floating point
    let f1: f64 = 0.01;

    // Boolean
    let b = true;

    // Characters
    let c: char = 'c';
    let e = '😭';

    // Type Conversion
    let i: i32 = 1;
    let u_conv: u32 = i as u32; // casting from signed to unsigned
    let x: u32 = u_conv + (i as u32); // adding signed and unsigned integers

    // Min and max
    let min_i: i32 = i32::MIN; // min val of i32
    let max_i: i32 = i32::MAX; // max val of i32
    let min_u: u32 = u32::MIN; // min val of u32 
    let max_u: u32 = u32::MAX; // max val of u32

    println!("i32 min: {min_i}");
    println!("i32 max: {max_i}");

    let min_char = '\u{0}'; // Smallest Unicode scalar value
    let max_char = char::MAX;

    println!("char min: {min_char}");
    println!("char max: {max_char}");

    // Overflow: This will panic because we have #[deny(arithmetic_overflow)]
    let mut u = u32::MAX;
    // u += 1; // This would panic due to arithmetic overflow

    // Instead, use wrapping_add to demonstrate overflow behavior safely
    let u_wrapped = u.wrapping_add(1);
    println!("Overflow u32 (wrapping_add): {u_wrapped}");

    // checked_add - returns Some(result) or None if overflow occurs
    let u_checked = u32::MAX.checked_add(1); // checked addition that returns an option
    println!("checked_add: {:?}", u_checked); // this will print None if overflow occurs

    // wrapping_add - safely wraps around without panic
    let u_wrapping = u32::MAX.wrapping_add(1);
    println!("wrapping_add: {u_wrapping}"); // this will wrap around to zero if overflow occurs
}
