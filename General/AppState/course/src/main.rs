use AppState::Post;

// App state intro
fn main(){
    let mut post = Post::new();
    post.add_text("I ate somefood for Lunch today");
    assert_eq!("",post.content());
}