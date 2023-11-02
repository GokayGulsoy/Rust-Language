fn main() {
    // double free problem
    let s1 = String::from("hello");
    let _s2 = s1;

    // println!("{}, world",s1); // if we try to access the value
                                 // of s1 Rust compiler gives a 
                                 // warning because we have already
                                 // moved s1 to s2

    // variables and data interchanging with clone
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}i s4 = {}",s3,s4);

        let s = String::from("hello");  // s comes into scope
    
        takes_ownership(s); // s's value moves into the function...
                                        // ... and so is no longer valid here
    
        let x = 5;                      // x comes into scope
    
        makes_copy(x);     // x would move into the function,
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward

        let str = String::from("hello");
        let (str2,length) = calculate_lengths(str);
        
    
        println!("The length of '{}' is {}.",str2,length);

        let str = String::from("hello");
        let len = calculate_lengths2(&str);

        println!("The length of '{}' is {}.",str,len);
} 

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_lengths(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s,length) 
}

fn calculate_lengths2(s: &String) -> usize {
    s.len()
}
