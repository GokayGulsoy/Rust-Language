use std::ops::Deref;
use std::ops::Drop;
use std::rc::Rc;
use crate::List::{Cons,Nil};

enum List {
    Cons(i32,Rc<List>),
    Nil,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
     fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!",self.data);
     }   
}

// defining our own smart pointer
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

     fn deref(&self) -> &Self::Target {
        &self.0     
     }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn _hello(name: &str) {
   println!("Hello, {name}!");
}

fn main() {
    let b = Box::new(5);
    println!("b = {}",b);

    // let _list = Cons(1,Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5,x);
    assert_eq!(5,*y);

    let z = MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*z); // behind the scenes *z is equal to *(z.deref())

    // if MyBox struct wouldn't have implmented the
    // Deref trait and deref method
    /*
      let m = MyBox::new(String::from("Rust"));
      hello(&(*m)[..]);  
     */

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");    

    let e = CustomSmartPointer {
        data: String::from("some data"),  
    };

    println!("CustomSmartPointer created.");
    drop(e);
    println!("CustomSmartPointer dropped before the end of the main.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
      let _c = Cons(4, Rc::clone(&a));
      println!("count after creating c = {}", Rc::strong_count(&a));  
    }

    println!("count after c goes out of scope = {}",Rc::strong_count(&a));

}
