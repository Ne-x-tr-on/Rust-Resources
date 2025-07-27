#![allow(warnings)]

fn main(){

//     struct Book{
//     Title:String,
//     Author:String,
//     pages:u32,
//     available:bool,
// }

struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

let mut user1: User = User{
    active:true,
    username:String::from("Newton Kamau"),
    email:String::from("Newton@gmail.com"),
    sign_in_count:1,

};

  println!("{}...{}...{}...{}",user1.username,user1.email,user1.active,user1.sign_in_count);
  user1.username = String::from("David Kamau");
  println!("{}",user1.username);


  fn build_user(email:String,username:String) -> User{
    User{
        active:true,
        email,
        username,sign_in_count:1,
    }
  }


  fn build_user(email:String,username:String) -> User{
    user{
      active:true,
      email,
      username,
      sign_in_count,
    }
  }



// Tuple Strucs
struct Color(i32,i32,i32);
let black = Color(0,0,0,0);













}

