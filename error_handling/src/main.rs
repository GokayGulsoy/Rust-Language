use std::fs::File;
use std::io::ErrorKind;
use std::io::{self,Read};

fn main() {
    /*
    let arr = vec![1,2,3];

    let any_entry = arr[99];
    println!("99th entry is: {any_entry}");
    */

    /* 
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
                 Ok(file) => file,   
                 Err(error) => panic!("Problem opening the file {:?}",error)   
    };
    */

    // handling exceptions with Result<T,E> enum and match statements
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // handling exception with closures
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap method returns value of Ok if it method
    // executes without error otherwise it will panic 
    // with the error message provided by Err
    let _greeting_file = File::open("hello.txt").unwrap();
    // expect callback can be used to give more informative output
    let _greeting_file = File::open("hello.txt")
                                                       .expect("hello.tx should be included in this project");                
}

// function that returns the error (propagating errors)
fn _read_username_from_file() -> Result<String,io::Error> {
   let username_file_result = File::open("hello.txt"); 

   let mut username_file  = match username_file_result {
           Ok(file) => file,
           Err(e) => return Err(e) 
   };

   let mut username = String::new();
   match username_file.read_to_string(&mut username) {
         Ok(_) => Ok(username),
         Err(e) => Err(e)   
   } 
}

// handling errors with ? operator
fn _read_username_from_file_shorter() -> Result<String,io::Error> {
   let mut username = String::new(); 

   File::open("hello.txt")?.read_to_string(&mut username)?;

   Ok(username) 
}