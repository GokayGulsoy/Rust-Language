struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// struct with references can't
// be initialized without lifetime 
// traits
struct StructWithReferences {
    active: bool,
    // username: &str,      lifetimespecifier is required for references types in structs    
    // email: &str,         lifetimespecifier is required for references types in structs
    sign_in_count: u64,
}
// Rust will give an compiler error
// for the above struct

// using tuple structs without name fields
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

// unit-like struct 
struct AlwaysEqual;

fn main() {
  // defining a mutable struct
  let mut user1 = User {
      active: true,  
      username: String::from("someusername123"),
      email: String::from("someone@example.com"),  
      sign_in_count: 1
  };

  let mut _user2 = User {
    active: true,  
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),  
    sign_in_count: 1
};


  // accessing the fields of struct with "." (dot) notation
  user1.email = String::from("anotheremail@example.com");

  // creating instances from other instances with
  // struct update syntax
  let user3 = User {
      active: user1.active,  
      username: user1.username,
      email: String::from("another@example.com"), 
      sign_in_count: user1.sign_in_count
  };

  let _user2 = User {
      email: String::from("another@example.com"),
      ..user3
  };
  
  // using tuple structs
  let _black = Color(0,0,0);
  let _origin = Point(0,0,0);

  // unit-like structs without any fields    
  let _subject = AlwaysEqual;



}


// function that returns sturct type
fn _build_user(email: String,username: String) -> User {
    User {
        active: true,
        username: username, // remove username for short hand init.
        email: email,       // remove email for short hand init.
        sign_in_count: 1
    }
}


