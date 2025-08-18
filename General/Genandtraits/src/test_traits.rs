#[allow(dead_code)]
// struct Person<PetType:Animal + NotDangerous>{
//   first_name:String,
//   pet:PetType,
// }
struct Person <PetType> where PetType:Animal+NotDangerous{
  first_name:String,
  pet:PetType,
}


#[allow(dead_code)]
trait Animal{
  fn make_sound(&self) -> ();
}

trait NotDangerous {}

#[allow(dead_code)]
struct Dog {}
impl NotDangerous for Dog{}
impl Animal for Dog{
  fn make_sound(&self) -> () {
      println!("Dog Barked");
  }
}

#[allow(dead_code)]
struct Cat{}
impl NotDangerous for Cat{}
impl Animal for Cat {
  fn make_sound(&self) -> () {
      println!("Cat Meowed");
  }
    
}

pub fn create_person(){
  let _pet1 = Dog{};
  let  _p1 = Person{ 
                                first_name: "Newton".to_string(),
                                pet:_pet1 };
                                _p1.pet.make_sound();
  let _pet2 = Cat{};
  let _p2 = Person{
                              first_name:"Vivian".to_string(),
                              pet:_pet2};
                              _p2.pet.make_sound();
}