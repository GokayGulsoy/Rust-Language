fn main() {
    
    // string slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello part of the string is: {}",hello);
    println!("world part of the string is: {}",world);

    // these two slices are equal
    let slice = &s[0..2];
    let slice2 = &s[..2];

    println!("slice is: {}",slice);
    println!("slice2 is: {}",slice2);

    let len = s.len();
    // these two slices are equal
    let slice = &s[3..len];
    let slice1 = &s[3..];


    println!("slice is: {}",slice);
    println!("slice1 is: {}",slice1);

    let slice = &s[0..len];
    let slice1 = &s[..];

    println!("slice is: {}",slice);
    println!("slice1 is: {}",slice1);

    // array slices
    let a = [1,2,3,4,5];
    let slice = &a[1..3];

    assert_eq!(slice,&[2,3]);
}
