trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

// Function accepts ANY Animal
fn make_it_speak(animal: &dyn Animal) {
    animal.speak();  // Rust checks at runtime which one to call
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    make_it_speak(&dog);  // Woof!
    make_it_speak(&cat);  // Meow!
}




