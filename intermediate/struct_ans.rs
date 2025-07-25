// ===========================
// RUST STRUCTS PRACTICE ANSWERS
// ===========================

// Q1
struct Person {
    name: String,
    age: u32,
}

// Q2
let alice = Person {
    name: String::from("Alice"),
    age: 30,
};

// Q3
impl Person {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

// Q4
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Q5
struct Color(u8, u8, u8);

// Q6
struct Marker;

// Q7
pub struct Student {
    pub id: u32,
    grade: char,
}

impl Student {
    pub fn new(id: u32, grade: char) -> Self {
        Student { id, grade }
    }
    pub fn get_grade(&self) -> char {
        self.grade
    }
}

// Q8
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }

    // Q9
    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// Q10
struct Login {
    username: String,
    email: String,
}

impl Login {
    fn display(&self) {
        println!("Username: {} | Email: {}", self.username, self.email);
    }
}

// Q11
#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}

// Q12
struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

// Q13
let bob = Person {
    name: String::from("Bob"),
    ..alice
};

// Q14
struct Car {
    mileage: u32,
}

impl Car {
    fn drive(&mut self) {
        self.mileage += 10;
    }
}

// Q15
enum Role {
    Admin,
    User,
    Guest,
}

struct User {
    username: String,
    role: Role,
}

let admin = User {
    username: String::from("Newton"),
    role: Role::Admin,
};

fn main() {
    println!("Struct answers loaded successfully!");
}
