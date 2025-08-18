// #![allow(warnings)]

// fn main(){

// //     struct Book{
// //     Title:String,
// //     Author:String,
// //     pages:u32,
// //     available:bool,
// // }

// struct User{
//     active:bool,
//     username:String,
//     email:String,
//     sign_in_count:u64,
// }

// let mut user1: User = User{
//     active:true,
//     username:String::from("Newton Kamau"),
//     email:String::from("Newton@gmail.com"),
//     sign_in_count:1,

// };

//   println!("{}...{}...{}...{}",user1.username,user1.email,user1.active,user1.sign_in_count);
//   user1.username = String::from("David Kamau");
//   println!("{}",user1.username);


//   fn build_user(email:String,username:String) -> User{
//     User{
//         active:true,
//         email,
//         username,sign_in_count:1,
//     }
//   }


//   fn build_user(email:String,username:String) -> User{
//     user{
//       active:true,
//       email,
//       username,
//       sign_in_count,
//     }
//   }



// // Tuple Strucs
// struct Color(i32,i32,i32);
// let black = Color(0,0,0,0);



// struct User{
//     username:String,
//     id:i32,
//     email:String,

// }

// fn main(){
//     let mut user1 = User{
//         username:String::from("Newton"),
//         id:23,
//         email:String::from("myemail@gmail.com")
//     };
//     user1.email = String::from("actualemail@gmail.com");
//     println!("{}:{}  ---> {}",user1.username,user1.id,user1.email);


//     fn build_user(_email:String,_username:String) -> User{
//     User { 
//         username: (String::from("Peris")),
//         id: (1234),
//         email: (String::from("peris@gmail.com"))
//      };
             
// }

// }

// }

fn main(){
  let mut account = BankAccount{
    owner:"Newton".to_string(),
    balance : 100.00,
  };

  account.check_balance();
  account.withdraw(100.0);
   account.check_balance();

}


struct BankAccount{
  owner:String,
  balance: f64,
}

impl BankAccount{
  fn withdraw(&mut self,amount:f64){
    println!("Withdrawing {} from acoount owned by {}",amount,self.owner);
    self.balance -= amount;
  }

  fn check_balance(&self){
    println!("Your balance is Kshs {}",self.balance);
  }
}