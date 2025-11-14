impl Dad {
    fn new<T: Into<String>>(age: i32, name: T, alive: bool) -> Self {
        Self {
            age,
            name: name.into(),
            alive,
        }
    }
}

Dad::new(55, "Kamau", true);
Dad::new(60, String::from("Peter"), false);
Dad::new(40, format!("Mr {}", "Njoroge"), true);

