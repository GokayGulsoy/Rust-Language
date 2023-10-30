use std::io;

fn main() {
    // creating a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of y is: {z}");

    // accessing the elements of tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of x is: {five_hundred}");
    println!("The value of y is: {six_point_four}");
    println!("The value of y is: {one}");

    // empty tuple
    let _empty_tuple = ();

    // creating an array
    let _array = [1, 2, 3, 4, 5];

    // creating an array with type and size
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // creating an array with same elements
    let _a = [3; 5]; // creates array [3,3,3,3,3]
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The value at 0 index is: {first}");
    println!("The value at 0 index is: {second}");

    // array index out of bounds panicking
    println!("Please anter an array index: ");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("File to readline");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
