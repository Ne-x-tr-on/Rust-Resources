pub struct User{
  pub id:i32,
  pub name:String,
  pub age:u8,
}

impl User{
  pub fn new(id:i32,name:&str,age:u8)-> Self{
    Self {
      id,
      name:name.to_string(),
      age,
    }
  }
}
