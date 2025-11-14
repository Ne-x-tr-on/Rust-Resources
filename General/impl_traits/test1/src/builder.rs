struct DadBuilder {
    age: i32,
    name: String,
    alive: bool,
}

impl DadBuilder {
    fn new() -> Self {
        Self {
            age: 0,
            name: "".to_string(),
            alive: true,
        }
    }

    fn age(mut self, age: i32) -> Self {
        self.age = age;
        self
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    fn alive(mut self, alive: bool) -> Self {
        self.alive = alive;
        self
    }

    fn build(self) -> Dad {
        Dad {
            age: self.age,
            name: self.name,
            alive: self.alive,
        }
    }
}


let dad = DadBuilder::new()
    .name("Kamau")
    .age(55)
    .alive(false)
    .build();
