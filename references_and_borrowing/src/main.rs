fn main() {
    let mut s1 = String::from("hello");

    // if the function parameters are references
    // don't have to return values from functions
    // because we never had ownership.
    let len = calculate_length(&mut s1);
    println!("The length of '{}' is {}.",s1,len);

    // change(&s); borrowed values can't be modified
}

fn calculate_length(s: &mut String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But it does not have ownership of what
  // it refers to, it is not dropped.
  
/* this function returns a reference to 
   a deallocated memory    
fn dangle() -> &String {
   let s: String = String::from("hello");   
   &s
}
*/

/*  references can't be modified unless they are mutable  
fn change(some_strings: &mut String) {
    some_strings.push_str(",world");
}
*/


