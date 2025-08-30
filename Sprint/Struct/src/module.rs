use std::cell::Cell;
pub fn test(){
  println!("Raptide Song");
}

pub enum IsHappy{
  Yes,
  No,
}

pub struct Person<'p>{
  pub first_name:Cell<&'p str>,
  pub last_name:String,
  pub birth_year:u16,
  pub birth_month:u8,
  pub feeling:IsHappy,
}

#[allow(unused)]
pub fn newperson() -> Person<'static>{
  let person1 = Person{
     first_name:Cell::from("David Kamau"),
     last_name:"Manyeki".to_string(),
     birth_year:1970,
     birth_month:2,
     feeling:IsHappy::Yes,

  };
  // person1.last_name = " - Son: Newton Manyeki Kamau".to_string();
  person1.first_name.set("Newton Kamau");
  println!("{:?}\n{:?}",person1.last_name,person1.first_name.get());
  return person1; 
  
}