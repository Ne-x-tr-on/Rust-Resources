struct Dad<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> Dad<'a> {
    fn new(name: &'a str, age: i32) -> Self {
        Self { name, age }
    }
}

let name = String::from("Kamau");
let dad = Dad::new(&name, 55);

