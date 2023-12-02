pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
            Post {
               state: Some(Box::new(Draft {})),
               content: String::new(),
            }
    }

    // method for adding text to a post
    pub fn add_text(&mut self,text: &str) {
            self.content.push_str(text);            
    }

    // returning an empty string slice 
    // until there is a state change from Draft
    pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
    }
    
    pub fn request_review(&mut self) {
            if let Some(s)= self.state.take() {
                self.state = Some(s.request_review())
            }
    }

    pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
    }

}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self,post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

// Draft struct implements 
// State trait 
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})        
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
            self
    }
}

struct PendingReview {}

// PendingReview struct implements 
// State trait 
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

struct Published {}

// Publishe struct implements 
// State trait 
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
       self 
    }

    fn content<'a>(&self,post: &'a Post) -> &'a str {
       &post.content 
    }
}
