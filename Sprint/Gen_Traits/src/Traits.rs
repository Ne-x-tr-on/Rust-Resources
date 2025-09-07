pub fn test_traits(){}
struct Person{
  pet:Dog,
}


struct Dog{}

impl Dog{
  fn bark(&self){
    println!("Barking");
  }
}

struct Cat{}

