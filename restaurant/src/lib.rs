mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad
    }

    pub struct Breakfest {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfest {
         pub fn summer(toast: &str) -> Breakfest {
                Breakfest { 
                    toast: String::from(toast),     
                    seasonal_fruit: String::from("peaches") 
                }
         } 
    }
}

// our libraries public API
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
       // Absolute path 
       crate::front_of_house::hosting::add_to_waitlist(); 
       // Relative path
       front_of_house::hosting::add_to_waitlist(); 

       let mut meal = back_of_house::Breakfest::summer("Rye"); 
       meal.toast = String::from("Wheat"); 
       println!("I'd like  {} toast please",meal.toast); 

       // The next line won't compile if we uncomment it; we're not allowed
       // to see or modify the seasonal fruit that comes with meal
       // meal.seasonal_fruit = String::from("blueberries"); 

       let _order1 = back_of_house::Appetizer::Soup;
       let _order2 = back_of_house::Appetizer::Salad; 

        // directly accessing to add_to_waitlist function inside hosting mod
        hosting::add_to_waitlist();
}