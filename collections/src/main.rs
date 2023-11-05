enum SpreadSheetCell {
     Int(i32),
     Float(f64),   
     Text(String)
}

use::std::collections::HashMap;
fn main() {
   // vector collection 
   let _v: Vec<i32> = Vec::new(); 
   // using vec!
   let _v = vec![1,2,3];
   let mut v = Vec::new();

   // Updating vector
   v.push(5);
   v.push(6);
   v.push(7);
   v.push(8);

   let third: &i32 = &v[2];
   println!("The third element is {third}"); 

   let third: Option<&i32> = v.get(2);
   match third  {
         Some(third) => println!("The third element is {third}"),
         None => println!("There is no third element")
   }

   /* 
   let mut v = vec![1,2,3,4,5];
   let first = &v[0]; immutable borrow

   v.push(6);  mutable borrow occurs here
   println!("The first element is: {first}")
   */

   let v = vec![100,32,57];
   for i in &v {
       println!("{i}"); 

   }

   let mut v = vec![100,32,57];
   for i in &mut v {
       *i += 50;
   }

   let _row = vec![
          SpreadSheetCell::Int(3),
          SpreadSheetCell::Text(String::from("blue")),  
          SpreadSheetCell::Float(10.12)  
   ];

   {

    let _v = vec![1,2,3,4];
    // do stuff with v
    // <- goes out of scope and is freed here
   } 

   // String data type
   let mut _s = String::new();

   let data = "initial contents";
   let _s = data.to_string();

   let _s = "initial contents".to_string();

   // Appending to a String with push_str and push 
   let mut s = String::from("foo");
   s.push_str("bar");

   println!("s is {s}");
   // appending a character to a string
   let s1 = String::from("Hello,");
   let s2 = String::from("world!");
   let _s3 = s1 + &s2;

   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");

   let s = s1 + "-" + &s2 + "-" + &s3;
   println!("s is {s}");

   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");

   let s = format!("{s1}-{s2}-{s3}");
   println!("s is {s}");

    /*
        code generates an error different
        from other languages Rust does not
        allow access to individual characters
        let s1 = String::from("hello");
        let h = s1[0]; 
    */
    
    // Hash Maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    // Accessing values in Hash Map
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    // iterating over a Hash Map
    for (key,value) in &scores {
            println!("{key}: {value}");
    }    

    // Hash Maps and Ownership
    // For types that implement "Copy" trait like i32
    // values are copied into the hash map. For owned 
    // values like String, values will be moved an the 
    // hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name,&field_value);
    // field_name and field_value are invalid at this point
    // try using them and see what compiler error you get
    /*println!("{field_name}: {field_value}");*/    

    // Overwriting a value
    let mut scores: HashMap<String,i32> = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Blue"),25);


    // latest value for the same key added will be used
    println!("{:?}",scores);

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"),10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}",scores);
    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;   
    }

    println!("{:?}",map);
}
