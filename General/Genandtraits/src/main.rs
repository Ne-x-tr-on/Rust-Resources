#[allow(dead_code)]


struct character {
    hit_points: u16,
}


struct Person <PetType:Animal>{
    first_name:String,
    pet:PetType,
}

trait Animal{}

struct Cat {}
struct Dog{}

impl Dog{
    fn bark(&self){
        println!("Bark");
    }
}

impl Animal for Dog {}



pub fn create_person(){
    let pet1 = Dog{};
    let p1 = Person{
        first_name:"Newton".to_string(),
        pet:pet1,
    };
}

fn main(){
    create_person();
}