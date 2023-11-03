#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

// defining a methods for rectangle struct 
impl Rectangle {
     // method without parameters
     fn area(&self) -> u32 {
        self.width * self.heigth
     }

     // method with parameters
     fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth    
     }
     
     // static method (method that does not belong to specific object)
     fn square(size: u32) -> Self {
        Self {
            width: size,
            heigth: size,
        }
     }                
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        heigth: 50,
    };

    println!("The area of the rectangle is {} square pixels",rect1.area());    

    let rect2 = Rectangle {
        width:  10,    
        heigth: 40,    
    };

    let rect3 = Rectangle {
        width:  60,
        heigth: 45,
    };

    println!("Can rect1 hold rect2 {}",rect1.can_hold(&rect2));    
    println!("Can rect1 hold rect3 {}",rect1.can_hold(&rect3));

    // calling the static method of struct Rectangle
    let _sq = Rectangle::square(3);


}




