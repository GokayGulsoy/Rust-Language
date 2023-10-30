fn main() {
    // variables 
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Value of the constant is: {THREE_HOURS_IN_SECONDS}");
 
    // floating-point numbers
    let y = 2.0; // by default f64 (double precision)
    let z: f32 = 3.0; // f32 (single precision)
 
    print!("y is double-precision float: {y}"); 
    print!("z is single-precision float: {z}");
 
    // basic arithmetic operators
    
    // addition
    let _sum = 5 + 10;
 
    // subtraction
    let _difference = 95.5 - 4.3;
 
    // multiplication
    let _product = 4 * 30;
 
    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // results in -1
 
    // remainder 
    let _remainder = 43 % 5;
 
    // boolean types
    let _t = true;
    let _f: bool = false; // with explicit type annotation
 
    // character type
    let _c =  'z';
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻'; // emoji are also character in Rust
 }