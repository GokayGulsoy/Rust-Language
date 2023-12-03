enum Color {
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
}

enum Message {
    _Quit,
    _Move {x: i32, y: i32},
    _Write(String),
    ChangeColor(Color),
}

enum Message2 {
    Hello {id: i32},
}

struct Point2 {
   x: i32,
   y: i32,
   z: i32, 
}

struct Point {
    x: i32,
    y: i32,
}

// function that takes values as a tuple
fn print_coordinates(&(x,y): &(i32,i32)) {
    println!("Current location ({}, {})",x,y);
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}",y);
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } 
    
    else if is_tuesday {
        println!("Tuesday is green day!");
    } 
    
    else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } 
    
    else {
        println!("Using blue as the background color");
    }

    let mut stack:Vec<u32>  = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
              println!("{}",top);  
    } 

    let v = vec!['a','b','c'];
    for (index,value) in v.iter().enumerate() {
        println!("{} is at index {}",value,index);        
    }

    let (_x,_y,_z) = (1,2,3);
    let point = (3,5);
    print_coordinates(&point);

    let some_option_value: Option<i32> = None;
   
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    let x = 1;

    match x {
          1 => println!("One"),
          2 => println!("two"),
          3 => println!("two"),
          _ => println!("two"),
    }    

    let x = Some(5);
    let y = 10;

    match x {
          Some(50) => println!("Got 50"),
          Some(y) => println!("Matched, y = {y}"),  
          _ => println!("Default case, x = {:?}",x),  
    }

    println!("at the end x = {:?}, y = {y}",x);
    
    let x = 1;

    match x {
        1 | 2=> println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),    
    }

    //matching with a range of values with ..=
    let x = 5;

    match x  {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // destructuring structs
    let p = Point {x: 0, y:7};
    let Point {x: a, y: b} = p;
    assert_eq!(0,a);
    assert_eq!(7,b);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // destructuring enums
    let msg = Message::ChangeColor(Color::Hsv(0,160,255));

    match msg {

        // Message::Quit => {
        //     println!("The quit variant has no data to destructure.");
        // }

        // Message::Move {x,y} => {
        //     println!("Move in the x direction {x} and in the y direction {y}");
        // }    

        // Message::Write(text) => {
        //     println!("Text message: {text}");
        // }            

        Message::ChangeColor(Color::Rgb(r,g,b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }

        Message::ChangeColor(Color::Hsv(h,s ,v)) => {
            println!("Change color  to hue {h}, saturation {s}, value {v}");
        }

        _ => (),
    }

    // first parameter is ignored 
    // due to use of _ pattern    
    foo(3,4);

    // ignoring a parts of a value
    let numbers  = (2,4,8,16,32);
    
    match numbers {
         (first,_,third,_,fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")    
         }   
    }

    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {
    //     println!("found a string");
    // }

    //  println!("{:?}",s); // gives an compile error as variable didn't bind
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point2 {x: 0, y: 0, z: 0};

    match origin {
        Point2 {x, ..} => println!("x is {}",x),
    }

    let numbers = (2,4,8,16,32);

    match numbers  {
        (first, ..,last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // extra conditionals with match guards
    let num = Some(4);

    match num {
          Some(x) if x % 2 == 0 => println!("The number {} is even",x),  
          Some(x) => println!("The number {} is odd",x),  
          None => (),  
    } 

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}",x),
    }      

    println!("at the end : x = {:?}, y = {y}",x);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = Message2::Hello {id: 5};

    match msg {
        Message2::Hello { 
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}",id_variable),

        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }

        Message2::Hello { id } => println!("Found some other id: {}",id),
    }

    

}
