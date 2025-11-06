pub struct Post{
  pub state:Option<Box<dyn State>>,
  pub content:String,
}

impl Post{
  pub fn new()->Post{
    Post { 
      state: Some(Box::new(Draft{})), 
      content: String::new(),
     }
  }
  pub fn add_text(&mut self,text:&str){
    self.content.push_str(text);
  }
}

trait State{
  fn request_review(){};
}

struct Draft{}

impl State for Draft{}

struct PendingReview{}

impl State for PendingReview{}