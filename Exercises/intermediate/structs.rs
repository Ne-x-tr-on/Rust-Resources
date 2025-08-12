// ===========================
// RUST STRUCTS PRACTICE QUESTIONS
// ===========================
//     // Q1. Define a struct named `Person` with fields `name` (String) and `age` (u32).
// struct Person{
//     name:String,
//     age:u32,
// }

// // Q3. Write a method for the `Person` struct called `greet` that prints "Hello, my name is NAME".
// impl Person{
//     fn greet(&self){
//         println!("Hello my name is {} and i was {}",self.name,self.age);
//     }
// }

// fn main() {
// // Q2. Create an instance of `Person` with the name "Alice" and age 30.
//     let _instance1 = Person{
//     name:String::from("David"),
//     age:54,    
// };
// let _ = &_instance1.greet();
  
// }



// Q4. Define a struct `Rectangle` with `width` and `height` as u32. Add a method `area` that returns the area.
// struct Rectangle{
//     width:i32,
//     height:i32,
// }

// impl Rectangle{
//     fn area(&self)->i32{
//         self.width * self.height
//     }
//     // println!("Area: {}",self.area);
// }

// fn main(){
//     let area = Rectangle{
//         width:20,
//         height:30,
//     };
//     let _val = area.area();
//      println!("Area: {}",_val);
// }

// // Q5. Implement a tuple struct `Color` that holds 3 u8 values for RGB.
// struct Color(u8,u8,u8);

// // Q6. Define a unit-like struct called `Marker`.
// struct Marker;

// Q7. Create a struct `Student` with private field `grade` and public field `id`. Add a constructor and method to access `grade`.
// pub struct Student{
//     pub id:u32,
//     grade:char,
// }

// impl Student{
//     pub fn constr(id:u32,grade:char) -> Self{
//         Student{grade,id}
//     }
//     pub fn meth(&self) -> char{
//         self.grade
//     }
// }

// Q8. Write a struct `Circle` with radius (f64), and an associated function `new` that returns a Circle instance.

pub struct Circle{
    radius:f64,
}

impl Circle{
    pub fn new(radius:f64) -> Self{
        Circle{radius}
    }
    // Q9. Add a method to `Circle` called `circumference` that returns the circumference

  pub fn circumference(&self) -> f64{
        2.0*std::f64::consts::PI*self.radius
    }
    
}

fn main(){
    
}



// Q10. Create a struct `Login` with fields `username`, `email` (String), and a method `display` that prints them.

// Q11. Create a struct `Book` with fields `title` and `pages`. Derive `Debug` and print an instance.

// Q12. Define two structs `Point` and `Line` where Line contains two `Point` fields: `start` and `end`.

// Q13. Use the struct update syntax to create a new `Person` from another, changing only the name.

// Q14. Implement a struct `Car` with a method `drive` that takes `self` by mutable reference and increments a mileage field.

// Q15. Define an enum `Role` and use it inside a struct `User` as a field. Create an instance of `User` with a Role.


