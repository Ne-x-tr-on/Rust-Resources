#[allow(dead_code)]
#[derive(Debug)]
pub enum Color{
  Green,
  Black,
  Blue,
  Yellow,
  Red,
}

pub struct Person{
  pub first_name:String,
  pub last_name:String,
  pub birth_year:u16,
  pub birth_month:u8,
}

pub fn new_person() -> Person{
  let p1 = Person{
    first_name:"Newton".to_string(),
    last_name:"Kamau".to_string(),
    birth_year: 2006,
    birth_month: 1,
  };
  return p1;
}

pub struct Vehicle{
  pub car_type:String,
  pub color: Color,
}

pub fn firstcar() -> Vehicle{
  let myfirstcar = Vehicle{
    car_type:"RangeRover".to_string(),
    color:Color::Black,
  };
  return myfirstcar;
}