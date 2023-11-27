pub struct Guess {
    value: i32,
} 

impl Guess {
     pub fn new(value: i32) -> Guess {
            if value < 1 {
                       panic!("Guess value must be greater than or equal to 1, got {}",value); 
            }

            else if value > 100 {
                    panic!("Guess value must be less than or equal to 100,got {}.",value)
            }    

            Guess { value }    
     }   
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}


pub fn prints_and_returns_10(a: i32) -> i32 {
       println!("I got the value {}",a); 
       10 
}

struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
     fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
     }
}

pub fn greeting(name: &str) -> String{
       String::from("Hello!") 
}

pub fn add_two_pub(a: i32) -> i32 {
       internal_adder(a,2) 
}

fn internal_adder(a: i32,b: i32) -> i32 {
   a + b 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2,4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        assert_ne!(4,add(2,3));
    }

    #[test]
    fn greeting_contains_name() {
       let result = greeting("Carol");
       assert!(result.contains("Carol"),"
       Greeting did not contain name, value was {}",result);     
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
       Guess::new(200);     
    }

    // we can also use result enum for testing purposes
    #[test]
    fn it_works2() -> Result<(), String> {
                    if 2 + 2 == 4 {
                        Ok(())
                    } 

                    else {
                        Err(String::from("two plus two does not equal four"))
                    }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10,value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5,value)
    }

    #[test]        
    fn add_two_and_two() {
       assert_eq!(4,add_two(2)); 
    }


    #[test]
    fn add_three_and_two()  {
       assert_eq!(5,add_two(3));     
    }

    #[test]
    fn one_hundred() {
       assert_eq!(102,add_two(100)); 
    }

    #[test]
    #[ignore]
    fn expensive_test() {
       // code that takes an hour to run 
    }


    // private functions can be tested
    // but it is not usually accepted as
    // good convention
    #[test] 
    fn internal() {
        assert_eq!(4,internal_adder(2,2));
    }

    



}
