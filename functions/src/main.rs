fn main() {
    println!("Hello, world!");

    another_function(5); // function with single parameter
    print_labeled_measurement(5, 'h'); // function with multiple parameters
    let _y = 6; // statement (statements don't return any value)
    
    let y = {
        let x = 3;
        x + 1 // curly braces can also be used to define 
              // expressions (be aware that we don't end the
              //   expressions with ;)
    };

    println!("The value of y is: {y}");
	let x = five();

	println!("The value of x is: {x}");
	let x = plus_one(5);
	println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("Another function and value of x is: {x}");
}

fn print_labeled_measurement(value: i32,unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// function with a return value
// syntax is: fn func_name() -> return_type {}
fn five() -> i32 {
    5
}

// function with a return value and parameter
fn plus_one(x: i32) -> i32 {
	x + 1
}

